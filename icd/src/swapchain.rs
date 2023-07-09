//! VK_KHR_swapchain extension device commands

use headers::vk_decls::*;
use runtime::image::Image;
use runtime::swapchain::*;
use runtime::*;

pub unsafe extern "C" fn vkCreateSwapchainKHR(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSwapchainCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSwapchain: Option<NonNull<VkSwapchainKHR>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
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

    *pSwapchain.as_ptr() = Swapchain::create(device, create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroySwapchainKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    Swapchain::drop_handle(swapchain);
}

pub unsafe extern "C" fn vkGetSwapchainImagesKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: Option<NonNull<u32>>,
    pSwapchainImages: Option<NonNull<VkImage>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(swapchain) = Swapchain::from_handle(swapchain) else {
        unreachable!()
    };

    let Some(pSwapchainImageCount) = pSwapchainImageCount else {
        unreachable!()
    };

    pSwapchainImages.map_or_else(
        || {
            *pSwapchainImageCount.as_ptr() = swapchain.lock().images.len() as u32;
            VkResult::VK_SUCCESS
        },
        |pSwapchainImages| {
            let images = swapchain
                .lock()
                .images
                .iter()
                .map(|x| x.lock().get_handle())
                .collect::<Vec<_>>();
            std::ptr::copy_nonoverlapping(
                images.as_ptr(),
                pSwapchainImages.as_ptr(),
                *pSwapchainImageCount.as_ptr() as usize,
            );
            VkResult::VK_SUCCESS
        },
    )
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
