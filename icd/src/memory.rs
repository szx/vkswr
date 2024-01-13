//! VkDeviceMemory device commands

use headers::vk_decls::*;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::logical_device::LogicalDevice;
use runtime::memory::*;
use runtime::*;
use std::ffi::c_void;

pub unsafe extern "C" fn vkAllocateMemory(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkMemoryAllocateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMemory: Option<NonNull<VkDeviceMemory>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pAllocateInfo) = pAllocateInfo else {
        unreachable!()
    };
    let allocate_info = pAllocateInfo.as_ref();

    let _ = pAllocator;

    let Some(pMemory) = pMemory else {
        unreachable!()
    };

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
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

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
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(memory) = MemoryAllocation::from_handle(memory) else {
        unreachable!()
    };

    let Some(pData) = ppData else { unreachable!() };

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
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(memory) = MemoryAllocation::from_handle(memory) else {
        unreachable!()
    };

    memory.lock().unmap_host();
}

pub unsafe extern "C" fn vkFlushMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: Option<NonNull<VkMappedMemoryRange>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let memory_ranges = pMemoryRanges.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), memoryRangeCount as usize)
    });

    let result = device.lock().flush_memory_ranges(memory_ranges);
    result
}

pub unsafe extern "C" fn vkInvalidateMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: Option<NonNull<VkMappedMemoryRange>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let memory_ranges = pMemoryRanges.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), memoryRangeCount as usize)
    });

    let result = device.lock().invalidate_memory_ranges(memory_ranges);
    result
}
