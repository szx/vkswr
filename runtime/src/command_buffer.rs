//! Image

use crate::buffer::Buffer;
use crate::context::{Dispatchable, NonDispatchable};
use crate::image::Image;
use crate::logical_device::LogicalDevice;
use crate::pipeline::{Framebuffer, Pipeline, PipelineLayout, RenderPass};
use headers::vk_decls::*;
use log::*;
use parking_lot::{Mutex, RwLockWriteGuard};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct CommandPool {
    pub(crate) handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    flags: VkCommandPoolCreateFlags,
    queue_family_index: u32,
}

impl CommandPool {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        create_info: &VkCommandPoolCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new CommandPool");
        let handle = VK_NULL_HANDLE;
        let flags = create_info.flags;
        let queue_family_index = create_info.queueFamilyIndex;

        let command_pool = Self {
            handle,
            logical_device,
            flags,
            queue_family_index,
        };
        command_pool.register_object()
    }
}

#[derive(Debug)]
pub struct CommandBuffer {
    pub(crate) handle: VkDispatchableHandle,
    level: VkCommandBufferLevel,
    command_pool: Arc<Mutex<CommandPool>>,
}

impl CommandBuffer {
    pub fn create(allocate_info: &VkCommandBufferAllocateInfo) -> VkDispatchableHandle {
        info!("new CommandBuffer");
        let handle = VkDispatchableHandle(None);
        let level = allocate_info.level;
        let Some(command_pool) = CommandPool::from_handle(allocate_info.commandPool) else {
            unreachable!()
        };

        let object = Self {
            handle,
            level,
            command_pool,
        };
        object.register_object()
    }

    pub fn begin(&mut self) {
        trace!("CommandBuffer::begin");
        // TODO: Start recording command buffer.
    }

    pub fn end(&mut self) {
        trace!("CommandBuffer::end");
        // TODO: Finish recording command buffer.
    }

    pub fn cmd_pipeline_barrier(&mut self) {
        trace!("CommandBuffer::cmd_pipeline_barrier");
        // TODO: Record pipeline barrier.
    }

    pub fn cmd_begin_render_pass(
        &mut self,
        render_pass: Arc<Mutex<RenderPass>>,
        framebuffer: Arc<Mutex<Framebuffer>>,
        render_area: VkRect2D,
        clear_values: &[VkClearValue],
        contents: VkSubpassContents,
    ) {
        trace!("CommandBuffer::cmd_begin_render_pass");
        let _ = render_pass;
        let _ = framebuffer;
        let _ = render_area;
        let _ = clear_values;
        let _ = contents;
        // TODO: Record render pass start.
    }

    pub fn cmd_end_render_pass(&mut self) {
        trace!("CommandBuffer::cmd_end_render_pass");
        // TODO: Record render pass finish.
    }

    pub fn cmd_bind_pipeline(
        &mut self,
        bind_point: VkPipelineBindPoint,
        pipeline: Arc<Mutex<Pipeline>>,
    ) {
        trace!("CommandBuffer::cmd_bind_pipeline");
        let _ = bind_point;
        let _ = pipeline;
        // TODO: Record pipeline binding.
    }

    pub fn cmd_bind_descriptor_sets(
        &mut self,
        bind_point: VkPipelineBindPoint,
        pipeline: Arc<Mutex<PipelineLayout>>,
        first_set: u32,
        descriptor_sets: &[VkDescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        trace!("CommandBuffer::cmd_bind_descriptor_sets");
        let _ = bind_point;
        let _ = pipeline;
        let _ = first_set;
        let _ = descriptor_sets;
        let _ = dynamic_offsets;
        // TODO: Record descriptor sets bindings.
    }

    pub fn cmd_bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        buffers: &[VkBuffer],
        offsets: &[VkDeviceSize],
    ) {
        trace!("CommandBuffer::cmd_bind_vertex_buffers");
        let _ = first_binding;
        let _ = buffers;
        let _ = offsets;
        // TODO: Record vertex buffers bindings.
    }

    pub fn cmd_set_viewport(&mut self, first_viewport: u32, viewports: &[VkViewport]) {
        trace!("CommandBuffer::cmd_set_viewport");
        let _ = first_viewport;
        let _ = viewports;
        // TODO: Record viewport dynamic state change.
    }

    pub fn cmd_set_scissors(&mut self, first_scissor: u32, scissors: &[VkRect2D]) {
        trace!("CommandBuffer::cmd_set_scissors");
        let _ = first_scissor;
        let _ = scissors;
        // TODO: Record scissors dynamic state change.
    }

    pub fn cmd_draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        trace!("CommandBuffer::cmd_draw");
        let _ = vertex_count;
        let _ = instance_count;
        let _ = first_vertex;
        let _ = first_instance;
        // TODO: Record draw.
    }

    pub fn cmd_copy_buffer_to_image(
        &mut self,
        src_buffer: Arc<Mutex<Buffer>>,
        dst_image: Arc<Mutex<Image>>,
        dst_image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        trace!("CommandBuffer::cmd_copy_buffer_to_image");
        let _ = src_buffer;
        let _ = dst_image;
        let _ = dst_image_layout;
        let _ = regions;
        // TODO: Record buffer to image copy.
    }

    pub fn cmd_copy_image_to_buffer(
        &mut self,
        src_image: Arc<Mutex<Image>>,
        dst_buffer: Arc<Mutex<Buffer>>,
        src_image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        trace!("CommandBuffer::cmd_copy_image_to_buffer");
        let _ = src_image;
        let _ = dst_buffer;
        let _ = src_image_layout;
        let _ = regions;
        // TODO: Record image to buffer copy.
    }

    pub fn cmd_copy_buffer_to_buffer(
        &mut self,
        src_buffer: Arc<Mutex<Buffer>>,
        dst_buffer: Arc<Mutex<Buffer>>,
        regions: &[VkBufferCopy],
    ) {
        trace!("CommandBuffer::cmd_copy_buffer_to_buffer");
        let _ = src_buffer;
        let _ = dst_buffer;
        let _ = regions;
        // TODO: Record buffer to buffer copy.
    }

    pub fn cmd_execute_commands(
        &mut self,
        command_buffers: impl IntoIterator<Item = Arc<Mutex<CommandBuffer>>>,
    ) {
        trace!("CommandBuffer::cmd_execute_commands");
        let _ = command_buffers;
        // TODO: Record execute commands.
    }
}
