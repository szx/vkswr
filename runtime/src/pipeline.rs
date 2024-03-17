//! Pipeline

use crate::command_buffer::CommandBuffer;
use crate::context::NonDispatchable;
use crate::image::ImageView;
use crate::logical_device::LogicalDevice;
use common::graphics::VertexInputState;
use gpu::{Command, InputAssemblyState, RasterizationState, ViewportState};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use shader::glsl::ShaderState;
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
    pub(crate) attachments: Arc<[AttachmentDescription]>,
    // TODO: dependencies: Arc<[VkSubpassDependency]>,
    subpasses: Arc<[SubpassDescription]>,
}

impl RenderPass {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        attachments: &[AttachmentDescription],
        dependencies: &[VkSubpassDependency],
        subpasses: &[SubpassDescription],
    ) -> VkNonDispatchableHandle {
        info!("new RenderPass");
        let handle = VK_NULL_HANDLE;
        let _ = dependencies;

        let object = Self {
            handle,
            logical_device,
            attachments: attachments.into(),
            subpasses: subpasses.into(),
        };
        object.register_object()
    }
}

#[derive(Debug, Clone)]
pub struct AttachmentDescription {
    pub flags: VkAttachmentDescriptionFlagBits,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub load_op: VkAttachmentLoadOp,
    pub store_op: VkAttachmentStoreOp,
    pub stencil_load_pp: VkAttachmentLoadOp,
    pub stencil_store_op: VkAttachmentStoreOp,
    pub initial_layout: VkImageLayout,
    pub final_layout: VkImageLayout,
}

#[derive(Debug, Clone)]
pub struct SubpassDescription {
    pub flags: VkSubpassDescriptionFlagBits,
    pub pipeline_bind_point: VkPipelineBindPoint,
    pub input_attachments: Arc<[VkAttachmentReference]>,
    pub color_attachments: Arc<[VkAttachmentReference]>,
    pub resolve_attachments: Arc<[VkAttachmentReference]>,
    pub depth_stencil_attachment: Option<VkAttachmentReference>,
    pub preserve_attachments: Arc<[u32]>,
}

#[derive(Debug)]
pub struct ShaderModule {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    pub(crate) code: Vec<u32>,
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
    pub handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    pub pipeline_cache: Option<Arc<Mutex<PipelineCache>>>,

    pub shader_state: ShaderState,
    pub vertex_input_state: VertexInputState,
    pub input_assembly_state: InputAssemblyState,
    pub viewport_state: ViewportState,
    pub rasterization_state: RasterizationState,
}

impl Pipeline {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        pipeline_cache: Option<Arc<Mutex<PipelineCache>>>,
        shader_state: ShaderState,
        vertex_input_state: Option<VertexInputState>,
        input_assembly_state: Option<InputAssemblyState>,
        viewport_state: Option<ViewportState>,
        rasterization_state: Option<RasterizationState>,
    ) -> VkNonDispatchableHandle {
        info!("new Pipeline");
        let handle = VK_NULL_HANDLE;

        let object = Self {
            handle,
            logical_device,
            pipeline_cache,
            shader_state,
            vertex_input_state: vertex_input_state.unwrap_or_default(),
            input_assembly_state: input_assembly_state.unwrap_or_default(),
            viewport_state: viewport_state.unwrap_or_default(),
            rasterization_state: rasterization_state.unwrap_or_default(),
        };
        object.register_object()
    }

    pub fn bind_states(&self, command_buffer: &mut gpu::CommandBuffer) {
        command_buffer.record(Command::SetShaderState {
            shader_state: self.shader_state.clone(),
        });
        command_buffer.record(Command::SetVertexInputState {
            vertex_input_state: self.vertex_input_state.clone(),
        });
        command_buffer.record(Command::SetInputAssemblyState {
            input_assembly_state: self.input_assembly_state.clone(),
        });
        command_buffer.record(Command::SetViewportState {
            viewport_state: self.viewport_state.clone(),
        });
        command_buffer.record(Command::SetRasterizationState {
            rasterization_state: self.rasterization_state.clone(),
        });
        warn!("TODO: Record rest of pipeline state");
    }
}

#[derive(Debug)]
pub struct Framebuffer {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    flags: VkFramebufferCreateFlagBits,
    width: u32,
    height: u32,
    layers: u32,
    pub(crate) attachments: Arc<[Arc<Mutex<ImageView>>]>,
    render_pass: Arc<Mutex<RenderPass>>,
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
        assert_eq!(layers, 1);

        let object = Self {
            handle,
            logical_device,
            flags: flags.into(),
            width,
            height,
            layers,
            attachments: attachments.into(),
            render_pass,
        };
        object.register_object()
    }
}
