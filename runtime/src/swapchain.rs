//! Swapchain: a set of images presented to a surface.

use crate::context::VulkanObject;
use crate::fence::Fence;
use crate::image::Image;
use crate::logical_device::LogicalDevice;
use crate::memory::MemoryAllocation;
use crate::semaphore::Semaphore;
use crate::surface::Surface;
use common::math::Extent3;
use headers::vk_decls::*;
use log::{info, warn};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct Swapchain {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
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
        let surface = Surface::from_handle(create_info.surface).expect("surface handle is live");

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
                Arc::clone(&logical_device),
                create_info.imageFormat,
                extent.width,
                extent.height,
                extent.depth,
                create_info.imageUsage,
            );
            let image = Image::from_handle(image).expect("image handle is live");
            let memory_allocation = MemoryAllocation::create(
                Arc::clone(&logical_device),
                image.lock().size_in_bytes(),
                0,
            );
            let memory_allocation =
                MemoryAllocation::from_handle(memory_allocation).expect("memory handle is live");

            image.lock().bind_memory(Arc::clone(&memory_allocation), 0);

            images.push(image);
            memory_allocations.push(memory_allocation);
        }

        warn!("TODO: Parse the rest of the swapchain create info");
        let _ = (
            create_info.imageSharingMode,
            create_info.queueFamilyIndexCount,
            create_info.pQueueFamilyIndices,
            create_info.preTransform,
            create_info.compositeAlpha,
            create_info.clipped,
            create_info.oldSwapchain,
        );

        Self {
            handle: VK_NULL_HANDLE,
            flags: create_info.flags,
            surface,
            extent,
            images,
            memory_allocations,
            color_space: create_info.imageColorSpace,
            present_mode: create_info.presentMode,
        }
        .register()
    }

    pub fn acquire_next_image(
        &mut self,
        _timeout: u64,
        _semaphore: Option<Arc<Mutex<Semaphore>>>,
        _fence: Option<Arc<Mutex<Fence>>>,
    ) -> u32 {
        warn!("TODO: Acquire next swapchain image");
        0
    }

    pub fn present(&mut self, image_index: u32) -> Result<VkResult, VkResult> {
        let memory_allocation = self
            .memory_allocations
            .get(image_index as usize)
            .expect("swapchain image index is valid")
            .clone();
        self.surface.lock().present(memory_allocation, self.extent)
    }
}
