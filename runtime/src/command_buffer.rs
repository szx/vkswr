//! Command pools and recorded command buffers.

use crate::buffer::Buffer;
use crate::context::VulkanObject;
use crate::image::Image;
use crate::logical_device::LogicalDevice;
use crate::pipeline::{Framebuffer, Pipeline, PipelineLayout, RenderPass};
use common::graphics::{IndexBuffer, VertexBindingNumber, VertexBuffer};
use common::math::{Extent2, Extent3, Offset2, Offset3};
use gpu::{Command, RegionCopyBufferImage};
use headers::vk_decls::*;
use log::{info, trace, warn};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug)]
pub struct CommandPool {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
    logical_device: Arc<Mutex<LogicalDevice>>,
    #[allow(dead_code)]
    flags: VkCommandPoolCreateFlags,
    #[allow(dead_code)]
    queue_family_index: u32,
}

impl CommandPool {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        create_info: &VkCommandPoolCreateInfo,
    ) -> VkNonDispatchableHandle {
        info!("new CommandPool");
        Self {
            handle: VK_NULL_HANDLE,
            logical_device,
            flags: create_info.flags,
            queue_family_index: create_info.queueFamilyIndex,
        }
        .register()
    }
}

#[derive(Debug)]
pub struct CommandBuffer {
    pub(crate) handle: VkDispatchableHandle,
    #[allow(dead_code)]
    level: VkCommandBufferLevel,
    #[allow(dead_code)]
    command_pool: Arc<Mutex<CommandPool>>,
    gpu_command_buffer: gpu::CommandBuffer,
    gpu_bound_render_target_indices: Vec<gpu::RenderTargetIndex>,
}

impl CommandBuffer {
    pub fn create(allocate_info: &VkCommandBufferAllocateInfo) -> VkDispatchableHandle {
        info!("new CommandBuffer");
        let command_pool = CommandPool::from_handle(allocate_info.commandPool)
            .expect("command pool handle is live");
        Self {
            handle: VkDispatchableHandle(None),
            level: allocate_info.level,
            command_pool,
            gpu_command_buffer: gpu::CommandBuffer::new(),
            gpu_bound_render_target_indices: Vec::new(),
        }
        .register()
    }

    pub fn gpu_command_buffer_for_submit(&mut self) -> gpu::CommandBuffer {
        std::mem::take(&mut self.gpu_command_buffer)
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
        let descriptions = render_pass.lock().attachments.clone();
        let image_views = framebuffer.lock().attachments.clone();
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
        for (index, ((description, image_view), clear_value)) in descriptions
            .iter()
            .zip(image_views.iter())
            .zip(clear_values)
            .enumerate()
        {
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
                    // Keep the existing contents.
                }
                VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR => {
                    self.gpu_command_buffer.record(Command::ClearRenderTarget {
                        index,
                        render_area,
                        color: (*clear_value).into(),
                    });
                }
                VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE
                | VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_NONE_EXT => {}
                other => unreachable!("unknown attachment load op: {other:?}"),
            }

            match description.store_op {
                VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE => {
                    // The render target is memory-backed, so storing is implicit.
                }
                VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE
                | VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_NONE => {}
                other => unreachable!("unknown attachment store op: {other:?}"),
            }

            match description.stencil_load_pp {
                VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_LOAD
                | VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR => {
                    warn!("TODO: Stencil commands support");
                }
                VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE
                | VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_NONE_EXT => {}
                other => unreachable!("unknown stencil load op: {other:?}"),
            }

            match description.stencil_store_op {
                VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE => {
                    warn!("TODO: Stencil commands support");
                }
                VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE
                | VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_NONE => {}
                other => unreachable!("unknown stencil store op: {other:?}"),
            }
        }
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
        assert_eq!(
            bind_point,
            VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
            "only graphics pipelines are implemented"
        );
        pipeline.lock().bind_states(&mut self.gpu_command_buffer);
    }

    pub fn cmd_bind_descriptor_sets(
        &mut self,
        _bind_point: VkPipelineBindPoint,
        _pipeline: Arc<Mutex<PipelineLayout>>,
        _first_set: u32,
        _descriptor_sets: &[VkDescriptorSet],
        _dynamic_offsets: &[u32],
    ) {
        trace!("CommandBuffer::cmd_bind_descriptor_sets");
    }

    pub fn cmd_push_constants(
        &mut self,
        _pipeline: Arc<Mutex<PipelineLayout>>,
        _shader_stage_flags: VkShaderStageFlags,
        _offset: u32,
        _values: &[u8],
    ) {
        trace!("CommandBuffer::cmd_push_constants");
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

    pub fn cmd_set_viewport(&mut self, _first_viewport: u32, _viewports: &[VkViewport]) {
        trace!("CommandBuffer::cmd_set_viewport");
    }

    pub fn cmd_set_scissors(&mut self, _first_scissor: u32, _scissors: &[VkRect2D]) {
        trace!("CommandBuffer::cmd_set_scissors");
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
        _dst_image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        let src_buffer = src_buffer.lock();
        let dst_image = dst_image.lock();
        for region in regions {
            self.gpu_command_buffer.record(Command::CopyBufferToImage {
                src_buffer: src_buffer.descriptor(),
                dst_image: dst_image.descriptor(),
                region: image_region(region, dst_image.format.into()),
            });
        }
    }

    pub fn cmd_copy_image_to_buffer(
        &mut self,
        src_image: Arc<Mutex<Image>>,
        dst_buffer: Arc<Mutex<Buffer>>,
        _src_image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        let src_image = src_image.lock();
        let dst_buffer = dst_buffer.lock();
        for region in regions {
            self.gpu_command_buffer.record(Command::CopyImageToBuffer {
                src_image: src_image.descriptor(),
                dst_buffer: dst_buffer.descriptor(),
                region: image_region(region, src_image.format.into()),
            });
        }
    }

    pub fn cmd_copy_buffer_to_buffer(
        &mut self,
        src_buffer: Arc<Mutex<Buffer>>,
        dst_buffer: Arc<Mutex<Buffer>>,
        regions: &[VkBufferCopy],
    ) {
        for region in regions {
            self.gpu_command_buffer.record(Command::CopyBufferToBuffer {
                src_buffer: src_buffer.lock().descriptor(),
                dst_buffer: dst_buffer.lock().descriptor(),
                region: gpu::RegionCopyBufferBuffer {
                    src_offset: region.srcOffset,
                    dst_offset: region.dstOffset,
                    size: region.size,
                },
            });
        }
    }

    pub fn cmd_execute_commands(
        &mut self,
        command_buffers: impl IntoIterator<Item = Arc<Mutex<Self>>>,
    ) {
        for command_buffer in command_buffers {
            self.gpu_command_buffer.record(Command::ExecuteCommands {
                command_buffer: command_buffer.lock().gpu_command_buffer.clone(),
            });
        }
    }
}

const fn image_region(
    region: &VkBufferImageCopy,
    image_format: common::math::Format,
) -> RegionCopyBufferImage {
    RegionCopyBufferImage {
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
        image_format,
    }
}
