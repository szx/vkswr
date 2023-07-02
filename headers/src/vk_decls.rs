#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use log::*;
pub use std::ptr::NonNull;
use std::sync::Arc;

pub(crate) type VkDispatchableHandle = Option<NonNull<std::ffi::c_void>>;

pub trait DispatchableHandle<T = Self> {
    fn register_handle(self: Arc<Self>) -> Arc<Self>;

    fn unregister_handle(self: &Arc<Self>);

    unsafe fn set_handle(handle: NonNull<VkDispatchableHandle>, value: Arc<T>) {
        trace!(
            "{}::set_handle arc: {} {}",
            std::any::type_name::<T>(),
            Arc::strong_count(&value),
            Arc::weak_count(&value)
        );
        let value = Arc::into_raw(value);
        Arc::decrement_strong_count(value);
        *handle.as_ptr() = std::mem::transmute(value);
    }

    unsafe fn get_handle(handle: VkDispatchableHandle) -> Option<Arc<T>> {
        handle.map_or_else(
            || None,
            |handle| {
                let ptr = std::mem::transmute::<_, *const T>(handle);
                Arc::increment_strong_count(ptr);
                let arc = Arc::from_raw(ptr);
                trace!(
                    "{}::get_handle arc: {} {}",
                    std::any::type_name::<T>(),
                    Arc::strong_count(&arc),
                    Arc::weak_count(&arc)
                );
                Some(arc)
            },
        )
    }
}

pub fn drop_dispatchable_handle(handle: Arc<impl DispatchableHandle>) {
    handle.unregister_handle();
    assert_eq!(Arc::strong_count(&handle), 1);
    drop(handle);
}

pub(crate) type VkNonDispatchableHandle = u64;

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
