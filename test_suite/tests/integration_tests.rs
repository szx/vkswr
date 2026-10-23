use base64::Engine;
use lazy_static::lazy_static;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

mod common;

#[test]
fn check_exported_symbols() -> common::TestResult {
    let cdylib_path = common::get_cdylib_path();

    unsafe {
        let cdylib = libloading::Library::new(cdylib_path)?;
        {
            type FnPtr = unsafe extern "C" fn(
                *const std::ffi::c_void,
                *const std::ffi::c_char,
            ) -> *const std::ffi::c_void;
            assert!(cdylib
                .get::<libloading::Symbol<FnPtr>>(b"vk_icdGetInstanceProcAddr")
                .is_ok());
        }
        {
            type FnPtr = unsafe extern "C" fn(
                *const std::ffi::c_void,
                *const std::ffi::c_void,
                *const std::ffi::c_void,
            ) -> *const std::ffi::c_void;
            assert!(cdylib
                .get::<libloading::Symbol<FnPtr>>(b"vkCreateInstance")
                .is_err());
        }
    }

    Ok(())
}

fn run_executable(
    executable_path: impl AsRef<Path>,
    current_dir: Option<&Path>,
    args: &[OsString],
    callback: impl Fn(),
) -> common::TestResult {
    let icd_json_path = common::get_icd_json_path();
    let mut command = Command::new(executable_path.as_ref());
    command
        .env("VK_ICD_FILENAMES", icd_json_path)
        .env("VK_LOADER_DEBUG", "error,warn,debug") // error,warn,info,debug,layer,driver
        //.env("ICD_WAIT_FOR_DEBUGGER", "true")
        .env("RUST_LOG", "trace");
    if let Some(current_dir) = current_dir {
        command.current_dir(current_dir);
    }
    let out = command.args(args).output()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    println!("stdout:\n{}", stdout);
    callback();
    assert!(
        out.status.success(),
        "Didn't pass: stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !stdout.contains("Passed:        0"),
        "Zero passed, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    Ok(())
}

#[test]
fn run_vulkaninfo() -> common::TestResult {
    let vulkaninfo = test_tools::sdk_bin("vulkaninfo")
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    run_executable(vulkaninfo, None, &[], || {})
}

#[test]
fn run_vkcube() -> common::TestResult {
    let vkcube =
        test_tools::sdk_bin("vkcube").map_err(|error| std::io::Error::other(error.to_string()))?;
    let args = [OsString::from("--c"), OsString::from("600")];
    run_executable(vkcube, None, &args, || {})
}

static IMAGE_OUTPUT_DIR: OnceLock<PathBuf> = OnceLock::new();

fn image_output_dir() -> &'static Path {
    IMAGE_OUTPUT_DIR.get_or_init(|| {
        // Don't wipe the directory: it also contains the reference images used
        // by the README. New results overwrite files with the same name.
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/deqp-vk");
        std::fs::create_dir_all(&path).unwrap();
        path
    })
}

fn run_deqp_vk(case_name: &'static str, log_images: bool) -> common::TestResult {
    let deqp_vk =
        test_tools::deqp_vk().map_err(|error| std::io::Error::other(error.to_string()))?;
    let run_dir = assert_fs::TempDir::new()?;
    let results_path = run_dir.join("TestResults.qpa");
    let args = [
        OsString::from(if log_images {
            "--deqp-log-images=enable"
        } else {
            "--deqp-log-images=disable"
        }),
        OsString::from("--deqp-log-shader-sources=disable"),
        OsString::from("--deqp-terminate-on-fail=enable"),
        OsString::from("--deqp-log-filename"),
        results_path.clone().into_os_string(),
        OsString::from("--deqp-shadercache=disable"),
        OsString::from("-n"),
        OsString::from(case_name),
    ];
    let print_deqp_vk_image = || {
        if log_images {
            print_deqp_vk_image(case_name, &results_path, image_output_dir());
        }
    };
    run_executable(deqp_vk, Some(run_dir.path()), &args, print_deqp_vk_image)
}

#[ignore]
#[test]
fn run_deqp_vk_api_all() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.*", false)
}

