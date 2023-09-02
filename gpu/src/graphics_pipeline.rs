use crate::{
    draw_line_bresenham, draw_points, Color, DescriptorBuffer, DescriptorImage, Extent2, Format,
    Fragment, FragmentShaderOutput, Memory, Offset2, Position, Range2, ShaderState, Vertex,
    VertexShaderOutput, MAX_VERTEX_ATTRIBUTES, MAX_VERTEX_ATTRIBUTE_OFFSET, MAX_VERTEX_BINDINGS,
    MAX_VERTEX_BINDING_STRIDE, MAX_VIEWPORTS,
};
use byteorder::ByteOrder;
use hashbrown::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Default)]
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
    pub fn new() -> Self {
        Self {
            render_targets: HashMap::default(),
            vertex_buffers: Default::default(),
            index_buffer: Default::default(),
            shader_state: Default::default(),
            vertex_input_state: Default::default(),
            input_assembly_state: Default::default(),
            viewport_state: Default::default(),
            rasterization_state: Default::default(),
        }
    }

    pub fn bind_render_target(&mut self, rt: RenderTarget) {
        self.render_targets.insert(rt.index, rt);
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
        let rt = self
            .render_targets
            .get(&index)
            .unwrap_or_else(|| unreachable!());
        assert_eq!(rt.samples, 1);
        assert!(area.offset.x >= 0);
        assert!(area.offset.y >= 0);

        let bytes_per_pixel = rt.format.info().bytes_per_pixel;
        let dst_offset = rt.image.extent.width * area.offset.y as u32 * bytes_per_pixel as u32;
        let dst = memory.get_memory_mut(&rt.image.binding);
        let mut dst = dst[dst_offset as usize..].as_mut_ptr();
        let src = color.to_bytes(rt.format);

        unsafe {
            let src = src.as_ptr();
            for _y in 0..area.extent.height {
                dst = dst.offset(area.offset.x as isize * bytes_per_pixel as isize);
                for _x in 0..area.extent.width {
                    std::ptr::copy_nonoverlapping(src, dst, bytes_per_pixel as usize);
                    dst = dst.offset(bytes_per_pixel as isize);
                }
            }
        }
    }

    pub fn set_shader_state(&mut self, shader_state: ShaderState) {
        self.shader_state = shader_state;
    }

    pub fn set_vertex_input_state(&mut self, vertex_input_state: VertexInputState) {
        self.vertex_input_state = vertex_input_state;
    }

    pub fn set_input_assembly_state(&mut self, input_assembly_state: InputAssemblyState) {
        self.input_assembly_state = input_assembly_state;
    }

    pub fn set_viewport_state(&mut self, viewport_state: ViewportState) {
        self.viewport_state = viewport_state;
    }

    pub fn set_rasterization_state(&mut self, rasterization_state: RasterizationState) {
        self.rasterization_state = rasterization_state;
    }

    pub fn bind_vertex_buffer(&mut self, vertex_buffer: VertexBuffer) {
        let index = vertex_buffer.binding_number;
        self.vertex_buffers[index] = Some(vertex_buffer);
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
        // Fetch vertices from vertex buffer using bindings.
        let vertices = self.fetch_vertex_input(
            memory,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );

        self.draw_primitive_rest(memory, vertices)
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

        self.draw_primitive_rest(memory, vertices)
    }

    fn draw_primitive_rest(&mut self, memory: &mut Memory, vertices: Vec<Vertex>) {
        // Vertex shader.
        let vertices = self.execute_vertex_shader(&self.vertex_input_state, vertices);

        // TODO: tesselation assembler
        // TODO: tesselation control shader
        // TODO: tesselation primitive generation
        // TODO: tesselation evaluation shader
        // TODO: geometry assembler
        // TODO: geometry shader

        // Primitive assembler.
        let Some(viewport) = self.viewport_state.viewports[ViewportIndex(0)].as_ref() else {
            // TODO: Use all set viewports.
            unreachable!();
        };
        assert_eq!(viewport.offset.x, 0.0f32);
        assert_eq!(viewport.offset.y, 0.0f32);
        let primitive_vertices = vertices
            .iter()
            .map(|vertex_shader_output| {
                let v = vertex_shader_output.position;

                let x = v.get_as_sfloat32(0);
                let y = v.get_as_sfloat32(1);
                let z = v.get_as_sfloat32(2);
                let w = v.get_as_sfloat32(3);
                // Perspective division.
                let x_ndc = x / w;
                let y_ndc = y / w;
                let z_ndc = z / w;
                // TODO: Depth test.
                // TODO: Back-face culling.
                // TODO: Clipping.
                // Viewport transformation
                // NOTE: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-viewport\
                assert_eq!(viewport.offset.x, 0.0);
                assert_eq!(viewport.offset.y, 0.0);
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
                    (p_x / 2.0) * x_ndc + o_x,
                    (p_y / 2.0) * y_ndc + o_y,
                    p_z * z_ndc + o_z,
                );
                Vertex {
                    position: Position::from_sfloat32_raw(x_screen, y_screen, z_screen, 1.0),
                    index: vertex_shader_output.vertex_index,
                }
            })
            .collect::<Vec<_>>();

        // Rasterization.
        let Some(rt) = self.render_targets.get_mut(&RenderTargetIndex(0)).cloned() else {
            // TODO: Determine used RenderTarget from fragment shader.
            unreachable!()
        };

        // TODO: Determine color in vertex shader.
        // TODO: Color interpolation.
        let color = Color::from_sfloat32_raw(1.0f32, 1.0f32, 1.0f32, 1.0f32);

        let mut fragments = vec![];
        match self.input_assembly_state.topology {
            PrimitiveTopology::PointList => draw_points(primitive_vertices, &mut fragments, color),
            PrimitiveTopology::LineList => unimplemented!(),
            PrimitiveTopology::LineStrip => unimplemented!(),
            PrimitiveTopology::TriangleList => {
                assert_eq!(primitive_vertices.len() % 3, 0);
                for triangle in primitive_vertices.chunks_exact(3) {
                    let vertices: [Vertex; 3] =
                        triangle.try_into().unwrap_or_else(|_| unreachable!());
                    match self.rasterization_state.polygon_mode {
                        PolygonMode::Fill | PolygonMode::Line => {
                            // TODO: Implement PolygonMode::Fill.
                            for i in 0..3 {
                                draw_line_bresenham(
                                    vertices[i],
                                    vertices[(i + 1) % 3],
                                    &mut fragments,
                                    color,
                                );
                            }
                        }
                        PolygonMode::Point => {
                            draw_points(vertices, &mut fragments, color);
                        }
                        PolygonMode::FillRectangle => unimplemented!(),
                    };
                }
            }
            PrimitiveTopology::TriangleStrip => unimplemented!(),
            PrimitiveTopology::TriangleFan => unimplemented!(),
            PrimitiveTopology::LineListWithAdjacency => unimplemented!(),
            PrimitiveTopology::LineStripWithAdjacency => unimplemented!(),
            PrimitiveTopology::TriangleListWithAdjacency => unimplemented!(),
            PrimitiveTopology::TriangleStripWithAdjacency => unimplemented!(),
            PrimitiveTopology::PatchList => unimplemented!(),
        };

        // TODO: early per-fragment operations

        // Fragment shader.
        let fragments = self.execute_fragment_shader(fragments);

        // TODO: late per-fragment operations
        // TODO: color/blending operations

        // Color attachment output
        // TODO: Fragment shader should write directly to render target.
        for fragment in fragments {
            let position = fragment.position;
            let color = fragment.color.to_bytes(rt.format);

            let framebuffer_width = rt.image.extent.width as u64;
            let framebuffer_height = rt.image.extent.height as u64;
            let framebuffer_x = position.get_as_sfloat32(0) as u64;
            let framebuffer_y = position.get_as_sfloat32(1) as u64;
            assert!(framebuffer_x < framebuffer_width);
            assert!(framebuffer_y < framebuffer_height);
            let dst_offset = (framebuffer_x + framebuffer_y * framebuffer_width)
                * rt.format.info().bytes_per_pixel as u64;
            memory.write_bytes(&color, &rt.image.binding, dst_offset); // TODO: Write texel to image function.
        }
    }
}

