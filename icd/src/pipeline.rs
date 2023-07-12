//! VkPipeline device commands

use headers::vk_decls::*;
use runtime::pipeline::*;
use runtime::*;

pub unsafe extern "C" fn vkCreatePipelineLayout(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkPipelineLayoutCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelineLayout: Option<NonNull<VkPipelineLayout>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    let set_layouts = create_info
        .pSetLayouts
        .map(|x| std::slice::from_raw_parts(x.as_ptr(), create_info.setLayoutCount as usize));
    let push_constant_ranges = create_info.pPushConstantRanges.map(|x| {
        std::slice::from_raw_parts(x.as_ptr(), create_info.pushConstantRangeCount as usize)
    });

    let _ = pAllocator;

    let Some(pPipelineLayout) = pPipelineLayout else {
        unreachable!()
    };

    *pPipelineLayout.as_ptr() =
        PipelineLayout::create(device, create_info.flags, set_layouts, push_constant_ranges);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyPipelineLayout(
    device: VkDevice,
    pipelineLayout: VkPipelineLayout,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    PipelineLayout::drop_handle(pipelineLayout);
}
