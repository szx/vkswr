//! Device memory

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct DeviceMemory {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    data: Vec<u8>,
    state: DeviceMemoryState,
}

#[derive(Debug)]
enum DeviceMemoryState {
    HostUnmapped,
    HostMapped,
}

impl DeviceMemory {
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
            logical_device,
            data: vec![0; size as usize],
            state: DeviceMemoryState::HostUnmapped,
        };
        object.register_object()
    }

    pub fn map_host(
        &mut self,
        offset: usize,
        size: usize,
    ) -> Result<NonNull<std::ffi::c_void>, VkResult> {
        match self.state {
            DeviceMemoryState::HostMapped => Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED),
            DeviceMemoryState::HostUnmapped => unsafe {
                self.state = DeviceMemoryState::HostMapped;
                if offset >= self.data.len() {
                    return Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED);
                }
                let size = if size == VK_WHOLE_SIZE as usize {
                    self.data.len()
                } else if offset + size <= self.data.len() {
                    size
                } else {
                    return Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED);
                };
                let ptr = self.data[offset..size].as_mut_ptr();
                let ptr = NonNull::new_unchecked(ptr as *mut std::ffi::c_void);
                Ok(ptr)
            },
        }
    }

    pub fn unmap_host(&mut self) {
        match self.state {
            DeviceMemoryState::HostMapped => {
                self.state = DeviceMemoryState::HostUnmapped;
            }
            DeviceMemoryState::HostUnmapped => {
                self.state = DeviceMemoryState::HostMapped;
            }
        }
    }
}

#[derive(Debug)]
pub struct MemoryBinding(pub Arc<Mutex<DeviceMemory>>, pub u64);
