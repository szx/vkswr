mod common;

#[test]
fn api_exists() -> common::TestResult {
    let cdylib_path = common::get_cdylib_path();

    // TODO: Implement 'vkGetInstanceProcAddr' or 'vk_icdGetInstanceProcAddr'.
    // TODO: Export VK_ICD_FILENAMES and run vulkaninfo & vkcube.
    // TODO: Generate functions from VK.xml. Create stubs.rs.

    unsafe {
        let cdylib = libloading::Library::new(&cdylib_path)?;
        let lib_test: libloading::Symbol<unsafe extern "C" fn() -> u32> =
            cdylib.get(b"lib_test")?;
        assert_eq!(lib_test(), 1);
    }
    Ok(())
}
