extern crate core;

use hashbrown::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut, Range};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub const MAX_VERTEX_ATTRIBUTES: u32 = 16;
pub const MAX_VERTEX_BINDINGS: u32 = 16;
pub const MAX_VERTEX_ATTRIBUTE_OFFSET: u32 = 2047;
pub const MAX_VERTEX_BINDING_STRIDE: u32 = 2048;

#[derive(Default)]
pub struct Gpu {
    // TODO: Usa bitmap allocator.
    memory_allocations: HashMap<MemoryBindingHandle, Vec<u8>>,
    memory_allocation_index: AtomicU64,

    render_targets: HashMap<RenderTargetIndex, RenderTarget>,
    vertex_buffers: [Option<VertexBuffer>; MAX_VERTEX_BINDINGS as usize],

    vertex_input_state: VertexInputState,
    input_assembly_state: InputAssemblyState,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            memory_allocations: HashMap::default(),
            memory_allocation_index: AtomicU64::new(1),
            render_targets: HashMap::default(),
            vertex_buffers: Default::default(),
            vertex_input_state: Default::default(),
            input_assembly_state: Default::default(),
        }
    }

    pub const fn memory_size_in_bytes() -> u64 {
        10 * 1024 * 1024
    }

    pub fn allocate_memory(&mut self, size: u64) -> MemoryAllocation {
        let handle =
            MemoryBindingHandle(self.memory_allocation_index.fetch_add(1, Ordering::Relaxed));
        self.memory_allocations
            .insert(handle, vec![0; size as usize]);
        MemoryAllocation { handle, size }
    }

    pub fn free_memory(&mut self, memory_allocation: MemoryAllocation) {
        self.memory_allocations.remove(&memory_allocation.handle);
    }

    pub fn map_host(
        &mut self,
        memory_allocation: MemoryAllocation,
        offset: u64,
        size: u64,
    ) -> NonNull<std::ffi::c_void> {
        let memory = self
            .memory_allocations
            .get_mut(&memory_allocation.handle)
            .unwrap_or_else(|| unreachable!());
        let ptr = memory[offset as usize..(offset + size) as usize].as_mut_ptr();
        unsafe { NonNull::new_unchecked(ptr as *mut std::ffi::c_void) }
    }

    pub const fn unmap_host(&self, _memory_allocation: MemoryAllocation) {
        // No-op.
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
                    self.bind_render_target(render_target);
                }
                Command::UnbindRenderTarget { index } => {
                    self.unbind_render_target(index);
                }
                Command::ClearRenderTarget {
                    index,
                    render_area,
                    color,
                } => {
                    self.clear_render_target(index, render_area, color);
                }
                Command::SetVertexInputState { vertex_input_state } => {
                    self.set_vertex_input_state(vertex_input_state);
                }
                Command::SetInputAssemblyState {
                    input_assembly_state,
                } => {
                    self.set_input_assembly_state(input_assembly_state);
                }
                Command::BindVertexBuffer { vertex_buffer } => {
                    self.bind_vertex_buffer(vertex_buffer);
                }
                Command::DrawPrimitive {
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                } => {
                    self.draw_primitive(vertex_count, instance_count, first_vertex, first_instance);
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
        self.copy_bytes(
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
        self.copy_bytes(
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
        self.copy_bytes(
            &src_buffer.binding,
            &dst_buffer.binding,
            region.src_offset,
            region.dst_offset,
            region.size,
        );
    }

    fn bind_render_target(&mut self, rt: RenderTarget) {
        self.render_targets.insert(rt.index, rt);
    }

    fn unbind_render_target(&mut self, index: RenderTargetIndex) {
        self.render_targets.remove(&index);
    }

    fn clear_render_target(&mut self, index: RenderTargetIndex, area: RenderArea, color: Color) {
        let rt = self
            .render_targets
            .get(&index)
            .unwrap_or_else(|| unreachable!());
        assert_eq!(rt.samples, 1);
        assert!(area.offset.x >= 0);
        assert!(area.offset.y >= 0);

        let bytes_per_pixel = rt.format.bytes_per_pixel();
        let dst_offset = rt.image.extent.width * area.offset.y as u32 * bytes_per_pixel as u32;
        let dst = self
            .memory_allocations
            .get_mut(&MemoryBindingHandle(
                rt.image.binding.memory_handle.load(Ordering::Relaxed),
            ))
            .unwrap_or_else(|| unreachable!());
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

    fn set_vertex_input_state(&mut self, vertex_input_state: VertexInputState) {
        self.vertex_input_state = vertex_input_state;
    }

    fn set_input_assembly_state(&mut self, input_assembly_state: InputAssemblyState) {
        self.input_assembly_state = input_assembly_state;
    }

    fn bind_vertex_buffer(&mut self, vertex_buffer: VertexBuffer) {
        let index = vertex_buffer.binding_number;
        self.vertex_buffers[index] = Some(vertex_buffer);
    }

    fn draw_primitive(
        &mut self,
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
        let bytes = self
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
            self.write_bytes(&color, &rt.image.binding, dst_offset); // TODO: Write texel to image function.
        }
    }
}

impl Debug for Gpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gpu")
            .field("memory_allocation_index", &self.memory_allocation_index)
            .finish()
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

#[derive(Debug, Copy, Clone)]
pub enum Format {
    R8Unorm,
    R8G8Unorm,
    R8G8B8A8Unorm,
    R32G32B32A32Sfloat,
    A2b10g10r10UnormPack32,
    D16Unorm,
}

impl Format {
    pub const fn bytes_per_pixel(&self) -> u8 {
        match *self {
            Format::R8Unorm => 1,
            Format::R8G8Unorm => 2,
            Format::R8G8B8A8Unorm => 4,
            Format::R32G32B32A32Sfloat => 16,
            Format::A2b10g10r10UnormPack32 => 4,
            Format::D16Unorm => 2,
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

#[derive(Debug, Clone)]
pub struct DescriptorBuffer {
    pub binding: MemoryBinding,
}

#[derive(Debug, Clone)]
pub struct DescriptorImage {
    pub binding: MemoryBinding,
    pub extent: Extent3d,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryBinding {
    /// Thanks to Arc cloned resource binding points to the same MemoryAllocation
    memory_handle: Arc<AtomicU64>,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryAllocation {
    handle: MemoryBindingHandle,
    pub size: u64,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct MemoryBindingHandle(pub u64);

impl MemoryBinding {
    pub fn new() -> Self {
        Self {
            memory_handle: Arc::new(Default::default()),
            offset: 0,
            size: 0,
        }
    }

    pub fn store(&mut self, memory_allocation: MemoryAllocation, offset: u64, size: u64) {
        self.memory_handle
            .store(memory_allocation.handle.0, Ordering::Relaxed);
        self.offset = offset;
        self.size = size;
    }
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

#[derive(Debug, Copy, Clone)]
pub struct Fragment {
    position: Color, // TODO: Replace Color with Vector4?
    color: Color,
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    /// Bit representation of components.
    components: [u32; 4],
}

impl Color {
    pub const fn from_raw(r: u32, g: u32, b: u32, a: u32) -> Self {
        Self {
            components: [r, g, b, a],
        }
    }

    pub fn from_sfloat(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            components: [
                f32::to_bits(r),
                f32::to_bits(g),
                f32::to_bits(b),
                f32::to_bits(a),
            ],
        }
    }

    pub fn sfloat_32(&self, index: impl std::slice::SliceIndex<[u32], Output = u32>) -> f32 {
        f32::from_bits(self.components[index])
    }

    pub fn unorm_8(&self, index: impl std::slice::SliceIndex<[u32], Output = u32>) -> u8 {
        let value = f32::from_bits(self.components[index]);
        (value * 255.0f32).round() as u8
    }

    pub fn unorm_16(&self, index: impl std::slice::SliceIndex<[u32], Output = u32>) -> u16 {
        let value = f32::from_bits(self.components[index]);
        (value * 65535.0f32).round() as u16
    }

    pub fn to_bytes(&self, format: Format) -> Vec<u8> {
        let mut result = vec![0u8; format.bytes_per_pixel() as usize];
        match format {
            Format::R8Unorm => {
                result[0] = self.unorm_8(0);
            }
            Format::R8G8Unorm => {
                result[0] = self.unorm_8(0);
                result[1] = self.unorm_8(1);
            }
            Format::R8G8B8A8Unorm => {
                result[0] = self.unorm_8(0);
                result[1] = self.unorm_8(1);
                result[2] = self.unorm_8(2);
                result[3] = self.unorm_8(3);
            }
            Format::R32G32B32A32Sfloat => {
                result[0..4].copy_from_slice(&self.sfloat_32(0).to_ne_bytes());
                result[4..8].copy_from_slice(&self.sfloat_32(1).to_ne_bytes());
                result[8..12].copy_from_slice(&self.sfloat_32(2).to_ne_bytes());
                result[12..16].copy_from_slice(&self.sfloat_32(3).to_ne_bytes());
            }
            Format::A2b10g10r10UnormPack32 => {
                unimplemented!()
            }
            Format::D16Unorm => {
                result[0..2].copy_from_slice(&self.unorm_16(0).to_ne_bytes());
            }
        }
        result
    }

    pub fn from_bytes(format: Format, bytes: &[u8]) -> Self {
        let (s0, s1, s2, s3) = match format {
            Format::R8Unorm => (Some(0..1), None, None, None),
            Format::R8G8Unorm => (Some(0..1), Some(1..2), None, None),
            Format::R8G8B8A8Unorm | Format::R32G32B32A32Sfloat => {
                (Some(0..4), Some(4..8), Some(8..12), Some(12..16))
            }
            Format::A2b10g10r10UnormPack32 => todo!(),
            Format::D16Unorm => todo!(),
        };
        let f = |s: Range<usize>| {
            let range_len = s.len();
            let (bytes_start, bytes_end) = (s.start.min(bytes.len()), s.end.min(bytes.len()));
            let bytes_len = bytes_end - bytes_start;
            let mut raw = [0_u8; 4];
            if cfg!(target_endian = "big") {
                raw[4 - bytes_len..].copy_from_slice(&bytes[bytes_start..bytes_end]);
            } else {
                raw[..bytes_len].copy_from_slice(&bytes[bytes_start..bytes_end]);
            }
            match range_len {
                1 => u8::from_ne_bytes(
                    raw[..range_len]
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                ) as u32,
                2 => u16::from_ne_bytes(
                    raw[..range_len]
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                ) as u32,
                4 => u32::from_ne_bytes(
                    raw[..range_len]
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                ),
                _ => unreachable!("{:#?}", &raw[..range_len]),
            }
        };
        Self {
            components: [
                s0.map_or_else(|| 0, f),
                s1.map_or_else(|| 0, f),
                s2.map_or_else(|| 0, f),
                s3.map_or_else(|| 0, f),
            ],
        }
    }
}

impl Gpu {
    fn copy_bytes(
        &mut self,
        src_binding: &MemoryBinding,
        dst_binding: &MemoryBinding,
        src_offset: u64,
        dst_offset: u64,
        size: u64,
    ) {
        let [src, dst] = self
            .memory_allocations
            .get_many_mut([
                &MemoryBindingHandle(src_binding.memory_handle.load(Ordering::Relaxed)),
                &MemoryBindingHandle(dst_binding.memory_handle.load(Ordering::Relaxed)),
            ])
            .unwrap_or_else(|| unreachable!());
        let src = src[src_offset as usize..(src_offset + size) as usize].as_ptr();
        let dst = dst[dst_offset as usize..(dst_offset + size) as usize].as_mut_ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, size as usize);
        }
    }

    fn write_bytes(&mut self, src: &[u8], dst_binding: &MemoryBinding, dst_offset: u64) {
        let dst = self
            .memory_allocations
            .get_mut(&MemoryBindingHandle(
                dst_binding.memory_handle.load(Ordering::Relaxed),
            ))
            .unwrap_or_else(|| unreachable!());
        let size = src.len();
        let src = src.as_ptr();
        let dst = dst[dst_offset as usize..dst_offset as usize + size].as_mut_ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, size);
        }
    }

    fn read_bytes(&self, src_binding: &MemoryBinding, offset: u64, size: u64) -> &[u8] {
        let src = self
            .memory_allocations
            .get(&MemoryBindingHandle(
                src_binding.memory_handle.load(Ordering::Relaxed),
            ))
            .unwrap_or_else(|| unreachable!());
        let offset = offset as usize;
        let size = size as usize;
        &src[offset..offset + size]
    }
}
