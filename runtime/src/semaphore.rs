//! Semaphore

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use crate::memory::MemoryAllocation;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

/// Synchronization primitive that can be used to insert a dependency between queue operations or
/// between a queue operation and the host.
#[derive(Debug)]
pub struct Semaphore {
    pub(crate) handle: VkNonDispatchableHandle,
    flags: VkSemaphoreCreateFlags,
}

impl Semaphore {
    pub fn create(create_info: &VkSemaphoreCreateInfo) -> VkNonDispatchableHandle {
        info!("new Semaphore");
        let handle = VK_NULL_HANDLE;
        let flags = create_info.flags;

        let semaphore = Self { handle, flags };
        semaphore.register_object()
    }
}
