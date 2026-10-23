//! Image and image view objects.

use crate::context::VulkanObject;
use crate::logical_device::LogicalDevice;
use crate::memory::MemoryAllocation;
use common::graphics::{DescriptorImage, MemoryBinding};
use common::math::Extent3;
use gpu::MemoryHandleStore;
use headers::vk_decls::*;
use log::info;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct Image {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
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
        _array_layers: u32,
        _image_usage: VkImageUsageFlags,
    ) -> VkNonDispatchableHandle {
        info!("new Image");
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
            format,
            width,
            height,
            gpu_binding: MemoryBinding::default(),
        }
        .register()
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
        let color_aspect: u32 = VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT.into();
        if subresource.aspectMask == color_aspect
            && subresource.arrayLayer == 0
            && subresource.mipLevel == 0
        {
            VkSubresourceLayout {
                offset: 0,
                size: self.size_in_bytes(),
                rowPitch: u64::from(self.width) * u64::from(self.format.bytes_per_pixel()),
                arrayPitch: 0,
                depthPitch: 0,
            }
        } else {
            unimplemented!("subresource: {subresource:?}")
        }
    }

    pub fn bind_memory(&mut self, memory: Arc<Mutex<MemoryAllocation>>, offset: u64) -> VkResult {
        self.gpu_binding.store(
            memory.lock().gpu_memory_allocation,
            offset,
            self.size_in_bytes(),
        );
        VkResult::VK_SUCCESS
    }

    pub fn descriptor(&self) -> DescriptorImage {
        DescriptorImage {
            binding: self.gpu_binding.clone(),
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
        let image = Image::from_handle(create_info.image).expect("image handle is live");
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
            image,
        }
        .register()
    }
}