fn print_deqp_vk_image(case_name: &str, results_path: &Path, image_output_dir: &Path) {
    let Ok(test_results_qpa) = std::fs::read_to_string(results_path) else {
        return;
    };

    lazy_static! {
        static ref RE_IMAGE: regex::Regex =
            regex::Regex::new(r#"(?s)<Image\sName="(?<name>.*?)"\sWidth="(?<width>.*?)"\sHeight="(?<height>.*?)"\sFormat="(?<format>.*?)"\sCompressionMode="(?<compression_mode>.*?)"\sDescription="(?<description>.*?)">(?<base64>.*?)<\/Image>"#).expect("regex");
    }
    for cap in RE_IMAGE.captures_iter(&test_results_qpa) {
        let image = cap.name("base64").unwrap().as_str();
        let image = image
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c as u8)
            .collect::<Vec<_>>();
        let image = base64::engine::general_purpose::STANDARD
            .decode(&image)
            .unwrap();

        let mut path = PathBuf::from(image_output_dir);
        let filename = format!(
            "{}_{}.{}",
            case_name,
            cap.name("name").unwrap().as_str(),
            cap.name("compression_mode").unwrap().as_str()
        );
        path.push(filename);
        std::fs::write(&path, image).unwrap();

        viuer::print_from_file(
            path,
            &viuer::Config {
                transparent: false,
                ..Default::default()
            },
        )
        .unwrap();
    }
}

#[test]
fn run_deqp_vk_api_info() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.info.*", false)
}

#[test]
fn run_deqp_vk_api_version_check_entry_points() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.version_check.entry_points", false)
}

#[test]
fn run_deqp_vk_api_device_init_create_device_basic() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.device_init.create_device.basic", false)
}

#[ignore] // Pass, long execution.
#[test]
fn run_deqp_vk_api_device_init_create_instance_device_intentional_alloc_fail_basic(
) -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.api.device_init.create_instance_device_intentional_alloc_fail.basic",
        false,
    )
}

#[test]
fn run_deqp_vk_api_device_init_create_device_unsupported_features_core() -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.api.device_init.create_device_unsupported_features.core",
        false,
    )
}

#[test]
fn run_deqp_vk_api_object_management_single_buffer_view_uniform_r8g8b8a8_unorm(
) -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.api.object_management.single.buffer_view_uniform_r8g8b8a8_unorm",
        false,
    )
}

#[test]
fn run_deqp_vk_api_smoke_create_shader() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.smoke.create_shader", false)
}

#[test]
fn run_deqp_vk_api_smoke_triangle() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.smoke.triangle", true)
}

#[ignore]
#[test]
fn run_deqp_memory_all() -> common::TestResult {
    run_deqp_vk("dEQP-VK.memory.*", false)
}

#[test]
fn run_deqp_memory_requirements_all() -> common::TestResult {
    run_deqp_vk("dEQP-VK.memory.requirements.*", false)
}

#[test]
fn run_deqp_vk_memory_requirements_core_buffer_regular() -> common::TestResult {
    run_deqp_vk("dEQP-VK.memory.requirements.core.buffer.regular", false)
}

#[test]
fn run_deqp_vk_memory_requirements_core_image_regular_tiling_linear() -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.requirements.core.image.regular_tiling_linear",
        false,
    )
}

#[test]
fn run_deqp_vk_memory_mapping_suballocation_random_0() -> common::TestResult {
    run_deqp_vk("dEQP-VK.memory.mapping.suballocation.random.0", false)
}

#[test]
fn run_deqp_vk_memory_pipeline_barrier_host_write_transfer_src_1024() -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.pipeline_barrier.graphics.host_write_transfer_src.1024",
        false,
    )
}

#[test]
fn run_deqp_vk_memory_mapping_suballocation_random_3() -> common::TestResult {
    run_deqp_vk("dEQP-VK.memory.mapping.suballocation.random.3", false)
}

#[test]
fn run_deqp_vk_memory_pipeline_barrier_host_write_vertex_buffer_1024_vertex_buffer_stride_2(
) -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.1024_vertex_buffer_stride_2",
        false,
    )
}

#[test]
fn run_deqp_vk_memory_pipeline_barrier_host_write_vertex_buffer_1024_vertex_buffer_stride_4(
) -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.1024_vertex_buffer_stride_4",
        false,
    )
}

#[test]
fn run_deqp_vk_memory_pipeline_barrier_host_write_vertex_buffer_8192_vertex_buffer_stride_4(
) -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.pipeline_barrier.graphics.host_write_vertex_buffer.8192_vertex_buffer_stride_2",
        false,
    )
}

#[test]
fn run_deqp_vk_memory_pipeline_barrier_host_write_index_buffer_1024() -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.pipeline_barrier.graphics.host_write_index_buffer.1024",
        false,
    )
}

#[test]
fn run_deqp_vk_memory_pipeline_barrier_host_write_uniform_buffer_1024() -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.pipeline_barrier.graphics.host_write_uniform_buffer.1024",
        true,
    )
}
