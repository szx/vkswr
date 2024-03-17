//! Buffer

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use crate::memory::MemoryAllocation;
use common::graphics::{DescriptorBuffer, MemoryBinding};
use gpu::MemoryHandleStore;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Buffer {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    size: VkDeviceSize,
    gpu_binding: MemoryBinding,
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
            gpu_binding: Default::default(),
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

    pub fn bind_memory(&mut self, memory: Arc<Mutex<MemoryAllocation>>, offset: u64) -> VkResult {
        self.gpu_binding.store(
            memory.lock().gpu_memory_allocation,
            offset,
            self.size.saturating_sub(offset),
        );
        VkResult::VK_SUCCESS
    }

    pub fn descriptor(&self) -> DescriptorBuffer {
        let binding = self.gpu_binding.clone();
        DescriptorBuffer { binding }
    }
}

#[derive(Debug)]
pub struct BufferView {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    buffer: Arc<Mutex<Buffer>>,
    format: VkFormat,
    size: VkDeviceSize,
    offset: VkDeviceSize,
}

impl BufferView {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        buffer: Arc<Mutex<Buffer>>,
        format: VkFormat,
        size: VkDeviceSize,
        offset: VkDeviceSize,
    ) -> VkNonDispatchableHandle {
        info!("new BufferView");
        let handle = VK_NULL_HANDLE;

        let object = Self {
            handle,
            logical_device,
            buffer,
            format,
            size,
            offset,
        };
        object.register_object()
    }
}
