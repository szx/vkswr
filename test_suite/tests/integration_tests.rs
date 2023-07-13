use std::process::Command;

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
    current_dir: Option<&str>,
    args: impl IntoIterator<Item = &'static str>,
) -> common::TestResult {
    let icd_json_path = common::get_icd_json_path();
    let mut out = Command::new(executable_path);
    let mut out = out
        .env("VK_ICD_FILENAMES", icd_json_path)
        .env("VK_LOADER_DEBUG", "error,warn,debug,driver") // error,warn,info,debug,layer,driver
        //.env("ICD_WAIT_FOR_DEBUGGER", "true")
        .env("RUST_LOG", "trace");
    let mut out = if let Some(current_dir) = current_dir {
        out.current_dir(std::fs::canonicalize(current_dir)?)
    } else {
        out
    };
    let mut out = out.args(args);
    let out = out.output()?;
    if !out.status.success() {
        eprintln!("stdout: {}", String::from_utf8_lossy(&out.stdout),);
        eprintln!("stderr: {}", String::from_utf8_lossy(&out.stderr),);
        assert!(false);
    }
    Ok(())
}

#[test]
fn run_vulkaninfo() -> common::TestResult {
    run_executable("vulkaninfo", None, [])
}

#[test]
fn run_vkcube() -> common::TestResult {
    run_executable("vkcube", None, [])
}

fn run_deqp_vk(case_name: &'static str) -> common::TestResult {
    run_executable(
        "./deqp-vk",
        std::env::var("VULKAN_CTS_PATH").ok().as_deref(),
        [
            "--deqp-log-images=disable",
            "--deqp-log-shader-sources=disable",
            "-n",
            case_name.into(),
        ],
    )
}

#[test]
fn run_deqp_vk_api_info() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.info.*")
}

#[test]
fn run_deqp_vk_api_version_check_entry_points() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.version_check.entry_points")
}

#[test]
fn run_deqp_vk_api_all() -> common::TestResult {
    run_deqp_vk("dEQP-VK.api.*")
}
