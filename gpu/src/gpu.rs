use crate::{
    Color, Format, GraphicsPipeline, InputAssemblyState, Memory, MemoryBinding, RenderArea,
    RenderTarget, RenderTargetIndex, VertexBuffer, VertexInputState,
};
use std::fmt::{Debug, Formatter};

#[derive(Default)]
pub struct Gpu {
    pub memory: Memory,
    pub graphics_pipeline: GraphicsPipeline,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            memory: Default::default(),
            graphics_pipeline: Default::default(),
        }
    }

    pub fn submit(&mut self, command_buffer: CommandBuffer) {
        // TODO: Just submit, mpsc event loop on other thread.
        for command in command_buffer.commands {
            match command {
                Command::CopyBufferToImage {
                    src_buffer,
                    dst_image,
                    region,
                } => {
                    self.copy_buffer_to_image(src_buffer, dst_image, region);
                }
                Command::CopyImageToBuffer {
                    src_image,
                    dst_buffer,
                    region,
                } => {
                    self.copy_image_to_buffer(src_image, dst_buffer, region);
                }
                Command::CopyBufferToBuffer {
                    src_buffer,
                    dst_buffer,
                    region,
                } => {
                    self.copy_buffer_to_buffer(src_buffer, dst_buffer, region);
                }
                Command::ExecuteCommands { command_buffer } => {
                    // TODO: Avoid submit recursion.
                    self.submit(command_buffer);
                }
                Command::BindRenderTarget { render_target } => {
                    self.graphics_pipeline.bind_render_target(render_target);
                }
                Command::UnbindRenderTarget { index } => {
                    self.graphics_pipeline.unbind_render_target(index);
                }
                Command::ClearRenderTarget {
                    index,
                    render_area,
                    color,
                } => {
                    self.graphics_pipeline.clear_render_target(
                        &mut self.memory,
                        index,
                        render_area,
                        color,
                    );
                }
                Command::SetVertexInputState { vertex_input_state } => {
                    self.graphics_pipeline
                        .set_vertex_input_state(vertex_input_state);
                }
                Command::SetInputAssemblyState {
                    input_assembly_state,
                } => {
                    self.graphics_pipeline
                        .set_input_assembly_state(input_assembly_state);
                }
                Command::BindVertexBuffer { vertex_buffer } => {
                    self.graphics_pipeline.bind_vertex_buffer(vertex_buffer);
                }
                Command::DrawPrimitive {
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                } => {
                    self.graphics_pipeline.draw_primitive(
                        &mut self.memory,
                        vertex_count,
                        instance_count,
                        first_vertex,
                        first_instance,
                    );
                }
            }
        }
    }
}

impl Gpu {
    fn copy_buffer_to_image(
        &mut self,
        src_buffer: DescriptorBuffer,
        dst_image: DescriptorImage,
        region: RegionCopyBufferImage,
    ) {
        // TODO: Complete buffer to image copy algorithm.
        let buffer_image_height = if region.buffer_image_height == 0 {
            region.image_extent.height
        } else {
            region.buffer_image_height
        };
        let buffer_row_len = if region.buffer_row_len == 0 {
            region.image_extent.width
        } else {
            region.buffer_row_len
        };
        assert_eq!(region.image_offset.x, 0);
        assert_eq!(region.image_offset.y, 0);
        assert_eq!(region.image_offset.z, 0);
        assert_eq!(region.image_mip_level, 0);
        assert_eq!(region.image_base_array_level, 0);
        assert_eq!(region.image_array_level_count, 1);
        assert_eq!(region.image_extent.depth, 1);
        let size =
            buffer_row_len * buffer_image_height * region.image_format.bytes_per_pixel() as u32;
        let src_offset = region.buffer_offset;
        let dst_offset = 0;
        self.memory.copy_bytes(
            &src_buffer.binding,
            &dst_image.binding,
            src_offset,
            dst_offset,
            size as u64,
        );
    }

    fn copy_image_to_buffer(
        &mut self,
        src_image: DescriptorImage,
        dst_buffer: DescriptorBuffer,
        region: RegionCopyBufferImage,
    ) {
        // TODO: Complete buffer to image copy algorithm.
        assert_eq!(region.buffer_offset, 0);
        let buffer_image_height = if region.buffer_image_height == 0 {
            region.image_extent.height
        } else {
            region.buffer_image_height
        };
        let buffer_row_len = if region.buffer_row_len == 0 {
            region.image_extent.width
        } else {
            region.buffer_row_len
        };
        assert_eq!(region.image_offset.x, 0);
        assert_eq!(region.image_offset.y, 0);
        assert_eq!(region.image_offset.z, 0);
        assert_eq!(region.image_mip_level, 0);
        assert_eq!(region.image_base_array_level, 0);
        assert_eq!(region.image_array_level_count, 1);
        assert_eq!(region.image_extent.depth, 1);
        let size =
            buffer_row_len * buffer_image_height * region.image_format.bytes_per_pixel() as u32;
        let src_offset = 0;
        let dst_offset = region.buffer_offset;
        self.memory.copy_bytes(
            &src_image.binding,
            &dst_buffer.binding,
            src_offset,
            dst_offset,
            size as u64,
        );
    }

    fn copy_buffer_to_buffer(
        &mut self,
        src_buffer: DescriptorBuffer,
        dst_buffer: DescriptorBuffer,
        region: RegionCopyBufferBuffer,
    ) {
        self.memory.copy_bytes(
            &src_buffer.binding,
            &dst_buffer.binding,
            region.src_offset,
            region.dst_offset,
            region.size,
        );
    }
}

impl Debug for Gpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gpu").finish()
    }
}

#[derive(Debug, Clone)]
pub struct CommandBuffer {
    commands: Vec<Command>,
}

impl CommandBuffer {
    pub const fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn record(&mut self, command: Command) {
        self.commands.push(command);
    }
}

#[derive(Debug, Clone)]
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
    SetVertexInputState {
        vertex_input_state: VertexInputState,
    },
    SetInputAssemblyState {
        input_assembly_state: InputAssemblyState,
    },
    BindVertexBuffer {
        vertex_buffer: VertexBuffer,
    },
    DrawPrimitive {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct RegionCopyBufferImage {
    pub buffer_offset: u64,
    pub buffer_row_len: u32,
    pub buffer_image_height: u32,

    pub image_mip_level: u32,
    pub image_base_array_level: u32,
    pub image_array_level_count: u32,
    pub image_offset: Offset3d,
    pub image_extent: Extent3d,
    pub image_format: Format,
}

#[derive(Debug, Copy, Clone)]
pub struct RegionCopyBufferBuffer {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct DescriptorBuffer {
    pub binding: MemoryBinding,
}

#[derive(Debug, Clone)]
pub struct DescriptorImage {
    pub binding: MemoryBinding,
    pub extent: Extent3d,
}

#[derive(Debug, Copy, Clone)]
pub struct Offset3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Offset2d {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Extent2d {
    pub width: u32,
    pub height: u32,
}
