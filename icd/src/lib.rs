//! Vulkan installable client driver (ICD) entry points.
//!
//! The ICD exposes C entry points to the Vulkan loader. Application input is
//! therefore trusted only after basic validation, and handles are resolved
//! through the runtime object registry.

#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![cfg_attr(wait_for_debugger, feature(core_intrinsics))]
#![cfg_attr(wait_for_debugger, allow(internal_features))]

mod buffer;
mod command_buffer;
mod descriptor;
mod image;
mod impls;
mod memory;
mod pipeline;
mod sampler;
mod surface;
mod swapchain;

use std::ffi::CStr;
use std::ptr::NonNull;

use headers::vk_decls::*;

/// Converts a typed entry point to the loader's opaque function pointer type.
macro_rules! entry_point {
    ($function:path) => {{
        // SAFETY: The loader invokes the pointer with the registry signature.
        unsafe { std::mem::transmute::<*const (), PFN_vkVoidFunction>($function as *const ()) }
    }};
}

/// Resolves a Vulkan command name against a table of ICD entry points.
macro_rules! dispatch {
    ($name:expr, { $($command:literal => $function:path),* $(,)? }) => {
        match $name {
            $($command => crate::entry_point!($function),)*
            _ => None,
        }
    };
}

pub(crate) use dispatch;
pub(crate) use entry_point;

#[cfg(wait_for_debugger)]
static WAIT_FOR_DEBUGGER: parking_lot::Mutex<bool> = parking_lot::Mutex::new(true);

#[cfg(wait_for_debugger)]
fn wait_for_debugger() {
    let mut debug = WAIT_FOR_DEBUGGER.lock();
    if !*debug {
        return;
    }
    env_logger::init();
    if std::env::var("ICD_WAIT_FOR_DEBUGGER").is_err() {
        *debug = false;
        return;
    }
    std::thread::sleep(std::time::Duration::from_secs(10));
    std::intrinsics::breakpoint();
    *debug = false;
}

#[cfg(not(wait_for_debugger))]
fn wait_for_debugger() {}

/// Returns the address of an ICD entry point.
///
/// # Safety
///
/// `pName` must be either null or point to a NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vk_icdGetInstanceProcAddr(
    _instance: VkInstance,
    pName: *const std::ffi::c_char,
) -> PFN_vkVoidFunction {
    if pName.is_null() {
        return None;
    }
    let Ok(pName) = (unsafe { CStr::from_ptr(pName) }).to_str() else {
        return None;
    };
    wait_for_debugger();
    impls::instance_command(pName)
}

/// Negotiates the ICD interface version understood by the loader.
///
/// # Safety
///
/// `pSupportedVersion` must be null or point to a writable `u32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vk_icdNegotiateLoaderICDInterfaceVersion(
    pSupportedVersion: Option<NonNull<std::ffi::c_uint>>,
) -> VkResult {
    let Some(pSupportedVersion) = pSupportedVersion else {
        return VkResult::VK_ERROR_INCOMPATIBLE_DRIVER;
    };
    const SUPPORTED_VERSION: u32 = 3;
    let demanded_version = unsafe { *pSupportedVersion.as_ptr() };
    if demanded_version < SUPPORTED_VERSION {
        VkResult::VK_ERROR_INCOMPATIBLE_DRIVER
    } else {
        unsafe {
            *pSupportedVersion.as_ptr() = demanded_version.min(SUPPORTED_VERSION);
        }
        VkResult::VK_SUCCESS
    }
}
