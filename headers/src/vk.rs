#![allow(non_snake_case)]

pub(crate) type VkDispatchableHandle = *const std::ffi::c_void;
pub(crate) type VkNonDispatchableHandle = u64;

include!(concat!(env!("OUT_DIR"), "/codegen_vk_decls.rs"));
