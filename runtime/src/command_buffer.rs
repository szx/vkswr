//! Image

use crate::buffer::Buffer;
use crate::context::{Dispatchable, NonDispatchable};
use crate::image::Image;
use crate::logical_device::LogicalDevice;
use crate::pipeline::{Framebuffer, Pipeline, PipelineLayout, RenderPass};
use headers::vk_decls::*;
use itertools::izip;
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
        let render_pass = render_pass.lock();
        let descriptions = render_pass.attachments.clone();
        drop(render_pass);
        let framebuffer = framebuffer.lock();
        let image_views = framebuffer.attachments.clone();
        drop(framebuffer);
        let _ = contents;

        let render_area = gpu::RenderArea {
            extent: gpu::Extent2d {
                width: render_area.extent.width,
                height: render_area.extent.height,
            },
            offset: gpu::Offset2d {
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

                self.gpu_command_buffer
                    .record(gpu::Command::BindRenderTarget {
                        render_target: gpu::RenderTarget {
                            index,
                            format: description.format.into(),
                            samples: description.samples.into(),
                            image: image_view.lock().image.lock().gpu_image(),
                        },
                    });

                match description.load_op {
                    VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_LOAD => {
                        // No-op.
                    }
                    VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR => {
                        self.gpu_command_buffer
                            .record(gpu::Command::ClearRenderTarget {
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
                // TODO: Stencil commands support.
            });
    }

    pub fn cmd_end_render_pass(&mut self) {
        for index in self.gpu_bound_render_target_indices.drain(..) {
            self.gpu_command_buffer
                .record(gpu::Command::UnbindRenderTarget { index });
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
        let _ = dst_image_layout;
        let src_buffer = src_buffer.lock();
        let dst_image = dst_image.lock();
        for region in regions {
            self.gpu_command_buffer
                .record(gpu::Command::CopyBufferToImage {
                    src_buffer: src_buffer.gpu_buffer(),
                    dst_image: dst_image.gpu_image(),
                    region: gpu::RegionCopyBufferImage {
                        buffer_offset: region.bufferOffset,
                        buffer_row_len: region.bufferRowLength,
                        buffer_image_height: region.bufferImageHeight,
                        image_mip_level: region.imageSubresource.mipLevel,
                        image_base_array_level: region.imageSubresource.baseArrayLayer,
                        image_array_level_count: region.imageSubresource.layerCount,
                        image_offset: gpu::Offset3d {
                            x: region.imageOffset.x,
                            y: region.imageOffset.y,
                            z: region.imageOffset.z,
                        },
                        image_extent: gpu::Extent3d {
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
            self.gpu_command_buffer
                .record(gpu::Command::CopyImageToBuffer {
                    src_image: src_image.gpu_image(),
                    dst_buffer: dst_buffer.gpu_buffer(),
                    region: gpu::RegionCopyBufferImage {
                        buffer_offset: region.bufferOffset,
                        buffer_row_len: region.bufferRowLength,
                        buffer_image_height: region.bufferImageHeight,
                        image_mip_level: region.imageSubresource.mipLevel,
                        image_base_array_level: region.imageSubresource.baseArrayLayer,
                        image_array_level_count: region.imageSubresource.layerCount,
                        image_offset: gpu::Offset3d {
                            x: region.imageOffset.x,
                            y: region.imageOffset.y,
                            z: region.imageOffset.z,
                        },
                        image_extent: gpu::Extent3d {
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
            self.gpu_command_buffer
                .record(gpu::Command::CopyBufferToBuffer {
                    src_buffer: src_buffer.lock().gpu_buffer(),
                    dst_buffer: dst_buffer.lock().gpu_buffer(),
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
        command_buffers: impl IntoIterator<Item = Arc<Mutex<CommandBuffer>>>,
    ) {
        for command_buffer in command_buffers {
            self.gpu_command_buffer
                .record(gpu::Command::ExecuteCommands {
                    command_buffer: command_buffer.lock().gpu_command_buffer.clone(),
                })
        }
    }
}
