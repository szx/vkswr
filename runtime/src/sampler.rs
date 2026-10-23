//! Sampler object.

use crate::context::VulkanObject;
use crate::logical_device::LogicalDevice;
use headers::vk_decls::*;
use log::info;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sampler {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
    logical_device: Arc<Mutex<LogicalDevice>>,
}

impl Sampler {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        _flags: VkSamplerCreateFlags,
    ) -> VkNonDispatchableHandle {
        info!("new Sampler");
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
        }
        .register()
    }
}
