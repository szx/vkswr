//! Binary semaphore synchronization primitive.

use crate::context::VulkanObject;
use headers::vk_decls::*;
use log::info;

/// Synchronization primitive that orders queue operations, or a queue
/// operation and presentation.
#[derive(Debug)]
pub struct Semaphore {
    pub(crate) handle: VkNonDispatchableHandle,
}

impl Semaphore {
    pub fn create(_create_info: &VkSemaphoreCreateInfo) -> VkNonDispatchableHandle {
        info!("new Semaphore");
        Self {
            handle: VK_NULL_HANDLE,
        }
        .register()
    }
}
