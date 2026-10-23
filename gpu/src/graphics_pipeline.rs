//! Fixed-function graphics pipeline state and draw execution.

use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use common::consts::{
    MAX_VERTEX_ATTRIBUTE_OFFSET, MAX_VERTEX_BINDING_STRIDE, MAX_VERTEX_BINDINGS, MAX_VIEWPORTS,
};
use common::graphics::{
    CullMode, DescriptorImage, FrontFace, IndexBuffer, PolygonMode, VertexBuffer, VertexInputRate,
    VertexInputState,
};
use common::math::{Color, Extent2, Format, Fragment, Offset2, Position, Range2, Vertex};
use log::warn;
use shader::glsl::{FragmentShaderOutput, ShaderState, VertexShaderOutput};

use crate::{Memory, draw_line_bresenham, draw_points, draw_triangle};

#[derive(Debug, Default)]
pub struct GraphicsPipeline {
    render_targets: HashMap<RenderTargetIndex, RenderTarget>,
    vertex_buffers: [Option<VertexBuffer>; MAX_VERTEX_BINDINGS as usize],
    index_buffer: Option<IndexBuffer>,

    shader_state: ShaderState,
    vertex_input_state: VertexInputState,
    input_assembly_state: InputAssemblyState,
    viewport_state: ViewportState,
    rasterization_state: RasterizationState,
}

impl GraphicsPipeline {
    pub fn bind_render_target(&mut self, render_target: RenderTarget) {
        self.render_targets
            .insert(render_target.index, render_target);
    }

    pub fn unbind_render_target(&mut self, index: RenderTargetIndex) {
        self.render_targets.remove(&index);
    }

    pub fn clear_render_target(
        &self,
        memory: &mut Memory,
        index: RenderTargetIndex,
        area: RenderArea,
        color: Color,
    ) {
        let render_target = self
            .render_targets
            .get(&index)
            .expect("render target is bound");
        assert_eq!(render_target.samples, 1);
        assert!(area.offset.x >= 0);
        assert!(area.offset.y >= 0);

        let texel_size = usize::from(render_target.format.info().bytes_per_pixel);
        let width = area.extent.width as usize;
        let height = area.extent.height as usize;
        let row_stride = render_target.image.extent.width as usize * texel_size;
        let image_x = area.offset.x as usize;
        let image_y = area.offset.y as usize;
        let dst_base = render_target.image.binding.offset as usize
            + (image_y * row_stride + image_x * texel_size);

        let src = color.to_bytes(render_target.format);
        let dst = memory.get_memory_mut(&render_target.image.binding);
        for y in 0..height {
            let row_start = dst_base + y * row_stride;
            let row = &mut dst[row_start..row_start + width * texel_size];
            for texel in row.chunks_exact_mut(texel_size) {
                texel.copy_from_slice(&src);
            }
        }
    }

    pub fn set_shader_state(&mut self, shader_state: ShaderState) {
        self.shader_state = shader_state;
    }

    pub const fn set_vertex_input_state(&mut self, vertex_input_state: VertexInputState) {
        self.vertex_input_state = vertex_input_state;
    }

    pub const fn set_input_assembly_state(&mut self, input_assembly_state: InputAssemblyState) {
        self.input_assembly_state = input_assembly_state;
    }

    pub const fn set_viewport_state(&mut self, viewport_state: ViewportState) {
        self.viewport_state = viewport_state;
    }

    pub const fn set_rasterization_state(&mut self, rasterization_state: RasterizationState) {
        self.rasterization_state = rasterization_state;
    }

    pub fn bind_vertex_buffer(&mut self, vertex_buffer: VertexBuffer) {
        let binding = vertex_buffer.binding_number;
        self.vertex_buffers[binding] = Some(vertex_buffer);
    }

    pub fn bind_index_buffer(&mut self, index_buffer: IndexBuffer) {
        self.index_buffer = Some(index_buffer);
    }

    pub fn draw_primitive(
        &mut self,
        memory: &mut Memory,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        let vertices = self.fetch_vertex_input(
            memory,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );
        self.draw_primitive_rest(memory, vertices);
    }

    pub fn draw_primitive_indexed(
        &mut self,
        memory: &mut Memory,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        let vertices = self.fetch_vertex_input_indexed(
            memory,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        );
        self.draw_primitive_rest(memory, vertices);
    }

