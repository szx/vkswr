#[no_mangle]
pub extern fn lib_test() -> u32 {
    println!("Hello from the library!");
    1
}