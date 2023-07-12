//! Swapchain

use crate::image::*;
use crate::{Context, Fence, LogicalDevice, NonDispatchable, Semaphore};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct Swapchain {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    flags: VkSwapchainCreateFlagsKHR,
    surface: VkSurfaceKHR,
    pub images: Vec<Arc<Mutex<Image>>>,
    color_space: VkColorSpaceKHR,
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
        let surface = create_info.surface;

        let image_count = create_info.minImageCount;
        let mut images = Vec::with_capacity(image_count as usize);
        for _ in 0..image_count {
            let image = Image::create(
                logical_device.clone(),
                create_info.imageFormat,
                create_info.imageExtent.width,
                create_info.imageExtent.height,
                create_info.imageArrayLayers,
                create_info.imageUsage,
            );
            let Some(image) = Image::from_handle(image) else {
                unreachable!()
            };
            images.push(image);
        }

        let color_space = create_info.imageColorSpace;
        let present_mode = create_info.presentMode;

        // TODO: Parse rest of swapchain create info.
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
            images,
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
        // TODO: Acquire next swapchain image.
        let _ = timeout;
        let _ = semaphore;
        let _ = fence;
        0
    }
}

impl NonDispatchable for Swapchain {
    fn get_hash<'a>(
        context: &'a Context,
    ) -> &'a HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.swapchains
    }

    fn get_hash_mut<'a>(
        context: &'a mut Context,
    ) -> &'a mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.swapchains
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
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
