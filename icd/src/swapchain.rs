//! VK_KHR_swapchain extension device commands

use headers::vk_decls::*;
use runtime::swapchain::*;
use runtime::*;

pub unsafe extern "C" fn vkCreateSwapchainKHR(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSwapchainCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSwapchain: Option<NonNull<VkSwapchainKHR>>,
) -> VkResult {
    let Some(device) = LogicalDevice::get_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let Some(pSwapchain) = pSwapchain else {
        unreachable!()
    };

    Swapchain::set_handle(pSwapchain, Swapchain::create(device, create_info));

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroySwapchainKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    todo!("vkDestroySwapchainKHR(device, swapchain, pAllocator")
}

pub unsafe extern "C" fn vkGetSwapchainImagesKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: Option<NonNull<u32>>,
    pSwapchainImages: Option<NonNull<VkImage>>,
) -> VkResult {
    todo!("vkGetSwapchainImagesKHR(device, swapchain, pSwapchainImageCount, pSwapchainImages")
}

pub unsafe extern "C" fn vkAcquireNextImageKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    pImageIndex: Option<NonNull<u32>>,
) -> VkResult {
    todo!("vkAcquireNextImageKHR(device, swapchain, timeout, semaphore, fence, pImageIndex")
}

pub unsafe extern "C" fn vkQueuePresentKHR(
    queue: VkQueue,
    pPresentInfo: Option<NonNull<VkPresentInfoKHR>>,
) -> VkResult {
    todo!("vkQueuePresentKHR(queue, pPresentInfo")
}
