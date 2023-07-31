use crate::{
    Color, DescriptorBuffer, DescriptorImage, Extent2, Format, Fragment, Memory, Offset2, Range2,
    Vertex, MAX_VERTEX_ATTRIBUTES, MAX_VERTEX_ATTRIBUTE_OFFSET, MAX_VERTEX_BINDINGS,
    MAX_VERTEX_BINDING_STRIDE, MAX_VIEWPORTS,
};
use hashbrown::HashMap;
use itertools::Itertools;
use std::ops::{Index, IndexMut};

#[derive(Default)]
pub struct GraphicsPipeline {
    render_targets: HashMap<RenderTargetIndex, RenderTarget>,
    vertex_buffers: [Option<VertexBuffer>; MAX_VERTEX_BINDINGS as usize],

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

        let bytes_per_pixel = rt.format.bytes_per_pixel();
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

    pub fn draw_primitive(
        &mut self,
        memory: &mut Memory,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        // Fetch vertices from vertex buffer using bindings.
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
        let element_size = element_format.bytes_per_pixel() as u32;
        let element_stride = if binding.stride == 0 {
            element_size
        } else {
            binding.stride
        };
        let bytes = memory
            .read_bytes(
                &vertex_buffer.buffer.binding,
                vertex_buffer.offset,
                vertex_buffer.buffer.binding.size - vertex_buffer.offset,
            )
            .to_vec();
        assert_eq!(bytes.len() % element_stride as usize, 0);
        // TODO: Determine vertex element components in shader?
        let vertices = bytes
            .chunks_exact(element_stride as usize)
            .take(vertex_count as usize)
            .map(|element| Vertex::from_bytes(element_format, element));
        // TODO: vertex shader
        // TODO: tesselation assembler
        // TODO: tesselation control shader
        // TODO: tesselation primitive generation
        // TODO: tesselation evaluation shader
        // TODO: geometry assembler
        // TODO: geometry shader

        // Primitive assembler.
        assert_eq!(
            self.input_assembly_state.topology,
            PrimitiveTopology::TriangleList
        );
        let primitives = vertices.clone().chunks(3); // TODO: remove clones after generating fragments
        let primitives = primitives
            .into_iter()
            .filter_map(|v| {
                let v = v.collect::<Vec<_>>();
                Triangle::new(v[0], v[1], v[2]) // TODO: CullMode.
            })
            .collect::<Vec<_>>();

        // TODO: rasterization
        // TODO: generate fragments from vertivec
        let fragments = vertices
            .map(|color| Fragment {
                position: Color::from_sfloat32(
                    color.sfloat32(0),
                    color.sfloat32(1),
                    color.sfloat32(2),
                    color.sfloat32(3),
                ),
                color,
            })
            .collect::<Vec<_>>();

        // TODO: pre-fragment operations
        // TODO: fragment assembler
        // TODO: fragment shader
        // TODO: post-fragment operations
        // TODO: color/blending operations

        // TODO: color attachment output
        let Some(rt) = self.render_targets.get_mut(&RenderTargetIndex(0)).cloned() else {
            // TODO: Determine used RenderTarget from fragment shader.
            unreachable!()
        };
        // TODO: Fragment shader should write directly to render target.
        let Some(viewport) = self.viewport_state.viewports[ViewportIndex(0)].as_ref() else {
            // TODO: Use all set viewports.
            unreachable!();
        };
        assert_eq!(viewport.offset.x, 0.0f32);
        assert_eq!(viewport.offset.y, 0.0f32);
        // TODO: Scissors.
        let (viewport_x, viewport_y) = (viewport.offset.x, viewport.offset.y);
        let (viewport_width, viewport_height) = (viewport.extent.width, viewport.extent.height);
        dbg!(viewport_width);
        for fragment in fragments {
            let width = rt.image.extent.width;
            let height = rt.image.extent.height;
            let position = Color::from_bytes(
                rt.format,
                fragment.position.to_bytes(element_format).as_slice(),
            );
            let color = fragment.color.to_bytes(rt.format);

            let x = position.sfloat32(0);
            let y = position.sfloat32(1);
            let z = position.sfloat32(2);
            let w = position.sfloat32(3);
            let x_ndc = x / w;
            let y_ndc = y / w;
            let z_ndc = z / w; // TODO: Depth test.
            let x_screen = (0.5 * (x_ndc + 1.0)).mul_add(viewport_width, viewport_x) as u32;
            let y_screen = (0.5 * (y_ndc + 1.0)).mul_add(viewport_height, viewport_y) as u32;
            let dst_offset =
                ((x_screen + y_screen * width) * rt.format.bytes_per_pixel() as u32) as u64;
            memory.write_bytes(&color, &rt.image.binding, dst_offset); // TODO: Write texel to image function.
        }
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VertexInputRate {
    Vertex,
    Instance,
}

#[derive(Debug, Clone, Default)]
pub struct InputAssemblyState {
    pub topology: PrimitiveTopology,
    pub primitive_restart: bool,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum PolygonMode {
    #[default]
    Fill,
    Line,
    Point,
    FillRectangle,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum CullMode {
    #[default]
    None,
    Front,
    Back,
    FrontAndBack,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
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

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    vertices: [Vertex; 3],
}

impl Triangle {
    /// Performs face culling.
    pub const fn new(v0: Vertex, v1: Vertex, v2: Vertex) -> Option<Self> {
        Some(Self {
            vertices: [v0, v1, v2],
        })
    }
}

impl Index<usize> for Triangle {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=2 => &self.vertices[index],
            _ => unreachable!(),
        }
    }
}

impl IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0..=2 => &mut self.vertices[index],
            _ => unreachable!(),
        }
    }
}
