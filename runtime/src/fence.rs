//! Fence synchronization primitive.

use crate::context::VulkanObject;
use crate::logical_device::LogicalDevice;
use headers::vk_decls::*;
use log::{info, trace};
use parking_lot::Mutex;
use std::sync::Arc;

/// Synchronization primitive that inserts a dependency from a queue to the host.
#[derive(Debug)]
pub struct Fence {
    pub(crate) handle: VkNonDispatchableHandle,
    /// Keeps the owning device alive for at least as long as the fence.
    #[allow(dead_code)]
    pub(crate) logical_device: Arc<Mutex<LogicalDevice>>,
    signaled: bool,
}

impl Fence {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        create_info: &VkFenceCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new Fence");
        let signaled = VkFenceCreateFlagBits::from(create_info.flags)
            & VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT
            != 0;
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
            signaled,
        }
        .register()
    }

    pub fn signal(&mut self) {
        trace!("fence {} signal", self.signaled);
        self.signaled = true;
    }

    pub fn reset(&mut self) {
        trace!("fence {} reset", self.signaled);
        self.signaled = false;
    }
}
