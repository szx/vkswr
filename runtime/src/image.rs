//! Image

use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Debug)]
pub struct Image {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<LogicalDevice>,
}

impl Image {
    pub fn new(
        logical_device: Arc<LogicalDevice>,
        format: VkFormat,
        extent: VkExtent2D,
        array_layers: u32,
        image_usage: VkImageUsageFlags,
    ) -> VkNonDispatchableHandle {
        info!("new Image");
        let handle = VK_NULL_HANDLE;
        let logical_device = logical_device.clone();
        let _ = format;
        let _ = extent;
        let _ = array_layers;
        let _ = image_usage;

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

#[derive(Debug)]
pub struct ImageView {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<LogicalDevice>,
    image: Arc<Mutex<Image>>,
}

impl ImageView {
    pub fn create(
        logical_device: Arc<LogicalDevice>,
        create_info: &VkImageViewCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new ImageView");
        let handle = VK_NULL_HANDLE;
        let logical_device = logical_device.clone();
        let image = Image::from_handle(create_info.image);

        let image = Self {
            handle,
            logical_device,
            image,
        };
        image.register_object()
    }
}

impl NonDispatchable for ImageView {
    fn get_hash<'a>(
        context: &'a Context,
    ) -> &'a HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.image_views
    }

    fn get_hash_mut<'a>(
        context: &'a mut Context,
    ) -> &'a mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.image_views
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}
