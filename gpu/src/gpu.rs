//! The software GPU: command submission and buffer/image copies.

use common::graphics::{DescriptorBuffer, DescriptorImage, IndexBuffer, VertexBuffer};
use common::math::{Color, Extent3, Format, Offset3};
use log::warn;

use crate::{
    GraphicsPipeline, InputAssemblyState, Memory, RasterizationState, RenderArea, RenderTarget,
    RenderTargetIndex, ViewportState,
};

/// Software implementation of a Vulkan physical device.
#[derive(Debug, Default)]
pub struct Gpu {
    pub memory: Memory,
    pub graphics_pipeline: GraphicsPipeline,
}

impl Gpu {
    /// Executes every command recorded in `command_buffer`.
    pub fn submit(&mut self, command_buffer: CommandBuffer) {
        warn!("TODO: Just submit, mpsc event loop on other thread?");
        for command in command_buffer.commands {
            command.apply(self);
        }
    }
}

/// A recorded GPU command.
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Command {
    CopyBufferToImage {
        src_buffer: DescriptorBuffer,
        dst_image: DescriptorImage,
        region: RegionCopyBufferImage,
    },
    CopyImageToBuffer {
        src_image: DescriptorImage,
        dst_buffer: DescriptorBuffer,
        region: RegionCopyBufferImage,
    },
    CopyBufferToBuffer {
        src_buffer: DescriptorBuffer,
        dst_buffer: DescriptorBuffer,
        region: RegionCopyBufferBuffer,
    },
    ExecuteCommands {
        command_buffer: CommandBuffer,
    },
    BindRenderTarget {
        render_target: RenderTarget,
    },
    UnbindRenderTarget {
        index: RenderTargetIndex,
    },
    ClearRenderTarget {
        index: RenderTargetIndex,
        render_area: RenderArea,
        color: Color,
    },
    SetShaderState {
        shader_state: shader::glsl::ShaderState,
    },
    SetVertexInputState {
        vertex_input_state: Box<common::graphics::VertexInputState>,
    },
    SetInputAssemblyState {
        input_assembly_state: InputAssemblyState,
    },
    SetViewportState {
        viewport_state: ViewportState,
    },
    SetRasterizationState {
        rasterization_state: RasterizationState,
    },
    BindVertexBuffer {
        vertex_buffer: VertexBuffer,
    },
    BindIndexBuffer {
        index_buffer: IndexBuffer,
    },
    DrawPrimitive {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
    DrawPrimitiveIndexed {
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    },
}

impl Command {
    fn apply(self, gpu: &mut Gpu) {
        match self {
            Self::CopyBufferToImage {
                src_buffer,
                dst_image,
                region,
            } => gpu.copy_buffer_to_image(&src_buffer, &dst_image, region),
            Self::CopyImageToBuffer {
                src_image,
                dst_buffer,
                region,
            } => gpu.copy_image_to_buffer(&src_image, &dst_buffer, region),
            Self::CopyBufferToBuffer {
                src_buffer,
                dst_buffer,
                region,
            } => gpu.copy_buffer_to_buffer(&src_buffer, &dst_buffer, region),
            Self::ExecuteCommands { command_buffer } => {
                warn!("TODO: Avoid submit recursion.");
                gpu.submit(command_buffer);
            }
            Self::BindRenderTarget { render_target } => {
                gpu.graphics_pipeline.bind_render_target(render_target);
            }
            Self::UnbindRenderTarget { index } => {
                gpu.graphics_pipeline.unbind_render_target(index);
            }
            Self::ClearRenderTarget {
                index,
                render_area,
                color,
            } => {
                gpu.graphics_pipeline.clear_render_target(
                    &mut gpu.memory,
                    index,
                    render_area,
                    color,
                );
            }
            Self::SetShaderState { shader_state } => {
                gpu.graphics_pipeline.set_shader_state(shader_state);
            }
            Self::SetVertexInputState { vertex_input_state } => {
                gpu.graphics_pipeline
                    .set_vertex_input_state(*vertex_input_state);
            }
            Self::SetInputAssemblyState {
                input_assembly_state,
            } => {
                gpu.graphics_pipeline
                    .set_input_assembly_state(input_assembly_state);
            }
            Self::SetViewportState { viewport_state } => {
                gpu.graphics_pipeline.set_viewport_state(viewport_state);
            }
            Self::SetRasterizationState {
                rasterization_state,
            } => {
                gpu.graphics_pipeline
                    .set_rasterization_state(rasterization_state);
            }
            Self::BindVertexBuffer { vertex_buffer } => {
                gpu.graphics_pipeline.bind_vertex_buffer(vertex_buffer);
            }
            Self::BindIndexBuffer { index_buffer } => {
                gpu.graphics_pipeline.bind_index_buffer(index_buffer);
            }
            Self::DrawPrimitive {
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            } => {
                gpu.graphics_pipeline.draw_primitive(
                    &mut gpu.memory,
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                );
            }
            Self::DrawPrimitiveIndexed {
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            } => {
                gpu.graphics_pipeline.draw_primitive_indexed(
                    &mut gpu.memory,
                    index_count,
                    instance_count,
                    first_index,
                    vertex_offset,
                    first_instance,
                );
            }
        }
    }
}

impl Gpu {
    fn copy_buffer_to_image(
        &mut self,
        src_buffer: &DescriptorBuffer,
        dst_image: &DescriptorImage,
        region: RegionCopyBufferImage,
    ) {
        self.copy_buffer_image(src_buffer, dst_image, region, true);
    }

    fn copy_image_to_buffer(
        &mut self,
        src_image: &DescriptorImage,
        dst_buffer: &DescriptorBuffer,
        region: RegionCopyBufferImage,
    ) {
        self.copy_buffer_image(dst_buffer, src_image, region, false);
    }

    /// Copies between a buffer and an image.
    ///
    /// `buffer_is_source` selects the copy direction; `buffer` and `image` are
    /// otherwise described identically.
    fn copy_buffer_image(
        &mut self,
        buffer: &DescriptorBuffer,
        image: &DescriptorImage,
        region: RegionCopyBufferImage,
        buffer_is_source: bool,
    ) {
        warn!("TODO: Complete buffer/image copy algorithm");
        assert_eq!(region.image_offset.x, 0);
        assert_eq!(region.image_offset.y, 0);
        assert_eq!(region.image_offset.z, 0);
        assert_eq!(region.image_mip_level, 0);
        assert_eq!(region.image_base_array_level, 0);
        assert_eq!(region.image_array_level_count, 1);
        assert_eq!(region.image_extent.depth, 1);

        let image_extent = region.image_extent;
        let image_height = if region.buffer_image_height == 0 {
            image_extent.height
        } else {
            region.buffer_image_height
        };
        let row_length = if region.buffer_row_len == 0 {
            image_extent.width
        } else {
            region.buffer_row_len
        };
        let texel_size = u64::from(region.image_format.info().bytes_per_pixel);
        let size = u64::from(row_length) * u64::from(image_height) * texel_size;

        let buffer_offset = buffer.binding.offset + region.buffer_offset;
        let image_offset = image.binding.offset;
        let (src, dst, src_offset, dst_offset) = if buffer_is_source {
            (&buffer.binding, &image.binding, buffer_offset, image_offset)
        } else {
            (&image.binding, &buffer.binding, image_offset, buffer_offset)
        };
        self.memory
            .copy_bytes(src, dst, src_offset, dst_offset, size);
    }

    fn copy_buffer_to_buffer(
        &mut self,
        src_buffer: &DescriptorBuffer,
        dst_buffer: &DescriptorBuffer,
        region: RegionCopyBufferBuffer,
    ) {
        self.memory.copy_bytes(
            &src_buffer.binding,
            &dst_buffer.binding,
            src_buffer.binding.offset + region.src_offset,
            dst_buffer.binding.offset + region.dst_offset,
            region.size,
        );
    }
}

/// A recorded command buffer.
#[derive(Debug, Clone, Default)]
pub struct CommandBuffer {
    commands: Vec<Command>,
}

impl CommandBuffer {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn record(&mut self, command: Command) {
        self.commands.push(command);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RegionCopyBufferImage {
    pub buffer_offset: u64,
    pub buffer_row_len: u32,
    pub buffer_image_height: u32,

    pub image_mip_level: u32,
    pub image_base_array_level: u32,
    pub image_array_level_count: u32,
    pub image_offset: Offset3<i32>,
    pub image_extent: Extent3<u32>,
    pub image_format: Format,
}

#[derive(Debug, Copy, Clone)]
pub struct RegionCopyBufferBuffer {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}
