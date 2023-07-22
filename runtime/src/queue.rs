//! Queue

use crate::command_buffer::CommandBuffer;
use crate::context::Dispatchable;
use crate::memory::MemoryAllocation;
use crate::physical_device::PhysicalDevice;
use crate::semaphore::Semaphore;
use crate::swapchain::Swapchain;
use headers::vk_decls::*;
use itertools::izip;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::Arc;

/// Queue associated with `LogicalDevice`.
#[derive(Debug)]
pub struct Queue {
    pub(crate) handle: VkDispatchableHandle,
    physical_device: Arc<Mutex<PhysicalDevice>>,
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
    ) {
        info!("Queue::present");

        for (swapchain, image_index, result) in izip!(swapchains, image_indices, results) {
            // TODO: Queue present.
            let _ = wait_semaphores;
            let _ = swapchain;
            let _ = image_index;
            *result = VkResult::VK_SUCCESS;
        }
    }

    pub fn wait_idle(&self) -> VkResult {
        info!("Queue::wait_idle");
        // TODO: LogicalDevice wait idle.
        VkResult::VK_SUCCESS
    }
}
