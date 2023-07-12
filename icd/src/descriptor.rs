//! VkDescriptorSets device commands

use headers::vk_decls::*;
use runtime::descriptor::*;
use runtime::*;

pub unsafe extern "C" fn vkCreateDescriptorSetLayout(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorSetLayoutCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSetLayout: Option<NonNull<VkDescriptorSetLayout>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    let Some(bindings) = create_info.pBindings else {
        unreachable!()
    };
    let bindings = std::slice::from_raw_parts(bindings.as_ptr(), create_info.bindingCount as usize);

    let _ = pAllocator;

    let Some(pSetLayout) = pSetLayout else {
        unreachable!()
    };

    *pSetLayout.as_ptr() = DescriptorSetLayout::create(device, create_info.flags, bindings);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyDescriptorSetLayout(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    DescriptorSetLayout::drop_handle(descriptorSetLayout);
}
