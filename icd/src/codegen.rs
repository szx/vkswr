#![allow(non_snake_case)]

use crate::impls::*;

pub(crate) type VkDispatchableHandle = *const std::ffi::c_void;
pub(crate) type VkNonDispatchableHandle = u64;
// TODO: Use X-macro to embed wait_for_debugger() in functions?
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
