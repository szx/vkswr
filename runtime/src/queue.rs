//! Queue: submission and presentation.

use crate::command_buffer::CommandBuffer;
use crate::context::VulkanObject;
use crate::physical_device::PhysicalDevice;
use crate::semaphore::Semaphore;
use crate::swapchain::Swapchain;
use headers::vk_decls::*;
use log::{info, warn};
use parking_lot::Mutex;
use std::sync::Arc;

/// Queue associated with a [`LogicalDevice`](crate::logical_device::LogicalDevice).
#[derive(Debug)]
pub struct Queue {
    pub(crate) handle: VkDispatchableHandle,
    physical_device: Arc<Mutex<PhysicalDevice>>,
}

impl Queue {
    pub fn create(
        physical_device: Arc<Mutex<PhysicalDevice>>,
        _create_info: &VkDeviceQueueCreateInfo,
    ) -> VkDispatchableHandle {
        info!("new Queue");
        Self {
            handle: VkDispatchableHandle(None),
            physical_device,
        }
        .register()
    }

    pub fn submit<W, S, T, C>(
        &mut self,
        _wait_semaphores: W,
        _wait_semaphores_stage_flags: S,
        _signal_semaphores: T,
        command_buffers: C,
    ) where
        W: IntoIterator<Item = Arc<Mutex<Semaphore>>>,
        S: IntoIterator,
        T: IntoIterator<Item = Arc<Mutex<Semaphore>>>,
        C: IntoIterator<Item = Arc<Mutex<CommandBuffer>>>,
    {
        info!("Queue::submit");
        for command_buffer in command_buffers {
            self.physical_device
                .lock()
                .gpu
                .submit(command_buffer.lock().gpu_command_buffer_for_submit());
        }
    }

    pub fn present<'a, W, S, I, R>(
        &mut self,
        _wait_semaphores: W,
        swapchains: S,
        image_indices: I,
        results: R,
    ) -> Result<VkResult, VkResult>
    where
        W: IntoIterator<Item = Arc<Mutex<Semaphore>>>,
        S: IntoIterator<Item = Arc<Mutex<Swapchain>>>,
        I: IntoIterator<Item = &'a u32>,
        R: IntoIterator<Item = &'a mut VkResult>,
    {
        let mut last_failure = Ok(VkResult::VK_SUCCESS);
        for ((swapchain, &image_index), result) in
            swapchains.into_iter().zip(image_indices).zip(results)
        {
            let last_result = swapchain.lock().present(image_index);
            *result = last_result.unwrap_or_else(|result| result);
            if last_failure.is_ok() && last_result.is_err() {
                last_failure = last_result;
            }
        }
        last_failure
    }

    pub fn wait_idle(&self) -> VkResult {
        warn!("TODO: LogicalDevice wait idle");
        VkResult::VK_SUCCESS
    }
}
