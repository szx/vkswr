//! Image

use crate::{Context, Dispatchable, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::{Mutex, RwLockWriteGuard};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct CommandPool {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    flags: VkCommandPoolCreateFlags,
    queue_family_index: u32,
}

impl CommandPool {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        create_info: &VkCommandPoolCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new CommandPool");
        let handle = VK_NULL_HANDLE;
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

#[derive(Debug)]
pub struct CommandBuffer {
    handle: VkDispatchableHandle,
    level: VkCommandBufferLevel,
    command_pool: Arc<Mutex<CommandPool>>,
}

impl CommandBuffer {
    pub fn create(allocate_info: &VkCommandBufferAllocateInfo) -> VkDispatchableHandle {
        info!("new CommandBuffer");
        let handle = VkDispatchableHandle(None);
        let level = allocate_info.level;
        let Some(command_pool) = CommandPool::from_handle(allocate_info.commandPool) else {
            unreachable!()
        };

        let object = Self {
            handle,
            level,
            command_pool,
        };
        object.register_object()
    }

    pub fn begin(&mut self) {
        trace!("CommandBuffer::begin");
        // TODO: Start recording command buffer.
    }

    pub fn cmd_pipeline_barrier(&mut self) {
        trace!("CommandBuffer::cmd_pipeline_barrier");
        // TODO: Record pipeline barrier.
    }
}

impl Dispatchable for CommandBuffer {
    fn get_hash(context: &Context) -> &HashMap<VkDispatchableHandle, Arc<Mutex<Self>>> {
        &context.command_buffers
    }

    fn get_hash_mut(context: &mut Context) -> &mut HashMap<VkDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.command_buffers
    }

    fn set_handle(&mut self, handle: VkDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkDispatchableHandle {
        self.handle
    }
}
