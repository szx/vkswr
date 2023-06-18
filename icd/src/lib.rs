#![allow(non_snake_case)]

mod codegen;
mod impls;

use crate::codegen::*;
use std::ffi::{c_char, c_uint, c_void, CStr};
use std::ptr::{null, null_mut};

fn wait_for_debugger() {
    static mut debug: bool = true;
    unsafe {
        if debug && std::env::var("ICD_WAIT_FOR_DEBUGGER").is_err() {
            debug = false;
        };
        while debug {}
        debug = false;
    }
}

#[no_mangle]
pub extern "C" fn vk_icdGetInstanceProcAddr(
    instance: VkInstance,
    pName: *const c_char,
) -> PFN_vkVoidFunction {
    let Ok(pName) = unsafe { CStr::from_ptr(pName) }.to_str() else { return None; };
    println!("vk_icdGetInstanceProcAddr: {:?} {:?}", instance, pName);
    wait_for_debugger();
    match pName {
        "vkCreateInstance" => unsafe {
            println!("HIRO");
            std::mem::transmute(vkCreateInstance as *const ())
        },
        "vkEnumerateInstanceExtensionProperties" => unsafe {
            println!("HIRO2");
            std::mem::transmute(vkEnumerateInstanceExtensionProperties as *const ())
        },
        &_ => None,
    }
}

// TODO: Implement Core 1.0 functions required by loader_icd_init_entries().

#[no_mangle]
pub extern "C" fn lib_test() -> u32 {
    println!("Hello from the library!");
    1
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn works_codegen() {}
}
