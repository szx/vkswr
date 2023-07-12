//! Descriptors

use crate::memory::{DeviceMemory, MemoryBinding};
use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct DescriptorSetLayout {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
}

impl DescriptorSetLayout {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkDescriptorSetLayoutCreateFlags,
        bindings: &[VkDescriptorSetLayoutBinding],
    ) -> VkNonDispatchableHandle {
        info!("new DescriptorSetLayouts");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let _ = bindings;

        let object = Self {
            handle,
            logical_device,
        };
        object.register_object()
    }
}

impl NonDispatchable for DescriptorSetLayout {
    fn get_hash<'a>(
        context: &'a Context,
    ) -> &'a HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.descriptor_set_layouts
    }

    fn get_hash_mut<'a>(
        context: &'a mut Context,
    ) -> &'a mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.descriptor_set_layouts
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}
