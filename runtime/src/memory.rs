//! Device memory

use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Debug)]
pub struct DeviceMemory {
    handle: VkNonDispatchableHandle,
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

    pub fn map_host(&mut self) -> Result<NonNull<std::ffi::c_void>, VkResult> {
        match self.state {
            DeviceMemoryState::HostMapped => Err(VkResult::VK_ERROR_MEMORY_MAP_FAILED),
            DeviceMemoryState::HostUnmapped => unsafe {
                self.state = DeviceMemoryState::HostMapped;
                let ptr = NonNull::new_unchecked(self.data.as_mut_ptr() as *mut c_void);
                Ok(ptr)
            },
        }
    }

    pub fn unmap_host(&mut self) {
        match self.state {
            DeviceMemoryState::HostMapped => {
                self.state = DeviceMemoryState::HostUnmapped;
            }
            DeviceMemoryState::HostUnmapped => {}
        }
    }
}

impl NonDispatchable for DeviceMemory {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.device_memories
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.device_memories
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

#[derive(Debug)]
pub struct MemoryBinding(pub Arc<Mutex<DeviceMemory>>, pub u64);
