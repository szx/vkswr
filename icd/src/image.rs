//! VkImage device commands

use headers::vk_decls::*;
use runtime::context::VulkanObject;
use runtime::image::*;
use runtime::logical_device::LogicalDevice;
use runtime::memory::MemoryAllocation;

pub unsafe extern "C" fn vkCreateImageView(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkImageViewCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pView: Option<NonNull<VkImageView>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let pView = pView.expect("Vulkan guarantees a non-null pointer");

    *pView.as_ptr() = ImageView::create(device, create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyImageView(
    device: VkDevice,
    imageView: VkImageView,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    ImageView::drop_handle(imageView);
}

pub unsafe extern "C" fn vkCreateImage(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkImageCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pImage: Option<NonNull<VkImage>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let pImage = pImage.expect("Vulkan guarantees a non-null pointer");

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
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    Image::drop_handle(image);
}

pub unsafe extern "C" fn vkGetImageMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let image = Image::from_handle(image).expect("Vulkan handle refers to a live object");

    let pMemoryRequirements = pMemoryRequirements.expect("Vulkan guarantees a non-null pointer");

    *pMemoryRequirements.as_ptr() = image.lock().memory_requirements();
}

pub unsafe extern "C" fn vkGetImageSubresourceLayout(
    device: VkDevice,
    image: VkImage,
    pSubresource: Option<NonNull<VkImageSubresource>>,
    pLayout: Option<NonNull<VkSubresourceLayout>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let image = Image::from_handle(image).expect("Vulkan handle refers to a live object");

    let pSubresource = pSubresource.expect("Vulkan guarantees a non-null pointer");
    let subresource = pSubresource.as_ref();

    let pLayout = pLayout.expect("Vulkan guarantees a non-null pointer");

    *pLayout.as_ptr() = image.lock().subresource_layout(subresource);
}

pub unsafe extern "C" fn vkBindImageMemory(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let image = Image::from_handle(image).expect("Vulkan handle refers to a live object");

    let memory =
        MemoryAllocation::from_handle(memory).expect("Vulkan handle refers to a live object");

    image.lock().bind_memory(memory, memoryOffset)
}
