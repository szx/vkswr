//! LogicalDevice

use crate::context::Dispatchable;
use crate::fence::Fence;
use crate::memory::{DeviceMemory, MemoryBinding};
use crate::physical_device::PhysicalDevice;
use crate::queue::Queue;
use headers::c_char_array;
use headers::vk_decls::*;
use lazy_static::lazy_static;
use log::*;
use parking_lot::{Mutex, MutexGuard, RawMutex};
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

/// Identifier used to associate functions with a `PhysicalDevice`.
#[derive(Debug)]
pub struct LogicalDevice {
    pub(crate) handle: VkDispatchableHandle,
    driver_name: &'static str,
    physical_device: Arc<Mutex<PhysicalDevice>>,
    enabled_features: VkPhysicalDeviceFeatures,
    queue: Arc<Mutex<Queue>>,
}

impl LogicalDevice {
    pub fn create(
        physical_device: Arc<Mutex<PhysicalDevice>>,
        enabled_features: Option<&VkPhysicalDeviceFeatures>,
        queue_create_info: &VkDeviceQueueCreateInfo,
    ) -> Result<VkDispatchableHandle, VkResult> {
        info!("new LogicalDevice");

        if enabled_features.is_some_and(|x| !physical_device.lock().supports_features(x)) {
            Err(VkResult::VK_ERROR_FEATURE_NOT_PRESENT)?;
        }

        let queue = Queue::create(physical_device.clone(), queue_create_info);
        let queue = Queue::from_handle(queue).unwrap();
        let logical_device = Self {
            handle: VkDispatchableHandle(None),
            driver_name: "vulkan_software_rasterizer",
            physical_device: physical_device.clone(),
            enabled_features: *enabled_features.unwrap_or(&physical_device.lock().features()),
            queue,
        };
        Ok(logical_device.register_object())
    }

    pub fn physical_device(&self) -> MutexGuard<'_, PhysicalDevice> {
        self.physical_device.lock()
    }
}

impl LogicalDevice {
    pub fn queue(&self, queue_family_index: u32, queue_index: u32) -> Arc<Mutex<Queue>> {
        let _ = queue_family_index;
        let _ = queue_index;
        self.queue.clone()
    }

    pub fn wait_for_fences(&self, fences: Vec<Arc<Mutex<Fence>>>, wait_all: bool, timeout: u64) {
        let _ = wait_all;
        let _ = timeout;
        for fence in fences {
            if fence.lock().logical_device.data_ptr() as *const _ != self as *const _ {
                continue;
            }
            // TODO: Wait for one or more fences to become signaled.
        }
    }

    pub fn reset_fences(&self, fences: Vec<Arc<Mutex<Fence>>>) {
        for mut fence in fences {
            // TODO: VUID-vkResetFences-pFences-01123
            fence.lock().reset();
        }
    }

    pub fn wait_idle(&self) -> VkResult {
        // TODO: LogicalDevice wait idle.
        VkResult::VK_SUCCESS
    }

    pub const fn flush_memory_ranges(&self, _memory_ranges: &[VkMappedMemoryRange]) -> VkResult {
        // No-op.
        VkResult::VK_SUCCESS
    }

    pub const fn invalidate_memory_ranges(
        &self,
        _memory_ranges: &[VkMappedMemoryRange],
    ) -> VkResult {
        // No-op.
        VkResult::VK_SUCCESS
    }
}