    fn draw_primitive_rest(&self, memory: &mut Memory, vertices: Vec<Vertex>) {
        // Vertex shader.
        let vertices = self.execute_vertex_shader(vertices);

        warn!("TODO: tesselation assembler");
        warn!("TODO: tesselation control shader");
        warn!("TODO: tesselation primitive generation");
        warn!("TODO: tesselation evaluation shader");
        warn!("TODO: geometry assembler");
        warn!("TODO: geometry shader");

        // Primitive assembler.
        let Some(viewport) = self.viewport_state.viewports[ViewportIndex(0)].as_ref() else {
            warn!("TODO: Use all set viewports");
            panic!("viewport 0 is not set");
        };
        assert_eq!(viewport.offset.x, 0.0);
        assert_eq!(viewport.offset.y, 0.0);

        let primitive_vertices: Vec<Vertex> = vertices
            .iter()
            .map(|output| {
                let position = output.position;
                let [x, y, z, w] = position.get_as_f32_array();
                // Perspective division.
                let x_ndc = x / w;
                let y_ndc = y / w;
                let z_ndc = z / w;
                warn!("TODO: Depth test.");
                warn!("TODO: Back-face culling.");
                warn!("TODO: Clipping.");
                // Viewport transformation.
                // See <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-viewport>.
                let (p_x, p_y, p_z) = (
                    viewport.extent.width,
                    viewport.extent.height,
                    viewport.depth.max - viewport.depth.min,
                );
                let (o_x, o_y, o_z) = (
                    viewport.offset.x + viewport.extent.width / 2.0,
                    viewport.offset.y + viewport.extent.height / 2.0,
                    viewport.depth.min,
                );
                let (x_screen, y_screen, z_screen) = (
                    (p_x / 2.0).mul_add(x_ndc, o_x),
                    (p_y / 2.0).mul_add(y_ndc, o_y),
                    p_z.mul_add(z_ndc, o_z),
                );
                Vertex {
                    position: Position::from_sfloat32_raw(x_screen, y_screen, z_screen, 1.0),
                    point_size: 1.0,
                    index: output.vertex_index,
                    clip_distances: [0.0; common::consts::MAX_CLIP_DISTANCES as usize],
                }
            })
            .collect();

        // Rasterization.
        let render_target = self
            .render_targets
            .get(&RenderTargetIndex(0))
            .cloned()
            .unwrap_or_else(|| {
                warn!("TODO: Determine used RenderTarget from fragment shader");
                panic!("render target 0 is not bound");
            });

        warn!("TODO: Determine color in vertex shader");
        warn!("TODO: Color interpolation");
        let color = Color::from_sfloat32_raw(1.0, 1.0, 1.0, 1.0);

        let mut fragments = Vec::new();
        match self.input_assembly_state.topology {
            PrimitiveTopology::PointList => draw_points(primitive_vertices, &mut fragments, color),
            PrimitiveTopology::LineList | PrimitiveTopology::LineStrip => unimplemented!(),
            PrimitiveTopology::TriangleList => {
                assert_eq!(primitive_vertices.len() % 3, 0);
                for triangle in primitive_vertices.chunks_exact(3) {
                    let vertices: [Vertex; 3] = triangle
                        .try_into()
                        .expect("chunks_exact(3) yields three vertices");
                    match self.rasterization_state.polygon_mode {
                        PolygonMode::Fill => draw_triangle(
                            vertices[0],
                            vertices[1],
                            vertices[2],
                            render_target.image.extent.width,
                            render_target.image.extent.height,
                            &mut fragments,
                            color,
                        ),
                        PolygonMode::Line => {
                            for (index, vertex) in vertices.iter().enumerate() {
                                draw_line_bresenham(
                                    *vertex,
                                    vertices[(index + 1) % 3],
                                    &mut fragments,
                                    color,
                                );
                            }
                        }
                        PolygonMode::Point => draw_points(vertices, &mut fragments, color),
                        PolygonMode::FillRectangle => unimplemented!(),
                    }
                }
            }
            PrimitiveTopology::TriangleStrip
            | PrimitiveTopology::TriangleFan
            | PrimitiveTopology::LineListWithAdjacency
            | PrimitiveTopology::LineStripWithAdjacency
            | PrimitiveTopology::TriangleListWithAdjacency
            | PrimitiveTopology::TriangleStripWithAdjacency
            | PrimitiveTopology::PatchList => unimplemented!(),
        }

        warn!("TODO: early per-fragment operations");

        // Fragment shader.
        let fragments = self.execute_fragment_shader(fragments);

        warn!("TODO: late per-fragment operations");
        warn!("TODO: color/blending operations");

        // Color attachment output.
        warn!("TODO: Fragment shader should write directly to render target");
        let framebuffer_width = u64::from(render_target.image.extent.width);
        let framebuffer_height = u64::from(render_target.image.extent.height);
        let texel_size = u64::from(render_target.format.info().bytes_per_pixel);
        for fragment in fragments {
            let color = fragment.color.to_bytes(render_target.format);
            let framebuffer_x = fragment.position.get_as_sfloat32(0) as u64;
            let framebuffer_y = fragment.position.get_as_sfloat32(1) as u64;
            assert!(framebuffer_x < framebuffer_width);
            assert!(framebuffer_y < framebuffer_height);
            let dst_offset = render_target.image.binding.offset
                + (framebuffer_x + framebuffer_y * framebuffer_width) * texel_size;
            warn!("TODO: Write texel to image function");
            memory.write_bytes(&color, &render_target.image.binding, dst_offset);
        }
    }
}

