use std::ffi::{c_char, c_uint, c_void, CStr};
use std::ptr::{null, null_mut};
use std::str::Utf8Error;

type PFN_vkVoidFunction = Option<unsafe extern "C" fn()>;
type VkDispatchableHandle = *const c_void;
type VkInstance = VkDispatchableHandle;

type VkInstanceCreateInfo = VkDispatchableHandle; // TODO: Codegen struct.
type VkAllocationCallbacks = VkDispatchableHandle; // TODO: Codegen struct.
type VkExtensionProperties = VkDispatchableHandle; // TODO: Codegen struct.


include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

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

#[no_mangle]
unsafe extern "C" fn vkCreateInstance(
    pCreateInfo: *const VkInstanceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pInstance: *mut VkInstance,
) -> VkResult {
    println!("Hello from vkCreateInstance()!");
    *pInstance = null_mut();
    // TODO: Create and register internal VkInstance.
    VkResult::VK_SUCCESS
}

#[no_mangle]
unsafe extern "C" fn vkEnumerateInstanceExtensionProperties(
    pLayerName: *const c_char,
    pPropertyCount: *mut c_uint,
    pProperties: *mut VkExtensionProperties,
) -> VkResult {
    println!("Hello from vkEnumerateInstanceExtensionProperties()!");
    assert_eq!(pLayerName, null());
    println!("*pPropertyCount = {}", *pPropertyCount);
    println!("*pProperties = {:?}", pProperties);
    if pProperties == null_mut() {
        *pPropertyCount = 0;
    }
    VkResult::VK_SUCCESS
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
    fn works_codegen() {
        
    }
}
