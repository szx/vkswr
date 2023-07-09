//! VkImage device commands

use headers::vk_decls::*;
use runtime::image::*;
use runtime::*;

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

    *pImage.as_ptr() = Image::create(device, create_info);

    VkResult::VK_SUCCESS
}
