#![allow(non_snake_case)]

mod impls;

use headers::vk_decls::*;
use impls::*;

use std::ffi::{c_char, CStr};

fn wait_for_debugger() {
    static mut DEBUG: bool = true;
    unsafe {
        if DEBUG && std::env::var("ICD_WAIT_FOR_DEBUGGER").is_err() {
            DEBUG = false;
        };
        while DEBUG {}
        DEBUG = false;
    }
}

/* Exported functions */

/// # Safety
///
/// Use of null `pName` in an undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn vk_icdGetInstanceProcAddr(
    instance: VkInstance,
    pName: *const c_char,
) -> PFN_vkVoidFunction {
    let Ok(pName) = CStr::from_ptr(pName).to_str() else { return None; };
    println!("vk_icdGetInstanceProcAddr: {:?} {:?}", instance, pName);
    wait_for_debugger();
    match pName {
        "vkCreateInstance" => unsafe { std::mem::transmute(vkCreateInstance as *const ()) },
        "vkEnumerateInstanceExtensionProperties" => unsafe {
            std::mem::transmute(vkEnumerateInstanceExtensionProperties as *const ())
        },
        /* Vulkan Core 1.0 functions required by loader_icd_init_entries(). */
        "vkDestroyInstance" => unsafe { std::mem::transmute(vkDestroyInstance as *const ()) },
        "vkEnumeratePhysicalDevices" => unsafe {
            std::mem::transmute(vkEnumeratePhysicalDevices as *const ())
        },
        "vkGetPhysicalDeviceFeatures" => unsafe {
            std::mem::transmute(vkGetPhysicalDeviceFeatures as *const ())
        },
        "vkGetPhysicalDeviceFormatProperties" => unsafe {
            std::mem::transmute(vkGetPhysicalDeviceFormatProperties as *const ())
        },
        "vkGetPhysicalDeviceImageFormatProperties" => unsafe {
            std::mem::transmute(vkGetPhysicalDeviceImageFormatProperties as *const ())
        },
        "vkGetPhysicalDeviceProperties" => unsafe {
            std::mem::transmute(vkGetPhysicalDeviceProperties as *const ())
        },
        "vkGetPhysicalDeviceQueueFamilyProperties" => unsafe {
            std::mem::transmute(vkGetPhysicalDeviceQueueFamilyProperties as *const ())
        },
        "vkGetPhysicalDeviceMemoryProperties" => unsafe {
            std::mem::transmute(vkGetPhysicalDeviceMemoryProperties as *const ())
        },
        "vkGetDeviceProcAddr" => unsafe { std::mem::transmute(vkGetDeviceProcAddr as *const ()) },
        "vkCreateDevice" => unsafe { std::mem::transmute(vkCreateDevice as *const ()) },
        "vkEnumerateDeviceExtensionProperties" => unsafe {
            std::mem::transmute(vkEnumerateDeviceExtensionProperties as *const ())
        },
        "vkGetPhysicalDeviceSparseImageFormatProperties" => unsafe {
            std::mem::transmute(vkGetPhysicalDeviceSparseImageFormatProperties as *const ())
        },
        &_ => None,
    }
}

#[no_mangle]
pub extern "C" fn lib_test() -> u32 {
    println!("Hello from the library!");
    1
}
