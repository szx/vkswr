#[no_mangle]
pub extern fn lib_test() -> u32 {
    println!("Hello from the library!");
    1
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn works() {

    }
}