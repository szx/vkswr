//! Logical device: an application's connection to a physical device.

use crate::context::VulkanObject;
use crate::fence::Fence;
use crate::physical_device::PhysicalDevice;
use crate::queue::Queue;
use headers::vk_decls::*;
use log::{info, warn};
use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;

/// Identifier used to associate functions with a `PhysicalDevice`.
#[derive(Debug)]
pub struct LogicalDevice {
    pub(crate) handle: VkDispatchableHandle,
    physical_device: Arc<Mutex<PhysicalDevice>>,
    #[allow(dead_code)]
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

        if enabled_features
            .is_some_and(|features| !physical_device.lock().supports_features(features))
        {
            return Err(VkResult::VK_ERROR_FEATURE_NOT_PRESENT);
        }

        let queue = Queue::create(Arc::clone(&physical_device), queue_create_info);
        let queue = Queue::from_handle(queue).ok_or(VkResult::VK_ERROR_INITIALIZATION_FAILED)?;

        let enabled_features = enabled_features
            .copied()
            .unwrap_or_else(|| physical_device.lock().features());

        Ok(Self {
            handle: VkDispatchableHandle(None),
            physical_device,
            enabled_features,
            queue,
        }
        .register())
    }

    pub fn physical_device(&self) -> MutexGuard<'_, PhysicalDevice> {
        self.physical_device.lock()
    }

    pub fn queue(&self, _queue_family_index: u32, _queue_index: u32) -> Arc<Mutex<Queue>> {
        Arc::clone(&self.queue)
    }

    pub fn wait_for_fences(&self, fences: Vec<Arc<Mutex<Fence>>>, _wait_all: bool, _timeout: u64) {
        for _fence in fences {
            warn!("TODO: Wait for one or more fences to become signaled");
        }
    }

    pub fn reset_fences(&self, fences: Vec<Arc<Mutex<Fence>>>) {
        for fence in fences {
            warn!("TODO: VUID-vkResetFences-pFences-01123");
            fence.lock().reset();
        }
    }

    pub fn wait_idle(&self) -> VkResult {
        warn!("TODO: LogicalDevice wait idle");
        VkResult::VK_SUCCESS
    }

    pub const fn flush_memory_ranges(&self, _memory_ranges: &[VkMappedMemoryRange]) -> VkResult {
        // No-op for host-coherent memory.
        VkResult::VK_SUCCESS
    }

    pub const fn invalidate_memory_ranges(
        &self,
        _memory_ranges: &[VkMappedMemoryRange],
    ) -> VkResult {
        // No-op for host-coherent memory.
        VkResult::VK_SUCCESS
    }
}
