//! Image

use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct Image {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<LogicalDevice>,
}

impl Image {
    pub fn new(
        logical_device: Arc<LogicalDevice>,
        image_format: VkFormat,
        image_extent: VkExtent2D,
        image_array_layers: u32,
        image_usage: VkImageUsageFlags,
    ) -> VkNonDispatchableHandle {
        info!("new Image");
        let handle = VK_NULL_HANDLE;
        let logical_device = logical_device.clone();

        let image = Self {
            handle,
            logical_device,
        };
        image.register_object()
    }
}

impl NonDispatchable for Image {
    fn get_hash<'a>(
        context: &'a Context,
    ) -> &'a HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.images
    }

    fn get_hash_mut<'a>(
        context: &'a mut Context,
    ) -> &'a mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.images
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("logical_device", &self.logical_device)
            .finish()
    }
}
