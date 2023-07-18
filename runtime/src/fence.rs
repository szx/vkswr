//! Fence

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use crate::memory::{MemoryAllocation, MemoryBinding};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

/// Synchronization primitive that can be used to insert a dependency from a queue to the host.
#[derive(Debug)]
pub struct Fence {
    pub(crate) handle: VkNonDispatchableHandle,
    pub(crate) logical_device: Arc<Mutex<LogicalDevice>>,
    flags: VkFenceCreateFlags,
    signaled: bool,
}

impl Fence {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        create_info: &VkFenceCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new Fence");
        let handle = VK_NULL_HANDLE;
        let flags = create_info.flags;
        let signaled = (Into::<VkFenceCreateFlagBits>::into(flags)
            & VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT)
            != 0;
        let fence = Self {
            handle,
            logical_device,
            flags,
            signaled,
        };
        fence.register_object()
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
