//! Swapchain

use crate::context::NonDispatchable;
use crate::fence::Fence;
use crate::image::*;
use crate::logical_device::LogicalDevice;
use crate::memory::MemoryAllocation;
use crate::semaphore::Semaphore;
use crate::surface::Surface;
use common::math::Extent3;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct Swapchain {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    flags: VkSwapchainCreateFlagsKHR,
    surface: Arc<Mutex<Surface>>,
    extent: Extent3<u32>,
    pub images: Vec<Arc<Mutex<Image>>>,
    pub memory_allocations: Vec<Arc<Mutex<MemoryAllocation>>>,
    #[allow(dead_code)]
    color_space: VkColorSpaceKHR,
    #[allow(dead_code)]
    present_mode: VkPresentModeKHR,
}

impl Swapchain {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        create_info: &VkSwapchainCreateInfoKHR,
    ) -> VkNonDispatchableHandle {
        info!("new Swapchain");
        let handle = VK_NULL_HANDLE;

        let flags = create_info.flags;
        let Some(surface) = Surface::from_handle(create_info.surface) else {
            unreachable!()
        };

        let image_count = create_info.minImageCount;
        let extent = Extent3 {
            width: create_info.imageExtent.width,
            height: create_info.imageExtent.height,
            depth: create_info.imageArrayLayers,
        };
        let mut images = Vec::with_capacity(image_count as usize);
        let mut memory_allocations = Vec::with_capacity(image_count as usize);
        for _ in 0..image_count {
            let image = Image::create(
                logical_device.clone(),
                create_info.imageFormat,
                extent.width,
                extent.height,
                extent.depth,
                create_info.imageUsage,
            );
            let Some(image) = Image::from_handle(image) else {
                unreachable!()
            };
            let memory_allocation =
                MemoryAllocation::create(logical_device.clone(), image.lock().size_in_bytes(), 0);
            let Some(memory_allocation) = MemoryAllocation::from_handle(memory_allocation) else {
                unreachable!()
            };

            image.lock().bind_memory(memory_allocation.clone(), 0);

            images.push(image);
            memory_allocations.push(memory_allocation);
        }

        let color_space = create_info.imageColorSpace;
        let present_mode = create_info.presentMode;

        warn!("TODO: Parse rest of swapchain create info");
        let _ = create_info.imageSharingMode;
        let _ = create_info.queueFamilyIndexCount;
        let _ = create_info.pQueueFamilyIndices;

        let _ = create_info.preTransform;
        let _ = create_info.compositeAlpha;

        let _ = create_info.clipped;

        let _ = create_info.oldSwapchain;

        let swapchain = Self {
            handle,
            logical_device,
            flags,
            surface,
            extent,
            images,
            memory_allocations,
            color_space,
            present_mode,
        };
        swapchain.register_object()
    }

    pub fn acquire_next_image(
        &mut self,
        timeout: u64,
        semaphore: Option<Arc<Mutex<Semaphore>>>,
        fence: Option<Arc<Mutex<Fence>>>,
    ) -> u32 {
        warn!("TODO: Acquire next swapchain image");
        let _ = timeout;
        let _ = semaphore;
        let _ = fence;
        0
    }

    pub fn present(&mut self, image_index: u32) -> Result<VkResult, VkResult> {
        let memory_allocation = self.memory_allocations[image_index as usize].clone();
        self.surface.lock().present(memory_allocation, self.extent)
    }
}

impl Debug for Swapchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("flags", &self.logical_device)
            .field("flags", &self.flags)
            .field("surface", &self.surface)
            .finish()
    }
}
