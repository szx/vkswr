//! VkDeviceMemory device commands

use headers::vk_decls::*;
use runtime::context::VulkanObject;
use runtime::logical_device::LogicalDevice;
use runtime::memory::*;

pub unsafe extern "C" fn vkAllocateMemory(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkMemoryAllocateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMemory: Option<NonNull<VkDeviceMemory>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pAllocateInfo = pAllocateInfo.expect("Vulkan guarantees a non-null pointer");
    let allocate_info = pAllocateInfo.as_ref();

    let _ = pAllocator;

    let pMemory = pMemory.expect("Vulkan guarantees a non-null pointer");

    *pMemory.as_ptr() = MemoryAllocation::create(
        device,
        allocate_info.allocationSize,
        allocate_info.memoryTypeIndex,
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkFreeMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    MemoryAllocation::drop_handle(memory);
}

pub unsafe extern "C" fn vkMapMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    _flags: VkMemoryMapFlags,
    ppData: Option<NonNull<NonNull<std::ffi::c_void>>>,
) -> VkResult {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let memory =
        MemoryAllocation::from_handle(memory).expect("Vulkan handle refers to a live object");

    let pData = ppData.expect("Vulkan guarantees a non-null pointer");

    let mapped_memory = memory.lock().map_host(offset, size);
    match mapped_memory {
        Ok(ptr) => {
            *pData.as_ptr() = ptr;
            VkResult::VK_SUCCESS
        }
        Err(e) => e,
    }
}

pub unsafe extern "C" fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) {
    let _device =
        LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let memory =
        MemoryAllocation::from_handle(memory).expect("Vulkan handle refers to a live object");

    memory.lock().unmap_host();
}

pub unsafe extern "C" fn vkFlushMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: Option<NonNull<VkMappedMemoryRange>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let memory_ranges = pMemoryRanges.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), memoryRangeCount as usize)
    });

    device.lock().flush_memory_ranges(memory_ranges)
}

pub unsafe extern "C" fn vkInvalidateMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: Option<NonNull<VkMappedMemoryRange>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let memory_ranges = pMemoryRanges.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), memoryRangeCount as usize)
    });

    device.lock().invalidate_memory_ranges(memory_ranges)
}
