//! Descriptor set layouts, pools, and sets.

use crate::context::VulkanObject;
use crate::logical_device::LogicalDevice;
use headers::vk_decls::*;
use log::info;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct DescriptorSetLayout {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
    logical_device: Arc<Mutex<LogicalDevice>>,
}

impl DescriptorSetLayout {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        _flags: VkDescriptorSetLayoutCreateFlags,
        _bindings: &[VkDescriptorSetLayoutBinding],
    ) -> VkNonDispatchableHandle {
        info!("new DescriptorSetLayout");
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
        }
        .register()
    }
}

#[derive(Debug)]
pub struct DescriptorPool {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
    logical_device: Arc<Mutex<LogicalDevice>>,
}

impl DescriptorPool {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        _flags: VkDescriptorPoolCreateFlags,
        _max_sets: u32,
        _pool_sizes: &[VkDescriptorPoolSize],
    ) -> VkNonDispatchableHandle {
        info!("new DescriptorPool");
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
        }
        .register()
    }
}

#[derive(Debug)]
pub struct DescriptorSet {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
    logical_device: Arc<Mutex<LogicalDevice>>,
    #[allow(dead_code)]
    descriptor_pool: Arc<Mutex<DescriptorPool>>,
}

impl DescriptorSet {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        descriptor_pool: Arc<Mutex<DescriptorPool>>,
        _set_layout: VkDescriptorSetLayout,
    ) -> VkNonDispatchableHandle {
        info!("new DescriptorSet");
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
            descriptor_pool,
        }
        .register()
    }
}
