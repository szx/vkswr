//! Instance

use crate::context::Dispatchable;

use crate::physical_device::PhysicalDevice;
use headers::c_char_array;
use headers::vk_decls::*;
use lazy_static::lazy_static;

use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

/// Contains system-level information and functionality
#[derive(Debug)]
pub struct Instance {
    pub(crate) handle: VkDispatchableHandle,
    physical_device: Arc<Mutex<PhysicalDevice>>,
}
impl Instance {
    // TODO: Remove all create() accepting create info.
    pub fn create() -> Result<VkDispatchableHandle, VkResult> {
        let physical_device = PhysicalDevice::create();
        let physical_device = PhysicalDevice::from_handle(physical_device)
            .map_or_else(|| Err(VkResult::VK_ERROR_INITIALIZATION_FAILED), Ok)?;

        let instance = Self {
            handle: VkDispatchableHandle(None),
            physical_device,
        };
        Ok(instance.register_object())
    }

    pub const fn physical_device_count() -> usize {
        1
    }

    pub fn physical_device(&self) -> Arc<Mutex<PhysicalDevice>> {
        self.physical_device.clone()
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
                extensionName: *VK_KHR_SURFACE_EXTENSION_NAME,
                specVersion: 25,
            },
            VkExtensionProperties {
                extensionName: *VK_KHR_XCB_SURFACE_EXTENSION_NAME,
                specVersion: 6,
            },
        ]
    }
}
