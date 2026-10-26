//! Instance: system-level Vulkan state.

use crate::context::VulkanObject;
use crate::extensions::{INSTANCE_EXTENSION_COUNT, INSTANCE_EXTENSION_PROPERTIES};
use crate::physical_device::PhysicalDevice;
use headers::vk_decls::*;
use parking_lot::Mutex;
use std::sync::Arc;

/// Contains system-level information and functionality.
#[derive(Debug)]
pub struct Instance {
    pub(crate) handle: VkDispatchableHandle,
    physical_device: Arc<Mutex<PhysicalDevice>>,
}

impl Instance {
    pub fn create() -> Result<VkDispatchableHandle, VkResult> {
        let physical_device = PhysicalDevice::create();
        let physical_device = PhysicalDevice::from_handle(physical_device)
            .ok_or(VkResult::VK_ERROR_INITIALIZATION_FAILED)?;

        Ok(Self {
            handle: VkDispatchableHandle(None),
            physical_device,
        }
        .register())
    }

    pub const fn physical_device_count() -> usize {
        1
    }

    pub fn physical_device(&self) -> Arc<Mutex<PhysicalDevice>> {
        Arc::clone(&self.physical_device)
    }

    pub const fn extension_count() -> usize {
        INSTANCE_EXTENSION_COUNT
    }

    pub const fn extension_properties() -> [VkExtensionProperties; INSTANCE_EXTENSION_COUNT] {
        INSTANCE_EXTENSION_PROPERTIES
    }
}
