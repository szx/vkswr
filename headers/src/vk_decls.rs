#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]

pub use std::ptr::NonNull;
use std::sync::Arc;
use log::*;

pub(crate) type VkDispatchableHandle = Option<NonNull<std::ffi::c_void>>;

pub unsafe fn set_dispatchable_handle<T>(
    handle: NonNull<VkDispatchableHandle>,
    value: Arc<T>,
) {
    trace!(
        "set_dispatchable_handle {} arc: {} {}",
        std::any::type_name::<T>(),
        Arc::strong_count(&value),
        Arc::weak_count(&value)
    );
    let value = Arc::into_raw(value);
    Arc::decrement_strong_count(value);
    *handle.as_ptr() = std::mem::transmute(value);
}

pub unsafe fn get_dispatchable_handle<T>(
    handle: Option<NonNull<std::ffi::c_void>>,
) -> Option<Arc<T>> {
    handle.map_or_else(
        || None,
        |handle| {
            let ptr = std::mem::transmute::<_, *const T>(handle);
            Arc::increment_strong_count(ptr);
            let arc = Arc::from_raw(ptr);
            trace!(
                "get_dispatchable_handle {} arc: {} {}",
                std::any::type_name::<T>(),
                Arc::strong_count(&arc),
                Arc::weak_count(&arc)
            );
            Some(arc)
        },
    )
}

pub trait RegisterDispatchable {
    fn register(self: Arc<Self>) -> Arc<Self>;
    fn unregister(self: &Arc<Self>);
}

pub fn drop_dispatchable_handle(handle: Arc<impl RegisterDispatchable>) {
    handle.unregister();
    assert_eq!(Arc::strong_count(&handle), 1);
    drop(handle);
}


pub(crate) type VkNonDispatchableHandle = u64;

// TODO: Smarter handling of unsupported FFI types.
pub(crate) type VkUnsupportedType = *const std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/codegen_vk_decls.rs"));
