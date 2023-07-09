//! VkDeviceMemory device commands

use headers::vk_decls::*;
use runtime::memory::*;
use runtime::*;

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
