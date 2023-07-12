//! VkDeviceMemory device commands

use headers::vk_decls::*;
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

    *pMemory.as_ptr() = DeviceMemory::create(
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
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    DeviceMemory::drop_handle(memory);
}

pub unsafe extern "C" fn vkMapMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    ppData: Option<NonNull<NonNull<std::ffi::c_void>>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(memory) = DeviceMemory::from_handle(memory) else {
        unreachable!()
    };

    let Some(pData) = ppData else { unreachable!() };

    let result = match memory.lock().map_host() {
        Ok(ptr) => {
            *pData.as_ptr() = ptr;
            VkResult::VK_SUCCESS
        }
        Err(e) => e,
    };
    result
}

pub unsafe extern "C" fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(memory) = DeviceMemory::from_handle(memory) else {
        unreachable!()
    };

    memory.lock().unmap_host();
}
