//! VkImage device commands

use headers::vk_decls::*;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::image::*;
use runtime::logical_device::LogicalDevice;
use runtime::memory::DeviceMemory;
use runtime::*;
use std::ops::Deref;

pub unsafe extern "C" fn vkCreateImageView(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkImageViewCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pView: Option<NonNull<VkImageView>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let Some(pView) = pView else { unreachable!() };

    *pView.as_ptr() = ImageView::create(device, create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyImageView(
    device: VkDevice,
    imageView: VkImageView,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    ImageView::drop_handle(imageView);
}

pub unsafe extern "C" fn vkCreateImage(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkImageCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pImage: Option<NonNull<VkImage>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let Some(pImage) = pImage else { unreachable!() };

    *pImage.as_ptr() = Image::create(
        device,
        create_info.format,
        create_info.extent.width,
        create_info.extent.height,
        create_info.arrayLayers,
        create_info.usage,
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyImage(
    device: VkDevice,
    image: VkImage,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    Image::drop_handle(image);
}

pub unsafe extern "C" fn vkGetImageMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(image) = Image::from_handle(image) else {
        unreachable!()
    };

    let Some(pMemoryRequirements) = pMemoryRequirements else {
        unreachable!()
    };

    *pMemoryRequirements.as_ptr() = image.lock().memory_requirements();
}

pub unsafe extern "C" fn vkGetImageSubresourceLayout(
    device: VkDevice,
    image: VkImage,
    pSubresource: Option<NonNull<VkImageSubresource>>,
    pLayout: Option<NonNull<VkSubresourceLayout>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(image) = Image::from_handle(image) else {
        unreachable!()
    };

    let Some(pSubresource) = pSubresource else {
        unreachable!()
    };
    let subresource = pSubresource.as_ref();

    let Some(pLayout) = pLayout else {
        unreachable!()
    };

    *pLayout.as_ptr() = image.lock().subresource_layout(subresource);
}

pub unsafe extern "C" fn vkBindImageMemory(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(image) = Image::from_handle(image) else {
        unreachable!()
    };

    let Some(memory) = DeviceMemory::from_handle(memory) else {
        unreachable!()
    };

    let result = image.lock().bind_memory(memory, memoryOffset);
    result
}
