//! Buffer

use crate::memory::{DeviceMemory, MemoryBinding};
use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Buffer {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    size: VkDeviceSize,
    binding: Option<MemoryBinding>,
}

impl Buffer {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        size: VkDeviceSize,
        flags: VkBufferUsageFlags,
        usage: VkBufferCreateFlags,
    ) -> VkNonDispatchableHandle {
        info!("new Buffer");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let _ = usage;

        let object = Self {
            handle,
            logical_device,
            size,
            binding: None,
        };
        object.register_object()
    }

    pub const fn memory_requirements(&self) -> VkMemoryRequirements {
        VkMemoryRequirements {
            size: self.size,
            alignment: 1,
            memoryTypeBits: 0, // TODO: Acquire MemoryType from PhysicalDevice..
        }
    }

    pub fn bind_memory(&mut self, memory: Arc<Mutex<DeviceMemory>>, offset: u64) -> VkResult {
        self.binding = Some(MemoryBinding(memory, offset));
        VkResult::VK_SUCCESS
    }
}

impl NonDispatchable for Buffer {
    fn get_hash<'a>(
        context: &'a Context,
    ) -> &'a HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.buffers
    }

    fn get_hash_mut<'a>(
        context: &'a mut Context,
    ) -> &'a mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.buffers
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}
