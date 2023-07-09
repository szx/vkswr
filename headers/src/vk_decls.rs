#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use std::fmt::{Debug, Formatter};
use std::hash::Hasher;
use std::num::NonZeroU64;
pub use std::ptr::NonNull;
use xcb;

/// ICD has to return pointer to struct with the first field being VkLoaderData.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct VkDispatchableHandle(pub Option<NonNull<VkDispatchableHandleInner>>);

impl std::hash::Hash for VkDispatchableHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            state.write_u64(self.0.unwrap().as_ref().key);
        }
        state.finish();
    }
}

unsafe impl Send for VkDispatchableHandle {}
unsafe impl Sync for VkDispatchableHandle {}

#[repr(C)]
pub struct VkDispatchableHandleInner {
    pub loader_data: VkLoaderData,
    pub key: u64,
}

impl Debug for VkDispatchableHandleInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VkDispatchableHandleInner")
            .field("key", &self.key)
            .finish()
    }
}

#[repr(C)]
pub union VkLoaderData {
    pub loader_magic: usize,
    pub loader_data: Option<NonNull<std::ffi::c_void>>,
}

impl VkLoaderData {
    pub const LOADER_MAGIC: usize = 0x01CDC0DE;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct VkNonDispatchableHandle(pub Option<NonZeroU64>);
pub const VK_NULL_HANDLE: VkNonDispatchableHandle = VkNonDispatchableHandle(None);

pub type xcb_connection_t = xcb::ffi::xcb_connection_t;
pub type xcb_window_t = u32;

// TODO: Smarter handling of unsupported FFI types.
pub(crate) type VkUnsupportedType = *const std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/codegen_vk_decls.rs"));

#[macro_export]
macro_rules! c_char_array {
    ($const_name:ident, $len_name:ident, $str:literal) => {
        lazy_static! {
            static ref $const_name: [c_char; $len_name as usize] = {
                let mut s: [u8; $len_name as usize] = [0; $len_name as usize];
                let str = $str;
                s[..str.len()].copy_from_slice(str.as_bytes());
                unsafe { std::mem::transmute(s) }
            };
        }
    };
}
