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
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.descriptor_set_layouts
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.descriptor_set_layouts
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

#[derive(Debug)]
pub struct DescriptorPool {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
}

impl DescriptorPool {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkDescriptorSetLayoutCreateFlags,
        max_sets: u32,
        pool_sizes: &[VkDescriptorPoolSize],
    ) -> VkNonDispatchableHandle {
        info!("new DescriptorPool");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let _ = max_sets;
        let _ = pool_sizes;

        let object = Self {
            handle,
            logical_device,
        };
        object.register_object()
    }
}

impl NonDispatchable for DescriptorPool {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.descriptor_pools
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.descriptor_pools
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

#[derive(Debug)]
pub struct DescriptorSet {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    descriptor_pool: Arc<Mutex<DescriptorPool>>,
}

impl DescriptorSet {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        descriptor_pool: Arc<Mutex<DescriptorPool>>,
        set_layout: &VkDescriptorSetLayout,
    ) -> VkNonDispatchableHandle {
        info!("new DescriptorSet");
        let handle = VK_NULL_HANDLE;

        let _ = set_layout;

        let object = Self {
            handle,
            logical_device,
            descriptor_pool,
        };
        object.register_object()
    }
}

impl NonDispatchable for DescriptorSet {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.descriptor_sets
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.descriptor_sets
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}
