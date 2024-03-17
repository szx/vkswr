use std::ops::{Index, IndexMut};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use crate::consts::{MAX_VERTEX_ATTRIBUTES, MAX_VERTEX_BINDINGS};
use crate::math::{Extent3, Format};

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

#[derive(Debug, Clone)]
pub struct DescriptorBuffer {
    pub binding: MemoryBinding,
}

#[derive(Debug, Clone)]
pub struct DescriptorImage {
    pub binding: MemoryBinding,
    pub extent: Extent3<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryBinding {
    /// Thanks to Arc cloned resource binding points to the same MemoryAllocation
    pub memory_handle: Arc<AtomicU64>,
    pub offset: u64,
    pub size: u64,
}

impl MemoryBinding {
    pub fn new() -> Self {
        Self {
            memory_handle: Arc::new(Default::default()),
            offset: 0,
            size: 0,
        }
    }
}
