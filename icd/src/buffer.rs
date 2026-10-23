//! VkBuffer device commands

use headers::vk_decls::*;
use runtime::buffer::*;
use runtime::context::VulkanObject;
use runtime::logical_device::LogicalDevice;
use runtime::memory::MemoryAllocation;

pub unsafe extern "C" fn vkCreateBuffer(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pBuffer: Option<NonNull<VkBuffer>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();

    let _ = pAllocator;

    let pBuffer = pBuffer.expect("Vulkan guarantees a non-null pointer");

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
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    Buffer::drop_handle(buffer);
}

pub unsafe extern "C" fn vkGetBufferMemoryRequirements(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let buffer = Buffer::from_handle(buffer).expect("Vulkan handle refers to a live object");

    let pMemoryRequirements = pMemoryRequirements.expect("Vulkan guarantees a non-null pointer");

    *pMemoryRequirements.as_ptr() = buffer.lock().memory_requirements();
}

pub unsafe extern "C" fn vkBindBufferMemory(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let buffer = Buffer::from_handle(buffer).expect("Vulkan handle refers to a live object");

    let memory =
        MemoryAllocation::from_handle(memory).expect("Vulkan handle refers to a live object");

    buffer.lock().bind_memory(memory, memoryOffset)
}

pub unsafe extern "C" fn vkCreateBufferView(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferViewCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pView: Option<NonNull<VkBufferView>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();
    let buffer =
        Buffer::from_handle(create_info.buffer).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    let pView = pView.expect("Vulkan guarantees a non-null pointer");

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
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    BufferView::drop_handle(bufferView);
}
