//! Sampler

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;

use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sampler {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    flags: VkSamplerCreateFlags,
}

impl Sampler {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkSamplerCreateFlags,
    ) -> VkNonDispatchableHandle {
        info!("new Sampler");
        let handle = VK_NULL_HANDLE;

        let object = Self {
            handle,
            logical_device,
            flags,
        };
        object.register_object()
    }
}