impl GraphicsPipeline {
    fn fetch_vertex_input(
        &mut self,
        memory: &mut Memory,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> Vec<Vertex> {
        assert_eq!(instance_count, 1);
        assert_eq!(first_vertex, 0);
        assert_eq!(first_instance, 0);
        let Some(binding) = self.vertex_input_state.bindings[0].as_ref() else {
            // TODO: Determine used VertexBindings from vertex shader (if any).
            unreachable!("{:#?}", self.vertex_input_state.bindings)
        };
        assert!(binding.stride < MAX_VERTEX_BINDING_STRIDE);
        assert_eq!(binding.input_rate, VertexInputRate::Vertex);

        let Some(attribute) = self.vertex_input_state.attributes[0].as_ref() else {
            // TODO: Determine used VertexAttributes from vertex shader (if any).
            unreachable!()
        };
        assert_eq!(attribute.binding, binding.number);
        assert_eq!(attribute.location, 0);
        assert!(attribute.offset < MAX_VERTEX_ATTRIBUTE_OFFSET);

        // Create elements from vertex buffer using attributes.
        let Some(vertex_buffer) = self.vertex_buffers[binding.number].as_ref() else {
            unreachable!()
        };
        let element_format = attribute.format;
        let element_size = element_format.info().bytes_per_pixel as u32;
        let element_stride = if binding.stride == 0 {
            element_size
        } else {
            binding.stride
        };
        let vertex_buffer_size = vertex_buffer.buffer.binding.size - vertex_buffer.offset;
        assert_eq!(vertex_buffer_size % element_stride as u64, 0);

        let bytes = memory
            .read_bytes(
                &vertex_buffer.buffer.binding,
                vertex_buffer.offset,
                vertex_buffer_size,
            )
            .to_vec(); // TODO: Stream vertex buffer bytes instead of reading all of them?

        // TODO: Determine vertex element components in shader?
        let vertices = bytes
            .chunks_exact(element_stride as usize)
            .take(vertex_count as usize)
            .map(|element| Vertex {
                position: Position::from_vertex_buffer_bytes(element_format, element),
                index: 0,
            })
            .collect();
        vertices
    }

    fn fetch_vertex_input_indexed(
        &self,
        memory: &mut Memory,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) -> Vec<Vertex> {
        assert_eq!(instance_count, 1);
        assert_eq!(first_index, 0);
        assert_eq!(first_instance, 0);

        let Some(index_buffer) = self.index_buffer.as_ref() else {
            unreachable!()
        };

        if let Some(binding) = self.vertex_input_state.bindings[0].as_ref() {
            // TODO: Determine used VertexBindings from vertex shader (if any).
            todo!();
        } else {
            let mut vertices = vec![];
            for index in first_index..first_index + index_count {
                let bytes = memory.read_bytes(
                    &index_buffer.buffer.binding,
                    index_buffer.offset + index as u64 * index_buffer.index_size as u64,
                    index_buffer.index_size as u64,
                );
                let index =
                    byteorder::NativeEndian::read_uint(bytes, index_buffer.index_size as usize)
                        as u32;
                vertices.push(Vertex {
                    position: Default::default(),
                    index,
                });
            }
            vertices
        }
    }

    fn execute_vertex_shader(
        &self,
        vertex_input_state: &VertexInputState,
        vertices: Vec<Vertex>,
    ) -> Vec<VertexShaderOutput> {
        let shader = self
            .shader_state
            .vertex_shader
            .as_ref()
            .unwrap_or_else(|| unreachable!());
        shader.execute_vertex_shader(vertex_input_state, vertices)
    }

    fn execute_fragment_shader(&self, fragments: Vec<Fragment>) -> Vec<FragmentShaderOutput> {
        let shader = self
            .shader_state
            .fragment_shader
            .as_ref()
            .unwrap_or_else(|| unreachable!());
        shader.execute_fragment_shader(fragments)
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

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct RenderTargetIndex(pub usize);

// TODO: struct RenderInput for Vulkan input attachments.

#[derive(Debug, Clone, Default)]
pub struct VertexInputState {
    pub attributes: [Option<VertexAttribute>; MAX_VERTEX_ATTRIBUTES as usize],
    pub bindings: [Option<VertexBinding>; MAX_VERTEX_BINDINGS as usize],
}

#[derive(Debug, Copy, Clone)]
pub struct VertexAttribute {
    /// Shader input location.
    pub location: u32,
    /// Binding number used to fetch data from.
    pub binding: VertexBindingNumber,
    /// Describes vertex attribute data.
    pub format: Format,
    /// Offset within element of binding.
    pub offset: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct VertexBinding {
    /// Binding number.
    pub number: VertexBindingNumber,
    /// Stride between elements.
    pub stride: u32,
    /// Specifies whether element is vertex of instance.
    pub input_rate: VertexInputRate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VertexBindingNumber(pub u32);

// TODO: impl_index_trait!()
impl Index<VertexBindingNumber> for [Option<VertexBuffer>] {
    type Output = Option<VertexBuffer>;

    fn index(&self, index: VertexBindingNumber) -> &Self::Output {
        let Some(value) = self.get(index.0 as usize) else {
            unreachable!()
        };
        value
    }
}

impl IndexMut<VertexBindingNumber> for [Option<VertexBuffer>] {
    fn index_mut(&mut self, index: VertexBindingNumber) -> &mut Self::Output {
        let Some(value) = self.get_mut(index.0 as usize) else {
            unreachable!()
        };
        value
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VertexInputRate {
    Vertex,
    Instance,
}

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

// TODO: impl_index_trait!()
impl Index<ViewportIndex> for [Option<Viewport>] {
    type Output = Option<Viewport>;

    fn index(&self, index: ViewportIndex) -> &Self::Output {
        let Some(value) = self.get(index.0 as usize) else {
            unreachable!()
        };
        value
    }
}

impl IndexMut<ViewportIndex> for [Option<Viewport>] {
    fn index_mut(&mut self, index: ViewportIndex) -> &mut Self::Output {
        let Some(value) = self.get_mut(index.0 as usize) else {
            unreachable!()
        };
        value
    }
}

impl Index<ViewportIndex> for [Option<Scissor>] {
    type Output = Option<Scissor>;

    fn index(&self, index: ViewportIndex) -> &Self::Output {
        let Some(value) = self.get(index.0 as usize) else {
            unreachable!()
        };
        value
    }
}

impl IndexMut<ViewportIndex> for [Option<Scissor>] {
    fn index_mut(&mut self, index: ViewportIndex) -> &mut Self::Output {
        let Some(value) = self.get_mut(index.0 as usize) else {
            unreachable!()
        };
        value
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

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum PolygonMode {
    #[default]
    Fill,
    Line,
    Point,
    FillRectangle,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum CullMode {
    #[default]
    None,
    Front,
    Back,
    FrontAndBack,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum FrontFace {
    #[default]
    CounterClockwise,
    Clockwise,
}

#[derive(Debug, Clone)]
pub struct VertexBuffer {
    pub binding_number: VertexBindingNumber,
    pub buffer: DescriptorBuffer,
    pub offset: u64,
}

#[derive(Debug, Clone)]
pub struct IndexBuffer {
    pub buffer: DescriptorBuffer,
    pub offset: u64,
    pub index_size: u8,
}
