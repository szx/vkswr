use crate::{
    Color, DescriptorBuffer, DescriptorImage, Extent2d, Format, Fragment, Memory, Offset2d,
    MAX_VERTEX_ATTRIBUTES, MAX_VERTEX_ATTRIBUTE_OFFSET, MAX_VERTEX_BINDINGS,
    MAX_VERTEX_BINDING_STRIDE,
};
use hashbrown::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Default)]
pub struct GraphicsPipeline {
    render_targets: HashMap<RenderTargetIndex, RenderTarget>,
    vertex_buffers: [Option<VertexBuffer>; MAX_VERTEX_BINDINGS as usize],

    vertex_input_state: VertexInputState,
    input_assembly_state: InputAssemblyState,
}

impl GraphicsPipeline {
    pub fn new() -> Self {
        Self {
            render_targets: HashMap::default(),
            vertex_buffers: Default::default(),
            vertex_input_state: Default::default(),
            input_assembly_state: Default::default(),
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
        // TODO: Use VertexElement instead of color.
        let vertices = bytes
            .chunks_exact(element_stride as usize)
            .take(vertex_count as usize)
            .map(|element| Color::from_bytes(element_format, element));
        // TODO: vertex shader
        // TODO: tesselation assembler
        // TODO: tesselation control shader
        // TODO: tesselation primitive generation
        // TODO: tesselation evaluation shader
        // TODO: geometry assembler
        // TODO: geometry shader
        // TODO: primitive assembler

        // TODO: rasterization
        let fragments = vertices
            .map(|color| Fragment {
                position: Color::from_sfloat(
                    color.sfloat_32(0),
                    color.sfloat_32(1),
                    color.sfloat_32(2),
                    color.sfloat_32(3),
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
        for fragment in fragments {
            let width = rt.image.extent.width;
            let height = rt.image.extent.height;
            let (viewport_x, viewport_y) = (0u32, 0u32); // TODO: Use viewport state.
            let (viewport_width, viewport_height) = (height, width);
            let position = Color::from_bytes(
                rt.format,
                fragment.position.to_bytes(element_format).as_slice(),
            );
            let color = fragment.color.to_bytes(rt.format);

            let x = position.sfloat_32(0);
            let y = position.sfloat_32(1);
            let z = position.sfloat_32(2);
            let w = position.sfloat_32(3);
            let x_ndc = x / w;
            let y_ndc = y / w;
            let z_ndc = z / w; // TODO: Depth test.
            let x_screen = viewport_x + (0.5 * (x_ndc + 1.0) * (viewport_width as f32)) as u32;
            let y_screen = viewport_y + (0.5 * (y_ndc + 1.0) * (viewport_height as f32)) as u32;
            let dst_offset =
                ((x_screen + y_screen * width) * rt.format.bytes_per_pixel() as u32) as u64;
            memory.write_bytes(&color, &rt.image.binding, dst_offset); // TODO: Write texel to image function.
        }
    }
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct VertexBuffer {
    pub binding_number: VertexBindingNumber,
    pub buffer: DescriptorBuffer,
    pub offset: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct RenderArea {
    pub extent: Extent2d,
    pub offset: Offset2d,
}
