//! Device memory allocation.

use crate::context::VulkanObject;
use crate::logical_device::LogicalDevice;
use headers::vk_decls::*;
use log::info;
use parking_lot::Mutex;
use std::ptr::NonNull;
use std::sync::Arc;

#[derive(Debug)]
pub struct MemoryAllocation {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    pub(crate) gpu_memory_allocation: gpu::MemoryAllocation,
    state: MemoryAllocationState,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MemoryAllocationState {
    HostUnmapped,
    HostMapped,
}

impl MemoryAllocation {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        size: u64,
        _memory_type_index: u32,
    ) -> VkNonDispatchableHandle {
        info!("new DeviceMemory");
        let gpu_memory_allocation = logical_device
            .lock()
            .physical_device()
            .gpu
            .memory
            .allocate_memory(size);
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
            gpu_memory_allocation,
            state: MemoryAllocationState::HostUnmapped,
        }
        .register()
    }

    pub fn map_host(
        &mut self,
        offset: u64,
        size: u64,
    ) -> Result<NonNull<std::ffi::c_void>, VkResult> {
        if self.state == MemoryAllocationState::HostMapped {
            return Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED);
        }

        let allocation_size = self.gpu_memory_allocation.size;
        if offset >= allocation_size {
            return Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED);
        }
        let size = if size == VK_WHOLE_SIZE {
            allocation_size - offset
        } else if offset
            .checked_add(size)
            .is_some_and(|end| end <= allocation_size)
        {
            size
        } else {
            return Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED);
        };

        let pointer = self
            .logical_device
            .lock()
            .physical_device()
            .gpu
            .memory
            .map_host(self.gpu_memory_allocation, offset, size)
            .ok_or(VkResult::VK_ERROR_MEMORY_MAP_FAILED)?;
        self.state = MemoryAllocationState::HostMapped;
        Ok(pointer)
    }

    pub fn unmap_host(&mut self) {
        if self.state == MemoryAllocationState::HostMapped {
            self.logical_device
                .lock()
                .physical_device()
                .gpu
                .memory
                .unmap_host(self.gpu_memory_allocation);
            self.state = MemoryAllocationState::HostUnmapped;
        }
    }
}

impl Drop for MemoryAllocation {
    fn drop(&mut self) {
        self.unmap_host();
        self.logical_device
            .lock()
            .physical_device()
            .gpu
            .memory
            .free_memory(self.gpu_memory_allocation);
    }
}
