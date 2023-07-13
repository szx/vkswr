//! Pipeline

use crate::context::NonDispatchable;
use crate::image::ImageView;
use crate::logical_device::LogicalDevice;
use crate::memory::{DeviceMemory, MemoryBinding};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct PipelineLayout {
    pub(crate) handle: VkNonDispatchableHandle,
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

#[derive(Debug)]
pub struct RenderPass {
    pub(crate) handle: VkNonDispatchableHandle,
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

#[derive(Debug)]
pub struct ShaderModule {
    pub(crate) handle: VkNonDispatchableHandle,
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

#[derive(Debug)]
pub struct PipelineCache {
    pub(crate) handle: VkNonDispatchableHandle,
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

#[derive(Debug)]
pub struct Pipeline {
    pub(crate) handle: VkNonDispatchableHandle,
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
    pub(crate) handle: VkNonDispatchableHandle,
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
