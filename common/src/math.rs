//! Format descriptions and the vector type shared by the shader and rasterizer.

use std::fmt::{Debug, Formatter};
use std::ops::Range;

/// Pixel formats understood by the software rasterizer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    R8Unorm,
    R8G8Unorm,
    R8G8B8A8Unorm,
    R32G32B32A32Sfloat,
    A2b10g10r10UnormPack32,
    D16Unorm,
}

/// Layout description of a [`Format`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatInfo {
    pub bytes_per_pixel: u8,
    pub bytes_per_component: Option<u8>,
    /// Byte range of each component inside a pixel, if the component exists.
    pub component_bytes: [Option<Range<usize>>; 4],
    pub is_unorm: bool,
}

impl FormatInfo {
    /// Byte range of `component` inside a pixel.
    #[must_use]
    pub fn component(&self, component: usize) -> Option<&Range<usize>> {
        self.component_bytes.get(component)?.as_ref()
    }
}

impl Format {
    /// Returns the layout of this format.
    #[must_use]
    pub const fn info(self) -> FormatInfo {
        match self {
            Self::R8Unorm => FormatInfo {
                bytes_per_pixel: 1,
                bytes_per_component: Some(1),
                component_bytes: [Some(0..1), None, None, None],
                is_unorm: true,
            },
            Self::R8G8Unorm => FormatInfo {
                bytes_per_pixel: 2,
                bytes_per_component: Some(1),
                component_bytes: [Some(0..1), Some(1..2), None, None],
                is_unorm: true,
            },
            Self::R8G8B8A8Unorm => FormatInfo {
                bytes_per_pixel: 4,
                bytes_per_component: Some(1),
                component_bytes: [Some(0..1), Some(1..2), Some(2..3), Some(3..4)],
                is_unorm: true,
            },
            Self::R32G32B32A32Sfloat => FormatInfo {
                bytes_per_pixel: 16,
                bytes_per_component: Some(4),
                component_bytes: [Some(0..4), Some(4..8), Some(8..12), Some(12..16)],
                is_unorm: false,
            },
            // TODO: Implement packed 10/10/10/2 formats.
            Self::A2b10g10r10UnormPack32 => FormatInfo {
                bytes_per_pixel: 4,
                bytes_per_component: None,
                component_bytes: [None, None, None, None],
                is_unorm: false,
            },
            Self::D16Unorm => FormatInfo {
                bytes_per_pixel: 2,
                bytes_per_component: Some(2),
                component_bytes: [Some(0..2), None, None, None],
                is_unorm: false,
            },
        }
    }
}

/// Four-component vector stored as raw 64-bit words.
///
/// The raw representation lets a single type carry integer, fixed-point, and
/// floating-point values without knowing the shader type that produced it.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Vector4 {
    /// Bit representation of each component.
    pub components: [u64; 4],
}

impl Vector4 {
    #[must_use]
    pub const fn from_raw(r: u64, g: u64, b: u64, a: u64) -> Self {
        Self {
            components: [r, g, b, a],
        }
    }

