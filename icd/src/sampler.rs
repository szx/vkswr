//! VkSampler device commands

use headers::vk_decls::*;
use runtime::context::VulkanObject;
use runtime::logical_device::LogicalDevice;
use runtime::sampler::*;

pub unsafe extern "C" fn vkCreateSampler(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSamplerCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSampler: Option<NonNull<VkSampler>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let pSampler = pSampler.expect("Vulkan guarantees a non-null pointer");

    *pSampler.as_ptr() = Sampler::create(device, create_info.flags);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroySampler(
    device: VkDevice,
    sampler: VkSampler,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    Sampler::drop_handle(sampler);
}
