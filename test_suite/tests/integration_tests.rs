use base64::Engine;
use lazy_static::lazy_static;
use std::cell::OnceCell;
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
    executable_path: &str,
    current_dir: Option<impl AsRef<Path>>,
    args: impl IntoIterator<Item = &'static str>,
    callback: impl Fn(),
) -> common::TestResult {
    let icd_json_path = common::get_icd_json_path();
    let mut out = Command::new(executable_path);
    let out = out
        .env("VK_ICD_FILENAMES", icd_json_path)
        .env("VK_LOADER_DEBUG", "error,warn,debug") // error,warn,info,debug,layer,driver
        //.env("ICD_WAIT_FOR_DEBUGGER", "true")
        .env("RUST_LOG", "debug");
    let out = if let Some(current_dir) = current_dir {
        out.current_dir(current_dir)
    } else {
        out
    };
    let out = out.args(args);
    let out = out.output()?;
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
    run_executable("vulkaninfo", None::<&str>, [], || {})
}

#[test]
fn run_vkcube() -> common::TestResult {
    run_executable("vkcube", None::<&str>, ["--c", "600"], || {})
}

static IMAGE_OUTPUT_DIR: OnceLock<PathBuf> = OnceLock::new();

fn run_deqp_vk(case_name: &'static str, log_images: bool) -> common::TestResult {
    let print_deqp_vk_image = || {
        if log_images {
            print_deqp_vk_image(
                case_name,
                IMAGE_OUTPUT_DIR.get_or_init(|| {
                    let path = std::fs::canonicalize(std::path::PathBuf::from(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/../assets/deqp-vk"
                    )))
                    .unwrap();
                    std::fs::remove_dir_all(&path).unwrap();
                    std::fs::create_dir(&path).unwrap();
                    path
                }),
            );
        }
    };
    run_executable(
        "./deqp-vk",
        Some(deqp_vk_dir()),
        [
            if log_images {
                "--deqp-log-images=enable"
            } else {
                "--deqp-log-images=disable"
            },
            "--deqp-log-shader-sources=disable",
            "--deqp-terminate-on-fail=enable",
            "-n",
            case_name,
        ],
        print_deqp_vk_image,
    )
}

fn deqp_vk_dir() -> PathBuf {
    assert!(std::env::var("VULKAN_CTS_PATH").is_ok());
    std::fs::canonicalize(std::env::var("VULKAN_CTS_PATH").unwrap()).unwrap()
}

#[ignore]
#[test]
fn run_deqp_vk_api_all() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.*", false)
}

fn print_deqp_vk_image(case_name: &str, image_output_dir: impl AsRef<Path>) {
    let mut test_results_qpa_path = deqp_vk_dir();
    test_results_qpa_path.push("TestResults.qpa");
    let test_results_qpa = std::fs::read_to_string(test_results_qpa_path).unwrap();

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

        let mut path = PathBuf::from(image_output_dir.as_ref());
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
        "dEQP-VK.memory.pipeline_barrier.host_write_transfer_src.1024",
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
        "dEQP-VK.memory.pipeline_barrier.host_write_vertex_buffer.1024_vertex_buffer_stride_2",
        true,
    )
}

#[test]
fn run_deqp_vk_memory_pipeline_barrier_host_write_vertex_buffer_1024_vertex_buffer_stride_4(
) -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.pipeline_barrier.host_write_vertex_buffer.1024_vertex_buffer_stride_4",
        true,
    )
}

#[test]
fn run_deqp_vk_memory_pipeline_barrier_host_write_vertex_buffer_8192_vertex_buffer_stride_4(
) -> common::TestResult {
    run_deqp_vk(
        "dEQP-VK.memory.pipeline_barrier.host_write_vertex_buffer.8192_vertex_buffer_stride_2",
        true,
    )
}
