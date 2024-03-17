//! VkBuffer device commands

use headers::vk_decls::*;
use runtime::buffer::*;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::logical_device::LogicalDevice;
use runtime::memory::MemoryAllocation;


pub unsafe extern "C" fn vkCreateBuffer(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pBuffer: Option<NonNull<VkBuffer>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let Some(pBuffer) = pBuffer else {
        unreachable!()
    };

    *pBuffer.as_ptr() = Buffer::create(
        device,
        create_info.size,
        create_info.usage,
        create_info.flags,
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyBuffer(
    device: VkDevice,
    buffer: VkBuffer,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    Buffer::drop_handle(buffer);
}

pub unsafe extern "C" fn vkGetBufferMemoryRequirements(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(buffer) = Buffer::from_handle(buffer) else {
        unreachable!()
    };

    let Some(pMemoryRequirements) = pMemoryRequirements else {
        unreachable!()
    };

    *pMemoryRequirements.as_ptr() = buffer.lock().memory_requirements();
}

pub unsafe extern "C" fn vkBindBufferMemory(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(buffer) = Buffer::from_handle(buffer) else {
        unreachable!()
    };

    let Some(memory) = MemoryAllocation::from_handle(memory) else {
        unreachable!()
    };

    let result = buffer.lock().bind_memory(memory, memoryOffset);
    result
}

pub unsafe extern "C" fn vkCreateBufferView(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferViewCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pView: Option<NonNull<VkBufferView>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    let Some(buffer) = Buffer::from_handle(create_info.buffer) else {
        unreachable!()
    };

    let _ = pAllocator;

    let Some(pView) = pView else { unreachable!() };

    *pView.as_ptr() = BufferView::create(
        device,
        buffer,
        create_info.format,
        create_info.offset,
        create_info.range,
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyBufferView(
    device: VkDevice,
    bufferView: VkBufferView,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    BufferView::drop_handle(bufferView);
}
