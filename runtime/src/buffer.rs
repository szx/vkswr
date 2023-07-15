//! Buffer

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use crate::memory::{DeviceMemory, MemoryBinding};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Buffer {
    pub(crate) handle: VkNonDispatchableHandle,
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

    pub fn memory_requirements(&self) -> VkMemoryRequirements {
        VkMemoryRequirements {
            size: self.size,
            alignment: 1,
            memoryTypeBits: self
                .logical_device
                .lock()
                .physical_device()
                .memory_type_bits_for_buffer(),
        }
    }

    pub fn bind_memory(&mut self, memory: Arc<Mutex<DeviceMemory>>, offset: u64) -> VkResult {
        self.binding = Some(MemoryBinding(memory, offset));
        VkResult::VK_SUCCESS
    }
}
