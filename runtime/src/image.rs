//! Image

use crate::context::NonDispatchable;
use crate::logical_device::LogicalDevice;
use crate::memory::MemoryAllocation;
use common::graphics::{DescriptorImage, MemoryBinding};
use common::math::Extent3;
use gpu::MemoryHandleStore;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct Image {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    pub(crate) format: VkFormat,
    width: u32,
    height: u32,
    gpu_binding: MemoryBinding,
}

impl Image {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        format: VkFormat,
        width: u32,
        height: u32,
        array_layers: u32,
        image_usage: VkImageUsageFlags,
    ) -> VkNonDispatchableHandle {
        info!("new Image");
        let handle = VK_NULL_HANDLE;

        let _ = array_layers;
        let _ = image_usage;

        let image = Self {
            handle,
            logical_device,
            format,
            width,
            height,
            gpu_binding: Default::default(),
        };
        image.register_object()
    }

    pub const fn size_in_bytes(&self) -> u64 {
        self.width as u64 * self.height as u64 * self.format.bytes_per_pixel() as u64
    }

    pub fn memory_requirements(&self) -> VkMemoryRequirements {
        VkMemoryRequirements {
            size: self.size_in_bytes(),
            alignment: 1,
            memoryTypeBits: self
                .logical_device
                .lock()
                .physical_device()
                .memory_type_bits_for_image(),
        }
    }

    pub fn subresource_layout(&self, subresource: &VkImageSubresource) -> VkSubresourceLayout {
        if subresource.aspectMask == VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT.into()
            && subresource.arrayLayer == 0
            && subresource.mipLevel == 0
        {
            VkSubresourceLayout {
                offset: 0,
                size: self.size_in_bytes(),
                rowPitch: self.width as u64 * self.format.bytes_per_pixel() as u64,
                arrayPitch: 0,
                depthPitch: 0,
            }
        } else {
            unimplemented!("subresource: {:?}", subresource)
        }
    }

    pub fn bind_memory(&mut self, memory: Arc<Mutex<MemoryAllocation>>, offset: u64) -> VkResult {
        self.gpu_binding.store(
            memory.lock().gpu_memory_allocation,
            offset,
            self.size_in_bytes().saturating_sub(offset),
        );
        VkResult::VK_SUCCESS
    }

    pub fn descriptor(&self) -> DescriptorImage {
        let binding = self.gpu_binding.clone();
        DescriptorImage {
            binding,
            extent: Extent3::<u32> {
                width: self.width,
                height: self.height,
                depth: 1,
            },
        }
    }
}

#[derive(Debug)]
pub struct ImageView {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
    logical_device: Arc<Mutex<LogicalDevice>>,
    pub(crate) image: Arc<Mutex<Image>>,
}

impl ImageView {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        create_info: &VkImageViewCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new ImageView");
        let handle = VK_NULL_HANDLE;

        let Some(image) = Image::from_handle(create_info.image) else {
            unreachable!()
        };

        let object = Self {
            handle,
            logical_device,
            image,
        };
        object.register_object()
    }
}
