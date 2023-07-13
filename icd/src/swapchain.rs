//! VK_KHR_swapchain extension device commands

use headers::vk_decls::*;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::fence::Fence;
use runtime::image::Image;
use runtime::logical_device::LogicalDevice;
use runtime::queue::Queue;
use runtime::semaphore::Semaphore;
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
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(swapchain) = Swapchain::from_handle(swapchain) else {
        unreachable!()
    };

    let semaphore = Semaphore::from_handle(semaphore);

    let fence = Fence::from_handle(fence);

    let Some(pImageIndex) = pImageIndex else {
        unreachable!()
    };

    *pImageIndex.as_ptr() = swapchain
        .lock()
        .acquire_next_image(timeout, semaphore, fence);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkQueuePresentKHR(
    queue: VkQueue,
    pPresentInfo: Option<NonNull<VkPresentInfoKHR>>,
) -> VkResult {
    let Some(queue) = Queue::from_handle(queue) else {
        unreachable!()
    };

    let Some(pPresentInfo) = pPresentInfo else {
        unreachable!()
    };
    let present_info = pPresentInfo.as_ref();
    let wait_semaphores = present_info
        .pWaitSemaphores
        .map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), present_info.waitSemaphoreCount as usize)
        })
        .iter()
        .map(|&handle| Semaphore::from_handle(handle).unwrap());
    let swapchains = present_info
        .pSwapchains
        .map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), present_info.swapchainCount as usize)
        })
        .iter()
        .map(|&handle| Swapchain::from_handle(handle).unwrap());
    let image_indices = present_info.pImageIndices.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), present_info.swapchainCount as usize)
    });
    let results = present_info.pResults.map_or(&mut [] as &mut [_], |x| {
        std::slice::from_raw_parts_mut(x.as_ptr(), present_info.swapchainCount as usize)
    });

    queue
        .lock()
        .present(wait_semaphores, swapchains, image_indices, results);

    VkResult::VK_SUCCESS
}
