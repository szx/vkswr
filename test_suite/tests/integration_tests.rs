use std::fs;
use std::process::Command;

mod common;

#[test]
fn load_cdylib() -> common::TestResult {

    let cdylib_path = common::get_cdylib_path();

    unsafe {
        let cdylib = libloading::Library::new(cdylib_path)?;
        let lib_test: libloading::Symbol<unsafe extern "C" fn() -> u32> =
            cdylib.get(b"lib_test")?;
        assert_eq!(lib_test(), 1);
    }

    Ok(())
}

#[test]
fn run_vulkaninfo() -> common::TestResult {
    let icd_json_path = common::get_icd_json_path();
    let out = Command::new("vulkaninfo")
        .env("VK_ICD_FILENAMES", icd_json_path)
        .env("VK_LOADER_DEBUG", "error,warn,debug,driver") // error,warn,info,debug,layer,driver
        //.env("ICD_WAIT_FOR_DEBUGGER", "true")
        .output()?;
    assert!(
        out.status.success(),
        "stdout: {},\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    Ok(())
}
