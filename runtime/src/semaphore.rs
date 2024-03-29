//! Semaphore

use crate::context::NonDispatchable;


use headers::vk_decls::*;
use log::*;

use std::fmt::Debug;


/// Synchronization primitive that can be used to insert a dependency between queue operations or
/// between a queue operation and the host.
#[derive(Debug)]
pub struct Semaphore {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
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
