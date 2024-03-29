//! Image

use crate::buffer::Buffer;
use crate::context::{Dispatchable, NonDispatchable};
use crate::image::Image;
use crate::logical_device::LogicalDevice;
use crate::pipeline::{Framebuffer, Pipeline, PipelineLayout, RenderPass};
use common::graphics::{IndexBuffer, VertexBindingNumber, VertexBuffer};
use common::math::{Extent2, Extent3, Offset2, Offset3};
use gpu::{Command, RegionCopyBufferImage};
use headers::vk_decls::*;
use itertools::izip;
use log::*;
use parking_lot::Mutex;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct CommandBuffer {
    pub(crate) handle: VkDispatchableHandle,
    level: VkCommandBufferLevel,
    command_pool: Arc<Mutex<CommandPool>>,
    gpu_command_buffer: gpu::CommandBuffer,
    gpu_bound_render_target_indices: Vec<gpu::RenderTargetIndex>,
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
            gpu_command_buffer: gpu::CommandBuffer::new(),
            gpu_bound_render_target_indices: vec![],
        };
        object.register_object()
    }

    pub fn gpu_command_buffer_for_submit(&mut self) -> gpu::CommandBuffer {
        std::mem::replace(&mut self.gpu_command_buffer, gpu::CommandBuffer::new())
    }

    pub fn begin(&mut self) {
        warn!("TODO: Start recording command buffer");
    }

    pub fn end(&mut self) {
        warn!("TODO: Stop recording command buffer");
    }

    pub fn cmd_pipeline_barrier(&mut self) {
        warn!("TODO: Record pipeline barrier");
    }

    pub fn cmd_begin_render_pass(
        &mut self,
        render_pass: Arc<Mutex<RenderPass>>,
        framebuffer: Arc<Mutex<Framebuffer>>,
        render_area: VkRect2D,
        clear_values: &[VkClearValue],
        contents: VkSubpassContents,
    ) {
        let render_pass = render_pass.lock();
        let descriptions = render_pass.attachments.clone();
        drop(render_pass);
        let framebuffer = framebuffer.lock();
        let image_views = framebuffer.attachments.clone();
        drop(framebuffer);
        let _ = contents;

        let render_area = gpu::RenderArea {
            extent: Extent2::<u32> {
                width: render_area.extent.width,
                height: render_area.extent.height,
            },
            offset: Offset2::<i32> {
                x: render_area.offset.x,
                y: render_area.offset.y,
            },
        };

        assert!(self.gpu_bound_render_target_indices.is_empty());
        izip!(descriptions.iter(), image_views.iter(), clear_values.iter())
            .enumerate()
            .for_each(|(index, (description, image_view, clear_value))| {
                let index = gpu::RenderTargetIndex(index);
                self.gpu_bound_render_target_indices.push(index);

                self.gpu_command_buffer.record(Command::BindRenderTarget {
                    render_target: gpu::RenderTarget {
                        index,
                        format: description.format.into(),
                        samples: description.samples.into(),
                        image: image_view.lock().image.lock().descriptor(),
                    },
                });

                match description.load_op {
                    VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_LOAD => {
                        // No-op.
                    }
                    VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR => {
                        self.gpu_command_buffer.record(Command::ClearRenderTarget {
                            index,
                            render_area,
                            color: (*clear_value).into(),
                        });
                    }
                    VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE
                    | VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_NONE_EXT => {
                        // No-op.
                    }
                    _ => unreachable!(),
                }

                match description.store_op {
                    VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE => {
                        // No-op.
                    }
                    VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE
                    | VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_NONE => {
                        // No-op.
                    }
                    _ => unreachable!(),
                };

                match description.stencil_load_pp {
                    VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_LOAD => {
                        warn!("TODO: Stencil commands support");
                    }
                    VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR => {
                        warn!("TODO: Stencil commands support");
                    }
                    VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE
                    | VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_NONE_EXT => {
                        // No-op.
                    }
                    _ => unreachable!(),
                };

                match description.stencil_store_op {
                    VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE => {
                        warn!("TODO: Stencil commands support");
                    }
                    VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE
                    | VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_NONE => {
                        // No-op.
                    }
                    _ => unreachable!(),
                };
            });
    }

    pub fn cmd_end_render_pass(&mut self) {
        for index in self.gpu_bound_render_target_indices.drain(..) {
            self.gpu_command_buffer
                .record(Command::UnbindRenderTarget { index });
        }
    }

    pub fn cmd_bind_pipeline(
        &mut self,
        bind_point: VkPipelineBindPoint,
        pipeline: Arc<Mutex<Pipeline>>,
    ) {
        if bind_point == VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS {
            pipeline.lock().bind_states(&mut self.gpu_command_buffer);
        } else {
            unreachable!();
        }
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

    pub fn cmd_push_constants(
        &mut self,
        pipeline: Arc<Mutex<PipelineLayout>>,
        shader_stage_flags: VkShaderStageFlags,
        offset: u32,
        values: &[u8],
    ) {
        trace!("CommandBuffer::cmd_push_constants");
        let _ = pipeline;
        let _ = shader_stage_flags;
        let _ = offset;
        let _ = values;
        // TODO: Record push constant update.
    }

    pub fn cmd_bind_vertex_buffer(
        &mut self,
        binding: u32,
        buffer: Arc<Mutex<Buffer>>,
        offset: VkDeviceSize,
    ) {
        self.gpu_command_buffer.record(Command::BindVertexBuffer {
            vertex_buffer: VertexBuffer {
                binding_number: VertexBindingNumber(binding),
                buffer: buffer.lock().descriptor(),
                offset,
            },
        });
    }

    pub fn cmd_bind_index_buffer(
        &mut self,
        buffer: Arc<Mutex<Buffer>>,
        offset: VkDeviceSize,
        index_size: u8,
    ) {
        self.gpu_command_buffer.record(Command::BindIndexBuffer {
            index_buffer: IndexBuffer {
                buffer: buffer.lock().descriptor(),
                offset,
                index_size,
            },
        });
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
        self.gpu_command_buffer.record(Command::DrawPrimitive {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        });
    }

    pub fn cmd_draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        self.gpu_command_buffer
            .record(Command::DrawPrimitiveIndexed {
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            });
    }

    pub fn cmd_copy_buffer_to_image(
        &mut self,
        src_buffer: Arc<Mutex<Buffer>>,
        dst_image: Arc<Mutex<Image>>,
        dst_image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        let _ = dst_image_layout;
        let src_buffer = src_buffer.lock();
        let dst_image = dst_image.lock();
        for region in regions {
            self.gpu_command_buffer.record(Command::CopyBufferToImage {
                src_buffer: src_buffer.descriptor(),
                dst_image: dst_image.descriptor(),
                region: RegionCopyBufferImage {
                    buffer_offset: region.bufferOffset,
                    buffer_row_len: region.bufferRowLength,
                    buffer_image_height: region.bufferImageHeight,
                    image_mip_level: region.imageSubresource.mipLevel,
                    image_base_array_level: region.imageSubresource.baseArrayLayer,
                    image_array_level_count: region.imageSubresource.layerCount,
                    image_offset: Offset3::<i32> {
                        x: region.imageOffset.x,
                        y: region.imageOffset.y,
                        z: region.imageOffset.z,
                    },
                    image_extent: Extent3::<u32> {
                        width: region.imageExtent.width,
                        height: region.imageExtent.height,
                        depth: region.imageExtent.depth,
                    },
                    image_format: dst_image.format.into(),
                },
            })
        }
    }

    pub fn cmd_copy_image_to_buffer(
        &mut self,
        src_image: Arc<Mutex<Image>>,
        dst_buffer: Arc<Mutex<Buffer>>,
        src_image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        let _ = src_image_layout;
        let src_image = src_image.lock();
        let dst_buffer = dst_buffer.lock();
        for region in regions {
            self.gpu_command_buffer.record(Command::CopyImageToBuffer {
                src_image: src_image.descriptor(),
                dst_buffer: dst_buffer.descriptor(),
                region: RegionCopyBufferImage {
                    buffer_offset: region.bufferOffset,
                    buffer_row_len: region.bufferRowLength,
                    buffer_image_height: region.bufferImageHeight,
                    image_mip_level: region.imageSubresource.mipLevel,
                    image_base_array_level: region.imageSubresource.baseArrayLayer,
                    image_array_level_count: region.imageSubresource.layerCount,
                    image_offset: Offset3::<i32> {
                        x: region.imageOffset.x,
                        y: region.imageOffset.y,
                        z: region.imageOffset.z,
                    },
                    image_extent: Extent3::<u32> {
                        width: region.imageExtent.width,
                        height: region.imageExtent.height,
                        depth: region.imageExtent.depth,
                    },
                    image_format: src_image.format.into(),
                },
            })
        }
    }

    pub fn cmd_copy_buffer_to_buffer(
        &mut self,
        src_buffer: Arc<Mutex<Buffer>>,
        dst_buffer: Arc<Mutex<Buffer>>,
        regions: &[VkBufferCopy],
    ) {
        let _ = regions;
        for region in regions {
            self.gpu_command_buffer.record(Command::CopyBufferToBuffer {
                src_buffer: src_buffer.lock().descriptor(),
                dst_buffer: dst_buffer.lock().descriptor(),
                region: gpu::RegionCopyBufferBuffer {
                    src_offset: region.srcOffset,
                    dst_offset: region.dstOffset,
                    size: region.size,
                },
            })
        }
    }

    pub fn cmd_execute_commands(
        &mut self,
        command_buffers: impl IntoIterator<Item = Arc<Mutex<Self>>>,
    ) {
        for command_buffer in command_buffers {
            self.gpu_command_buffer.record(Command::ExecuteCommands {
                command_buffer: command_buffer.lock().gpu_command_buffer.clone(),
            })
        }
    }
}
