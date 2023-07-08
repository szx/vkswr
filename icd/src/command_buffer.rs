//! VkCommandBuffer device commands

use headers::vk_decls::*;
use runtime::command_buffer::*;
use runtime::*;

pub unsafe extern "C" fn vkCreateCommandPool(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkCommandPoolCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pCommandPool: Option<NonNull<VkCommandPool>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let Some(pCommandPool) = pCommandPool else { unreachable!() };

    *pCommandPool.as_ptr() = CommandPool::create(device, create_info);

    VkResult::VK_SUCCESS
}
