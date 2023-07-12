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

pub unsafe extern "C" fn vkCreateRenderPass(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkRenderPassCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pRenderPass: Option<NonNull<VkRenderPass>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    let attachments = create_info
        .pAttachments
        .map(|x| std::slice::from_raw_parts(x.as_ptr(), create_info.attachmentCount as usize));
    let dependencies = create_info
        .pDependencies
        .map(|x| std::slice::from_raw_parts(x.as_ptr(), create_info.dependencyCount as usize));
    let subpasses = create_info
        .pSubpasses
        .map(|x| std::slice::from_raw_parts(x.as_ptr(), create_info.subpassCount as usize));

    let _ = pAllocator;

    let Some(pRenderPass) = pRenderPass else {
        unreachable!()
    };

    *pRenderPass.as_ptr() = RenderPass::create(
        device,
        create_info.flags,
        attachments,
        dependencies,
        subpasses,
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyRenderPass(
    device: VkDevice,
    renderPass: VkRenderPass,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    RenderPass::drop_handle(renderPass);
}

pub unsafe extern "C" fn vkCreateShaderModule(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkShaderModuleCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pShaderModule: Option<NonNull<VkShaderModule>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    let Some(code) = create_info.pCode else {
        unreachable!()
    };
    let code = std::slice::from_raw_parts(code.as_ptr(), create_info.codeSize as usize);

    let _ = pAllocator;

    let Some(pShaderModule) = pShaderModule else {
        unreachable!()
    };

    *pShaderModule.as_ptr() = ShaderModule::create(device, create_info.flags, code);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyShaderModule(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    ShaderModule::drop_handle(shaderModule);
}
