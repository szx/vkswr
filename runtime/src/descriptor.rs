//! Descriptors

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use crate::memory::MemoryAllocation;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct DescriptorSetLayout {
    pub(crate) handle: VkNonDispatchableHandle,
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

#[derive(Debug)]
pub struct DescriptorPool {
    pub(crate) handle: VkNonDispatchableHandle,
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

#[derive(Debug)]
pub struct DescriptorSet {
    pub(crate) handle: VkNonDispatchableHandle,
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
