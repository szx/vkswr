#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use std::num::NonZeroU64;
pub use std::ptr::NonNull;
use xcb;

pub type VkDispatchableHandle = Option<NonNull<std::ffi::c_void>>;

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
