//! VkPipeline device commands

use headers::vk_decls::*;
use log::warn;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::image::ImageView;
use runtime::logical_device::LogicalDevice;
use runtime::physical_device::PhysicalDevice;
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
    let Some(_device) = LogicalDevice::from_handle(device) else {
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
        .map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), create_info.attachmentCount as usize)
        })
        .iter()
        .map(|x| AttachmentDescription {
            flags: x.flags.into(),
            format: x.format,
            samples: x.samples,
            load_op: x.loadOp,
            store_op: x.storeOp,
            stencil_load_pp: x.stencilLoadOp,
            stencil_store_op: x.stencilStoreOp,
            initial_layout: x.initialLayout,
            final_layout: x.finalLayout,
        })
        .collect::<Vec<_>>();
    let attachments = &attachments[..];

    let dependencies = create_info.pDependencies.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), create_info.dependencyCount as usize)
    });

    let subpasses = create_info
        .pSubpasses
        .map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), create_info.subpassCount as usize)
        })
        .iter()
        .map(|vk| SubpassDescription {
            flags: vk.flags.into(),
            pipeline_bind_point: vk.pipelineBindPoint,
            input_attachments: vk
                .pInputAttachments
                .map_or(&[] as &[_], |x| {
                    std::slice::from_raw_parts(x.as_ptr(), vk.inputAttachmentCount as usize)
                })
                .into(),
            color_attachments: vk
                .pColorAttachments
                .map_or(&[] as &[_], |x| {
                    std::slice::from_raw_parts(x.as_ptr(), vk.colorAttachmentCount as usize)
                })
                .into(),
            resolve_attachments: vk
                .pResolveAttachments
                .map_or(&[] as &[_], |x| {
                    std::slice::from_raw_parts(x.as_ptr(), vk.colorAttachmentCount as usize)
                })
                .into(),
            depth_stencil_attachment: vk.pDepthStencilAttachment.map(|x| *x.as_ptr()),
            preserve_attachments: vk
                .pPreserveAttachments
                .map_or(&[] as &[_], |x| {
                    std::slice::from_raw_parts(x.as_ptr(), vk.preserveAttachmentCount as usize)
                })
                .into(),
        })
        .collect::<Vec<_>>();
    let subpasses = &subpasses[..];

    let _ = pAllocator;

    let Some(pRenderPass) = pRenderPass else {
        unreachable!()
    };

    *pRenderPass.as_ptr() = RenderPass::create(device, attachments, dependencies, subpasses);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyRenderPass(
    device: VkDevice,
    renderPass: VkRenderPass,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
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
    assert_eq!(create_info.codeSize % 4, 0);
    let code_size = create_info.codeSize / 4;
    let code = std::slice::from_raw_parts(code.as_ptr(), code_size as usize);

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
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    ShaderModule::drop_handle(shaderModule);
}

pub unsafe extern "C" fn vkCreatePipelineCache(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkPipelineCacheCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelineCache: Option<NonNull<VkPipelineCache>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    let initial_data = create_info.pInitialData.map_or(&[] as &[u8], |x| {
        std::slice::from_raw_parts(x.as_ptr() as *mut u8, create_info.initialDataSize as usize)
    });

    let _ = pAllocator;

    let Some(pPipelineCache) = pPipelineCache else {
        unreachable!()
    };

    *pPipelineCache.as_ptr() = PipelineCache::create(device, create_info.flags, initial_data);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyPipelineCache(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    PipelineCache::drop_handle(pipelineCache);
}

pub unsafe extern "C" fn vkCreateGraphicsPipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: Option<NonNull<VkGraphicsPipelineCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelines: Option<NonNull<VkPipeline>>,
) -> VkResult {
    let mut result = VkResult::VK_SUCCESS;

    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let pipelineCache = PipelineCache::from_handle(pipelineCache);

    let _ = pAllocator;

    let Some(pPipelines) = pPipelines else {
        unreachable!()
    };
    let pipelines = std::slice::from_raw_parts_mut(pPipelines.as_ptr(), createInfoCount as usize);

    let Some(pCreateInfos) = pCreateInfos else {
        unreachable!()
    };
    let create_infos = std::slice::from_raw_parts(pCreateInfos.as_ptr(), createInfoCount as usize);

    for (create_info, pipeline) in std::iter::zip(create_infos, pipelines) {
        let shader_stages = create_info
            .pStages
            .map_or(&[] as &[VkPipelineShaderStageCreateInfo], |x| {
                std::slice::from_raw_parts(x.as_ptr(), create_info.stageCount as usize)
            });
        let shader_state = match PhysicalDevice::parse_shader_stages(shader_stages) {
            Ok(inner) => inner,
            Err(err) => {
                result = err;
                continue;
            }
        };
        let vertex_input_state = create_info
            .pVertexInputState
            .map(|x| PhysicalDevice::parse_vertex_input_state(*x.as_ref()));
        let input_assembly_state = create_info
            .pInputAssemblyState
            .map(|x| PhysicalDevice::parse_input_assembly_state(*x.as_ref()));
        warn!("TODO: Parse rest of Vulkan pipeline states");
        let tessellation_state = create_info.pTessellationState.map(|x| x.as_ref());
        let viewport_state = create_info
            .pViewportState
            .map(|x| PhysicalDevice::parse_viewport_state(*x.as_ref()));
        let rasterization_state = create_info
            .pRasterizationState
            .map(|x| PhysicalDevice::parse_rasterization_state(*x.as_ref()));
        let multisample_state = create_info.pMultisampleState.map(|x| x.as_ref());
        let depth_stencil_state = create_info.pDepthStencilState.map(|x| x.as_ref());
        let color_blend_state = create_info.pColorBlendState.map(|x| x.as_ref());
        let dynamic_state = create_info.pDynamicState.map(|x| x.as_ref());
        *pipeline = Pipeline::create(
            device.clone(),
            pipelineCache.clone(),
            shader_state,
            vertex_input_state,
            input_assembly_state,
            viewport_state,
            rasterization_state,
        );
    }

    result
}

pub unsafe extern "C" fn vkDestroyPipeline(
    device: VkDevice,
    pipeline: VkPipeline,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    Pipeline::drop_handle(pipeline);
}

pub unsafe extern "C" fn vkCreateFramebuffer(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkFramebufferCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFramebuffer: Option<NonNull<VkFramebuffer>>,
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
        .map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), create_info.attachmentCount as usize)
        })
        .iter()
        .flat_map(|&handle| ImageView::from_handle(handle))
        .collect::<Vec<_>>();
    let Some(render_pass) = RenderPass::from_handle(create_info.renderPass) else {
        unreachable!()
    };

    let _ = pAllocator;

    let Some(pFramebuffer) = pFramebuffer else {
        unreachable!()
    };

    *pFramebuffer.as_ptr() = Framebuffer::create(
        device,
        create_info.flags,
        create_info.width,
        create_info.height,
        create_info.layers,
        attachments,
        render_pass,
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyFramebuffer(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(_device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let _ = pAllocator;

    Framebuffer::drop_handle(framebuffer);
}
