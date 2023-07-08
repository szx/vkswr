//! Image

use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct CommandPool {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<LogicalDevice>,
    flags: VkCommandPoolCreateFlags,
    queue_family_index: u32,
}

impl CommandPool {
    pub fn create(
        logical_device: Arc<LogicalDevice>,
        create_info: &VkCommandPoolCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new CommandPool");
        let handle = VK_NULL_HANDLE;
        let logical_device = logical_device.clone();
        let flags = create_info.flags;
        let queue_family_index = create_info.queueFamilyIndex;

        let command_pool = Self {
            handle,
            logical_device,
            flags,
            queue_family_index,
        };
        command_pool.register_object()
    }
}

impl NonDispatchable for CommandPool {
    fn get_hash<'a>(
        context: &'a Context,
    ) -> &'a HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.command_pools
    }

    fn get_hash_mut<'a>(
        context: &'a mut Context,
    ) -> &'a mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.command_pools
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}
