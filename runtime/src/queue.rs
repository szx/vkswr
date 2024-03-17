//! Queue

use crate::command_buffer::CommandBuffer;
use crate::context::Dispatchable;

use crate::physical_device::PhysicalDevice;
use crate::semaphore::Semaphore;
use crate::swapchain::Swapchain;
use headers::vk_decls::*;

use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;

use std::sync::Arc;

/// Queue associated with `LogicalDevice`.
#[derive(Debug)]
pub struct Queue {
    pub(crate) handle: VkDispatchableHandle,
    physical_device: Arc<Mutex<PhysicalDevice>>,
    #[allow(dead_code)]
    flags: VkDeviceQueueCreateFlags,
}

impl Queue {
    pub fn create(
        physical_device: Arc<Mutex<PhysicalDevice>>,
        create_info: &VkDeviceQueueCreateInfo,
    ) -> VkDispatchableHandle {
        info!("new Queue");
        let flags = create_info.flags;

        let queue = Self {
            handle: VkDispatchableHandle(None),
            physical_device,
            flags,
        };
        queue.register_object()
    }

    pub fn submit<'a>(
        &mut self,
        wait_semaphores: impl IntoIterator<Item = Arc<Mutex<Semaphore>>>,
        wait_semaphores_stage_flags: impl IntoIterator<Item = &'a VkPipelineStageFlags>,
        signal_semaphores: impl IntoIterator<Item = Arc<Mutex<Semaphore>>>,
        command_buffers: impl IntoIterator<Item = Arc<Mutex<CommandBuffer>>>,
    ) {
        info!("Queue::submit");
        let _ = wait_semaphores.into_iter();
        let _ = wait_semaphores_stage_flags.into_iter();
        let _ = signal_semaphores.into_iter();
        for command_buffer in command_buffers {
            let gpu = &mut self.physical_device.lock().gpu;
            gpu.submit(command_buffer.lock().gpu_command_buffer_for_submit());
        }
    }

    pub fn present<'a>(
        &mut self,
        wait_semaphores: impl IntoIterator<Item = Arc<Mutex<Semaphore>>>,
        swapchains: impl IntoIterator<Item = Arc<Mutex<Swapchain>>>,
        image_indices: impl IntoIterator<Item = &'a u32>,
        results: impl IntoIterator<Item = &'a mut VkResult>,
    ) -> Result<VkResult, VkResult> {
        let _ = wait_semaphores.into_iter();
        let mut swapchains = swapchains.into_iter();
        let mut image_indices = image_indices.into_iter();
        let mut results = results.into_iter();
        let mut last_failure = Ok(VkResult::VK_SUCCESS);
        loop {
            let (Some(swapchain), Some(image_index), result) =
                (swapchains.next(), image_indices.next(), results.next())
            else {
                return last_failure;
            };
            let last_result = swapchain.lock().present(*image_index);
            if let Some(result) = result {
                *result = match last_result {
                    Ok(result) => result,
                    Err(result) => result,
                };
            }
            if last_failure.is_ok() && last_result.is_err() {
                last_failure = last_result;
            }
        }
    }

    pub fn wait_idle(&self) -> VkResult {
        warn!("TODO: LogicalDevice wait idle");
        VkResult::VK_SUCCESS
    }
}
