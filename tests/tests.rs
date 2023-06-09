use std::path::PathBuf;

type TestResult = Result<(), Box<dyn std::error::Error>>;


#[test]
fn api_exists() -> TestResult {
    build_cdylib()?;
    let cdylib_path = get_cdylib_path();

    // TODO: Move to text fixture.
    unsafe {
        let cdylib = libloading::Library::new(&cdylib_path)?;
        let lib_test: libloading::Symbol<unsafe extern fn() -> u32> = cdylib.get(b"lib_test")?;
        assert_eq!(lib_test(), 1);
    }
    Ok(())
}

fn build_cdylib() -> TestResult  {
    escargot::CargoBuild::new()
        .manifest_path("Cargo.toml")
        .exec()?;
    Ok(())
}

fn get_cdylib_path() -> PathBuf {
    let mut cdylib_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cdylib_path.push("target/debug/librust_software_rasterizer.so");
    cdylib_path
}