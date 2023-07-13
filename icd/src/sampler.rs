//! VkSampler device commands

use headers::vk_decls::*;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::logical_device::LogicalDevice;
use runtime::sampler::*;
use runtime::*;

pub unsafe extern "C" fn vkCreateSampler(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSamplerCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSampler: Option<NonNull<VkSampler>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let Some(pSampler) = pSampler else {
        unreachable!()
    };

    *pSampler.as_ptr() = Sampler::create(device, create_info.flags);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroySampler(
    device: VkDevice,
    sampler: VkSampler,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    Sampler::drop_handle(sampler);
}