    #[must_use]
    pub const fn from_sfloat32_raw(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            components: [
                r.to_bits() as u64,
                g.to_bits() as u64,
                b.to_bits() as u64,
                a.to_bits() as u64,
            ],
        }
    }

    #[must_use]
    pub const fn from_sfloat64_raw(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            components: [r.to_bits(), g.to_bits(), b.to_bits(), a.to_bits()],
        }
    }

    /// Reinterprets the components of `value` as 32-bit floats.
    #[must_use]
    pub const fn from_sfloat32(value: Self) -> Self {
        Self::from_sfloat32_raw(
            value.get_as_sfloat32(0),
            value.get_as_sfloat32(1),
            value.get_as_sfloat32(2),
            value.get_as_sfloat32(3),
        )
    }

    /// Reinterprets the components of `value` as 64-bit floats.
    #[must_use]
    pub const fn from_sfloat64(value: Self) -> Self {
        Self::from_sfloat64_raw(
            value.get_as_sfloat64(0),
            value.get_as_sfloat64(1),
            value.get_as_sfloat64(2),
            value.get_as_sfloat64(3),
        )
    }

    #[must_use]
    pub const fn get_as_sfloat32(&self, index: usize) -> f32 {
        f32::from_bits(self.components[index] as u32)
    }

    #[must_use]
    pub const fn get_as_sfloat64(&self, index: usize) -> f64 {
        f64::from_bits(self.components[index])
    }

    /// Converts an 8-bit unsigned normalized component to float.
    ///
    /// See <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-fixedfpconv>.
    #[must_use]
    pub fn get_as_unorm8(&self, index: usize) -> f32 {
        self.components[index] as f32 / f32::from(u8::MAX)
    }

    /// Converts a 32-bit unsigned normalized component to float.
    ///
    /// See <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-fixedfpconv>.
    #[must_use]
    pub fn get_as_unorm32(&self, index: usize) -> f32 {
        self.components[index] as f32 / u32::MAX as f32
    }

    #[must_use]
    pub const fn get_as_uint8(&self, index: usize) -> u8 {
        self.components[index] as u8
    }

    #[must_use]
    pub const fn get_as_uint32(&self, index: usize) -> u32 {
        self.components[index] as u32
    }

    #[must_use]
    pub const fn get_as_uint64(&self, index: usize) -> u64 {
        self.components[index]
    }

    #[must_use]
    pub fn get_as_f32_array(&self) -> [f32; 4] {
        std::array::from_fn(|index| self.get_as_sfloat32(index))
    }

    const fn to_sfloat32_bytes(self, index: usize) -> [u8; 4] {
        f32::from_bits(self.components[index] as u32).to_ne_bytes()
    }

    fn to_unorm8_byte(self, index: usize) -> u8 {
        (self.get_as_sfloat32(index) * f32::from(u8::MAX)).round() as u8
    }

    fn to_unorm16_bytes(self, index: usize) -> [u8; 2] {
        ((self.get_as_sfloat32(index) * f32::from(u16::MAX)).round() as u16).to_ne_bytes()
    }

    #[allow(dead_code)]
    fn to_unorm32_bytes(self, index: usize) -> [u8; 4] {
        ((self.get_as_sfloat32(index) * u32::MAX as f32).round() as u32).to_ne_bytes()
    }

    /// Serializes the vector into `format`.
    ///
    /// # Panics
    ///
    /// Panics for formats that are not implemented yet.
    #[must_use]
    pub fn to_bytes(&self, format: Format) -> Vec<u8> {
        let mut result = vec![0; format.info().bytes_per_pixel as usize];
        match format {
            Format::R8Unorm => result[0] = self.to_unorm8_byte(0),
            Format::R8G8Unorm => {
                result[0] = self.to_unorm8_byte(0);
                result[1] = self.to_unorm8_byte(1);
            }
            Format::R8G8B8A8Unorm => {
                for (byte, index) in result.iter_mut().zip(0..) {
                    *byte = self.to_unorm8_byte(index);
                }
            }
            Format::R32G32B32A32Sfloat => {
                for (bytes, index) in result.chunks_exact_mut(4).zip(0..) {
                    bytes.copy_from_slice(&self.to_sfloat32_bytes(index));
                }
            }
            Format::A2b10g10r10UnormPack32 => unimplemented!("packed 10/10/10/2 formats"),
            Format::D16Unorm => result[0..2].copy_from_slice(&self.to_unorm16_bytes(0)),
        }
        result
    }

    /// Reads a vertex attribute from `bytes` according to `format`.
    #[must_use]
    pub fn from_vertex_buffer_bytes(format: Format, bytes: &[u8]) -> Self {
        let info = format.info();
        let mut components = [0; 4];
        for (component, range) in components.iter_mut().zip(info.component_bytes) {
            if let Some(range) = range {
                *component = read_component(bytes, range);
            }
        }
        let result = Self { components };
        match info.bytes_per_component {
            Some(1) if info.is_unorm => result.to_unorm8(),
            Some(4) if info.is_unorm => result.to_unorm32(),
            _ => result,
        }
    }

    #[must_use]
    pub fn to_unorm8(self) -> Self {
        Self::from_sfloat32_raw(
            self.get_as_unorm8(0),
            self.get_as_unorm8(1),
            self.get_as_unorm8(2),
            self.get_as_unorm8(3),
        )
    }

    #[must_use]
    pub fn to_unorm32(self) -> Self {
        Self::from_sfloat32_raw(
            self.get_as_unorm32(0),
            self.get_as_unorm32(1),
            self.get_as_unorm32(2),
            self.get_as_unorm32(3),
        )
    }
}

fn read_component(bytes: &[u8], range: Range<usize>) -> u64 {
    let start = range.start.min(bytes.len());
    let end = range.end.min(bytes.len());
    let bytes = &bytes[start..end];
    match bytes.len() {
        1 => u64::from(bytes[0]),
        2 => u64::from(u16::from_ne_bytes(
            bytes.try_into().expect("slice has the expected length"),
        )),
        4 => u64::from(u32::from_ne_bytes(
            bytes.try_into().expect("slice has the expected length"),
        )),
        8 => u64::from_ne_bytes(bytes.try_into().expect("slice has the expected length")),
        length => panic!("unsupported component size: {length}"),
    }
}

impl Debug for Vector4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector4")
            .field("x", &self.get_as_sfloat32(0))
            .field("y", &self.get_as_sfloat32(1))
            .field("z", &self.get_as_sfloat32(2))
            .field("w", &self.get_as_sfloat32(3))
            .finish()
    }
}

pub type Texel = Vector4;
pub type Color = Vector4;
pub type Position = Vector4;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Position,
    pub point_size: f32,
    pub index: u32,
    pub clip_distances: [f32; crate::consts::MAX_CLIP_DISTANCES as usize],
}

#[derive(Debug, Copy, Clone)]
pub struct Fragment {
    pub position: Position,
    pub color: Color,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Offset3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Offset2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Extent3<T> {
    pub width: T,
    pub height: T,
    pub depth: T,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Extent2<T> {
    pub width: T,
    pub height: T,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Range2<T> {
    pub min: T,
    pub max: T,
}
