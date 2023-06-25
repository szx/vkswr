#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use std::ops::Deref;

pub(crate) type VkDispatchableHandle = std::ptr::NonNull<std::ffi::c_void>;

pub unsafe fn set_dispatchable_handle<T>(
    handle: std::ptr::NonNull<VkDispatchableHandle>,
    value: &T,
) {
    *handle.as_ptr() = std::mem::transmute(value);
}

pub unsafe fn get_dispatchable_handle_ref<'a, T>(
    handle: VkDispatchableHandle,
) -> &'a T
{
    std::mem::transmute::<_, std::ptr::NonNull<T>>(handle.as_ptr()).as_ref()
}

pub(crate) type VkNonDispatchableHandle = u64;

// TODO: Smarter handling of unsupported FFI types.
pub(crate) type VkUnsupportedType = *const std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/codegen_vk_decls.rs"));
