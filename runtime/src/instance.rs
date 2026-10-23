//! Instance: system-level Vulkan state.

use crate::context::VulkanObject;
use crate::physical_device::PhysicalDevice;
use headers::c_char_array;
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

    pub fn extension_count() -> usize {
        Self::extension_properties().len()
    }

    pub fn extension_properties() -> [VkExtensionProperties; 2] {
        c_char_array!(
            VK_KHR_SURFACE_EXTENSION_NAME,
            VK_MAX_EXTENSION_NAME_SIZE,
            "VK_KHR_surface"
        );
        c_char_array!(
            VK_KHR_XCB_SURFACE_EXTENSION_NAME,
            VK_MAX_EXTENSION_NAME_SIZE,
            "VK_KHR_xcb_surface"
        );
        [
            VkExtensionProperties {
                extensionName: VK_KHR_SURFACE_EXTENSION_NAME,
                specVersion: 25,
            },
            VkExtensionProperties {
                extensionName: VK_KHR_XCB_SURFACE_EXTENSION_NAME,
                specVersion: 6,
            },
        ]
    }
}
