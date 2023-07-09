//! Device memory

use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Debug)]
pub struct DeviceMemory {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    size: u64,
    // TODO: Allocate actual memory.
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
            size,
        };
        object.register_object()
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
