//! Pipeline

use crate::image::ImageView;
use crate::memory::{DeviceMemory, MemoryBinding};
use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct PipelineLayout {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
}

impl PipelineLayout {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkDescriptorSetLayoutCreateFlags,
        set_layouts: Option<&[VkDescriptorSetLayout]>,
        push_constant_ranges: Option<&[VkPushConstantRange]>,
    ) -> VkNonDispatchableHandle {
        info!("new PipelineLayout");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let _ = set_layouts;
        let _ = push_constant_ranges;

        let object = Self {
            handle,
            logical_device,
        };
        object.register_object()
    }
}

impl NonDispatchable for PipelineLayout {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.pipeline_layouts
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.pipeline_layouts
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

#[derive(Debug)]
pub struct RenderPass {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
}

impl RenderPass {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkDescriptorSetLayoutCreateFlags,
        attachments: Option<&[VkAttachmentDescription]>,
        dependencies: Option<&[VkSubpassDependency]>,
        subpasses: Option<&[VkSubpassDescription]>,
    ) -> VkNonDispatchableHandle {
        info!("new RenderPass");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let _ = attachments;
        let _ = dependencies;
        let _ = subpasses;

        let object = Self {
            handle,
            logical_device,
        };
        object.register_object()
    }
}

impl NonDispatchable for RenderPass {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.render_passes
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.render_passes
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

#[derive(Debug)]
pub struct ShaderModule {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    code: Vec<u32>,
}

impl ShaderModule {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkDescriptorSetLayoutCreateFlags,
        code: &[u32],
    ) -> VkNonDispatchableHandle {
        info!("new ShaderModule");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let code = code.to_vec();

        let object = Self {
            handle,
            logical_device,
            code,
        };
        object.register_object()
    }
}

impl NonDispatchable for ShaderModule {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.shader_modules
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.shader_modules
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

#[derive(Debug)]
pub struct PipelineCache {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    initial_data: Vec<u8>,
}

impl PipelineCache {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkDescriptorSetLayoutCreateFlags,
        initial_data: &[u8],
    ) -> VkNonDispatchableHandle {
        info!("new PipelineCache");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let initial_data = initial_data.to_vec();

        let object = Self {
            handle,
            logical_device,
            initial_data,
        };
        object.register_object()
    }
}

impl NonDispatchable for PipelineCache {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.pipeline_caches
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.pipeline_caches
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

#[derive(Debug)]
pub struct Pipeline {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    pub pipeline_cache: Option<Arc<Mutex<PipelineCache>>>,
}

impl Pipeline {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        pipeline_cache: Option<Arc<Mutex<PipelineCache>>>,
        flags: VkDescriptorSetLayoutCreateFlags,
        stages: &[VkPipelineShaderStageCreateInfo],
        state: GraphicsPipelineStateCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new Pipeline");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let _ = stages;
        let _ = state.vertex_input_state;
        let _ = state.input_assembly_state;
        assert!(state.tessellation_state.is_none());
        let _ = state.viewport_state;
        let _ = state.rasterization_state;
        let _ = state.multisample_state;
        let _ = state.depth_stencil_state;
        let _ = state.color_blend_state;
        let _ = state.dynamic_state;

        let object = Self {
            handle,
            logical_device,
            pipeline_cache,
        };
        object.register_object()
    }
}

impl NonDispatchable for Pipeline {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.pipelines
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.pipelines
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

pub struct GraphicsPipelineStateCreateInfo<'a> {
    pub vertex_input_state: Option<&'a VkPipelineVertexInputStateCreateInfo>,
    pub input_assembly_state: Option<&'a VkPipelineInputAssemblyStateCreateInfo>,
    pub tessellation_state: Option<&'a VkPipelineTessellationStateCreateInfo>,
    pub viewport_state: Option<&'a VkPipelineViewportStateCreateInfo>,
    pub rasterization_state: Option<&'a VkPipelineRasterizationStateCreateInfo>,
    pub multisample_state: Option<&'a VkPipelineMultisampleStateCreateInfo>,
    pub depth_stencil_state: Option<&'a VkPipelineDepthStencilStateCreateInfo>,
    pub color_blend_state: Option<&'a VkPipelineColorBlendStateCreateInfo>,
    pub dynamic_state: Option<&'a VkPipelineDynamicStateCreateInfo>,
}

#[derive(Debug)]
pub struct Framebuffer {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
}

impl Framebuffer {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkFramebufferCreateFlags,
        width: u32,
        height: u32,
        layers: u32,
        attachments: Vec<Arc<Mutex<ImageView>>>,
        render_pass: Arc<Mutex<RenderPass>>,
    ) -> VkNonDispatchableHandle {
        info!("new Framebuffer");
        let handle = VK_NULL_HANDLE;

        let _ = flags;
        let _ = width;
        let _ = height;
        let _ = layers;
        let _ = attachments;
        let _ = render_pass;

        let object = Self {
            handle,
            logical_device,
        };
        object.register_object()
    }
}

impl NonDispatchable for Framebuffer {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.framebuffers
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.framebuffers
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}