impl GraphicsPipeline {
    fn fetch_vertex_input(
        &self,
        memory: &Memory,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> Vec<Vertex> {
        assert_eq!(instance_count, 1);
        assert_eq!(first_vertex, 0);
        assert_eq!(first_instance, 0);

        if self.vertex_input_state.bindings.iter().all(Option::is_none) {
            return (first_vertex..vertex_count)
                .map(|index| Vertex {
                    position: Position::default(),
                    point_size: 1.0,
                    index,
                    clip_distances: Default::default(),
                })
                .collect();
        }

        let Some(binding) = self.vertex_input_state.bindings[0].as_ref() else {
            warn!("TODO: Determine used VertexBindings from vertex shader (if any)");
            panic!("{:#?}", self.vertex_input_state.bindings);
        };
        assert!(binding.stride < MAX_VERTEX_BINDING_STRIDE);
        assert_eq!(binding.input_rate, VertexInputRate::Vertex);

        let Some(attribute) = self.vertex_input_state.attributes[0].as_ref() else {
            warn!("TODO: Determine used VertexAttributes from vertex shader (if any)");
            panic!("vertex attribute 0 is not set");
        };
        assert_eq!(attribute.binding, binding.number);
        assert_eq!(attribute.location, 0);
        assert!(attribute.offset < MAX_VERTEX_ATTRIBUTE_OFFSET);

        // Create elements from the vertex buffer using the attributes.
        let Some(vertex_buffer) = self.vertex_buffers[binding.number].as_ref() else {
            panic!(
                "vertex buffer for binding {} is not bound",
                binding.number.0
            );
        };
        let element_format = attribute.format;
        let element_size = u32::from(element_format.info().bytes_per_pixel);
        let element_stride = if binding.stride == 0 {
            element_size
        } else {
            binding.stride
        };
        let binding = &vertex_buffer.buffer.binding;
        let vertex_buffer_size = binding.size - vertex_buffer.offset;
        assert_eq!(vertex_buffer_size % u64::from(element_stride), 0);

        warn!("TODO: Stream vertex buffer bytes instead of reading all of them?");
        warn!("TODO: Determine vertex element components in shader?");
        memory
            .read_bytes(
                binding,
                binding.offset + vertex_buffer.offset,
                vertex_buffer_size,
            )
            .chunks_exact(element_stride as usize)
            .take(vertex_count as usize)
            .map(|element| Vertex {
                position: Position::from_vertex_buffer_bytes(element_format, element),
                point_size: 1.0,
                index: 0,
                clip_distances: Default::default(),
            })
            .collect()
    }

    fn fetch_vertex_input_indexed(
        &self,
        memory: &Memory,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        _vertex_offset: i32,
        first_instance: u32,
    ) -> Vec<Vertex> {
        assert_eq!(instance_count, 1);
        assert_eq!(first_index, 0);
        assert_eq!(first_instance, 0);

        let Some(index_buffer) = self.index_buffer.as_ref() else {
            panic!("index buffer is not bound");
        };

        if self.vertex_input_state.bindings[0].is_some() {
            warn!("TODO: Determine used VertexBindings from vertex shader (if any)");
            todo!();
        }

        let binding = &index_buffer.buffer.binding;
        let index_size = u64::from(index_buffer.index_size);
        (first_index..first_index + index_count)
            .map(|index| {
                let bytes = memory.read_bytes(
                    binding,
                    binding.offset + index_buffer.offset + u64::from(index) * index_size,
                    index_size,
                );
                Vertex {
                    position: Position::default(),
                    point_size: 1.0,
                    index: read_index(bytes, index_buffer.index_size),
                    clip_distances: Default::default(),
                }
            })
            .collect()
    }

    fn execute_vertex_shader(&self, vertices: Vec<Vertex>) -> Vec<VertexShaderOutput> {
        self.shader_state
            .vertex_shader
            .as_ref()
            .expect("vertex shader is bound")
            .execute_vertex_shader(&self.vertex_input_state, vertices)
    }

    fn execute_fragment_shader(&self, fragments: Vec<Fragment>) -> Vec<FragmentShaderOutput> {
        self.shader_state
            .fragment_shader
            .as_ref()
            .expect("fragment shader is bound")
            .execute_fragment_shader(fragments)
    }
}

fn read_index(bytes: &[u8], index_size: u8) -> u32 {
    match index_size {
        1 => u32::from(bytes[0]),
        2 => u32::from(u16::from_ne_bytes(
            bytes.try_into().expect("index buffer has two bytes"),
        )),
        4 => u32::from_ne_bytes(bytes.try_into().expect("index buffer has four bytes")),
        size => panic!("unsupported index size: {size}"),
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RenderArea {
    pub extent: Extent2<u32>,
    pub offset: Offset2<i32>,
}

#[derive(Debug, Clone)]
pub struct RenderTarget {
    pub index: RenderTargetIndex,
    pub format: Format,
    pub samples: u32,
    pub image: DescriptorImage,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RenderTargetIndex(pub usize);

#[derive(Debug, Clone, Default)]
pub struct InputAssemblyState {
    pub topology: PrimitiveTopology,
    pub primitive_restart: bool,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum PrimitiveTopology {
    #[default]
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
    LineListWithAdjacency,
    LineStripWithAdjacency,
    TriangleListWithAdjacency,
    TriangleStripWithAdjacency,
    PatchList,
}

#[derive(Debug, Clone, Default)]
pub struct ViewportState {
    pub viewports: [Option<Viewport>; MAX_VIEWPORTS as usize],
    pub scissors: [Option<Scissor>; MAX_VIEWPORTS as usize],
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Viewport {
    pub offset: Offset2<f32>,
    pub extent: Extent2<f32>,
    pub depth: Range2<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct Scissor {
    pub render_area: RenderArea,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ViewportIndex(pub u32);

impl Index<ViewportIndex> for [Option<Viewport>] {
    type Output = Option<Viewport>;

    fn index(&self, index: ViewportIndex) -> &Self::Output {
        self.get(index.0 as usize)
            .expect("viewport index is within MAX_VIEWPORTS")
    }
}

impl IndexMut<ViewportIndex> for [Option<Viewport>] {
    fn index_mut(&mut self, index: ViewportIndex) -> &mut Self::Output {
        self.get_mut(index.0 as usize)
            .expect("viewport index is within MAX_VIEWPORTS")
    }
}

impl Index<ViewportIndex> for [Option<Scissor>] {
    type Output = Option<Scissor>;

    fn index(&self, index: ViewportIndex) -> &Self::Output {
        self.get(index.0 as usize)
            .expect("scissor index is within MAX_VIEWPORTS")
    }
}

impl IndexMut<ViewportIndex> for [Option<Scissor>] {
    fn index_mut(&mut self, index: ViewportIndex) -> &mut Self::Output {
        self.get_mut(index.0 as usize)
            .expect("scissor index is within MAX_VIEWPORTS")
    }
}

#[derive(Debug, Clone, Default)]
pub struct RasterizationState {
    pub depth_clamp_enable: bool,
    pub rasterizer_discard_enable: bool,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullMode,
    pub front_face: FrontFace,
    pub depth_bias_enable: bool,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}
