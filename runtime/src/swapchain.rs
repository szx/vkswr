//! Swapchain

use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct Swapchain {
    logical_device: Arc<LogicalDevice>,
    flags: VkSwapchainCreateFlagsKHR,
    surface: VkSurfaceKHR,
}

impl Swapchain {
    pub fn create(
        logical_device: Arc<LogicalDevice>,
        create_info: &VkSwapchainCreateInfoKHR,
    ) -> VkNonDispatchableHandle {
        info!("new Swapchain");
        let logical_device = logical_device.clone();
        let flags = create_info.flags;
        let surface = create_info.surface;
        // TODO: Parse rest of create info.
        let image_count = create_info.minImageCount;
        let image_format = create_info.imageFormat;
        let image_color_space = create_info.imageColorSpace;
        let image_extent = create_info.imageExtent;
        let image_array_layers = create_info.imageArrayLayers;
        let image_usage = create_info.imageUsage;
        let image_sharing_mode = create_info.imageSharingMode;
        let queue_family_index_count = create_info.queueFamilyIndexCount;
        let queue_family_indices =
            if let Some(pQueueFamilyIndices) = create_info.pQueueFamilyIndices {
                unsafe {
                    std::slice::from_raw_parts(
                        create_info.pQueueFamilyIndices.unwrap().as_ptr(),
                        queue_family_index_count as usize,
                    )
                }
            } else {
                &[]
            };
        let pre_transform = create_info.preTransform;
        let composite_alpha = create_info.compositeAlpha;
        let present_mode = create_info.presentMode;
        let clipped = create_info.clipped;
        let old_swapchain = create_info.oldSwapchain;

        let surface = Self {
            logical_device,
            flags,
            surface,
        };
        surface.register_handle()
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
