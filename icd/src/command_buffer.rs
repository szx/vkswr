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

    let Some(pCommandPool) = pCommandPool else {
        unreachable!()
    };

    *pCommandPool.as_ptr() = CommandPool::create(device, create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkAllocateCommandBuffers(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkCommandBufferAllocateInfo>>,
    pCommandBuffers: Option<NonNull<VkCommandBuffer>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pAllocateInfo) = pAllocateInfo else {
        unreachable!()
    };
    let allocate_info = pAllocateInfo.as_ref();

    let Some(pCommandBuffers) = pCommandBuffers else {
        unreachable!()
    };

    let command_buffer_count = allocate_info.commandBufferCount as usize;
    let command_buffers = vec![CommandBuffer::create(allocate_info); command_buffer_count]
        .iter()
        .map(|x| *x)
        .collect::<Vec<_>>();
    std::ptr::copy_nonoverlapping(
        command_buffers.as_ptr(),
        pCommandBuffers.as_ptr(),
        command_buffer_count,
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkBeginCommandBuffer(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: Option<NonNull<VkCommandBufferBeginInfo>>,
) -> VkResult {
    let Some(commandBuffer) = CommandBuffer::from_handle(commandBuffer) else {
        unreachable!()
    };

    let Some(pBeginInfo) = pBeginInfo else {
        unreachable!()
    };
    let _ = pBeginInfo.as_ref();

    commandBuffer.lock().begin();

    VkResult::VK_SUCCESS
}
