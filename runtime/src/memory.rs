//! Device memory allocation

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use gpu::MemoryAddress;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct MemoryAllocation {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    pub(crate) gpu_memory_allocation: gpu::MemoryAllocation,
    state: MemoryAllocationState,
}

#[derive(Debug)]
enum MemoryAllocationState {
    HostUnmapped,
    HostMapped,
}

impl MemoryAllocation {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        size: u64,
        memory_type_index: u32,
    ) -> VkNonDispatchableHandle {
        info!("new DeviceMemory");
        let handle = VK_NULL_HANDLE;
        let _ = memory_type_index; // TODO: Acquire MemoryType from PhysicalDevice.

        let object = Self {
            handle,
            logical_device: logical_device.clone(),
            gpu_memory_allocation: logical_device
                .lock()
                .physical_device()
                .gpu
                .allocate_memory(size),
            state: MemoryAllocationState::HostUnmapped,
        };
        object.register_object()
    }

    pub fn map_host(
        &mut self,
        offset: u64,
        size: u64,
    ) -> Result<NonNull<std::ffi::c_void>, VkResult> {
        match self.state {
            MemoryAllocationState::HostMapped => Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED),
            MemoryAllocationState::HostUnmapped => unsafe {
                self.state = MemoryAllocationState::HostMapped;
                if offset >= self.gpu_memory_allocation.size {
                    return Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED);
                }
                let size = if size == VK_WHOLE_SIZE {
                    self.gpu_memory_allocation.size
                } else if offset + size <= self.gpu_memory_allocation.size {
                    size
                } else {
                    return Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED);
                };
                let ptr = self.logical_device.lock().physical_device().gpu.map_host(
                    self.gpu_memory_allocation,
                    offset,
                    size,
                );
                Ok(ptr)
            },
        }
    }

    pub fn unmap_host(&mut self) {
        match self.state {
            MemoryAllocationState::HostMapped => {
                self.state = MemoryAllocationState::HostUnmapped;
                self.logical_device
                    .lock()
                    .physical_device()
                    .gpu
                    .unmap_host(self.gpu_memory_allocation);
            }
            MemoryAllocationState::HostUnmapped => {
                self.state = MemoryAllocationState::HostUnmapped;
            }
        }
    }
}

impl Drop for MemoryAllocation {
    fn drop(&mut self) {
        self.logical_device
            .lock()
            .physical_device()
            .gpu
            .free_memory(self.gpu_memory_allocation);
    }
}

#[derive(Debug)]
pub struct MemoryBinding {
    pub memory: Arc<Mutex<MemoryAllocation>>,
    pub offset: u64,
    pub size: u64,
}
