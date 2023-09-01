use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub enum Format {
    R8Unorm,
    R8G8Unorm,
    R8G8B8A8Unorm,
    R32G32B32A32Sfloat,
    A2b10g10r10UnormPack32,
    D16Unorm,
}

pub struct FormatInfo {
    pub bytes_per_pixel: u8,
    pub bytes_per_component: Option<u8>,
    pub bytes0: Option<Range<usize>>,
    pub bytes1: Option<Range<usize>>,
    pub bytes2: Option<Range<usize>>,
    pub bytes3: Option<Range<usize>>,
    pub is_unorm: bool,
}

impl Format {
    pub const fn info(&self) -> FormatInfo {
        match *self {
            Format::R8Unorm => FormatInfo {
                bytes_per_pixel: 1,
                bytes_per_component: Some(1),
                bytes0: Some(0..1),
                bytes1: None,
                bytes2: None,
                bytes3: None,
                is_unorm: true,
            },
            Format::R8G8Unorm => FormatInfo {
                bytes_per_pixel: 2,
                bytes_per_component: Some(1),
                bytes0: Some(0..1),
                bytes1: Some(1..2),
                bytes2: None,
                bytes3: None,
                is_unorm: true,
            },
            Format::R8G8B8A8Unorm => FormatInfo {
                bytes_per_pixel: 4,
                bytes_per_component: Some(1),
                bytes0: Some(0..1),
                bytes1: Some(1..2),
                bytes2: Some(2..3),
                bytes3: Some(3..4),
                is_unorm: true,
            },
            Format::R32G32B32A32Sfloat => FormatInfo {
                bytes_per_pixel: 16,
                bytes_per_component: Some(4),
                bytes0: Some(0..4),
                bytes1: Some(4..8),
                bytes2: Some(8..12),
                bytes3: Some(12..16),
                is_unorm: false,
            },
            Format::A2b10g10r10UnormPack32 => FormatInfo {
                bytes_per_pixel: 4,
                bytes_per_component: None,
                bytes0: todo!(),
                bytes1: todo!(),
                bytes2: todo!(),
                bytes3: todo!(),
                is_unorm: false,
            },
            Format::D16Unorm => FormatInfo {
                bytes_per_pixel: 2,
                bytes_per_component: Some(2),
                bytes0: Some(0..2),
                bytes1: None,
                bytes2: None,
                bytes3: None,
                is_unorm: false,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Vector4 {
    /// Bit representation of components.
    components: [u64; 4],
}

impl Vector4 {
    pub const fn from_raw(r: u64, g: u64, b: u64, a: u64) -> Self {
        Self {
            components: [r, g, b, a],
        }
    }

    pub fn from_sfloat32_raw(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            components: [
                f32::to_bits(r) as u64,
                f32::to_bits(g) as u64,
                f32::to_bits(b) as u64,
                f32::to_bits(a) as u64,
            ],
        }
    }

    pub fn from_sfloat64_raw(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            components: [
                f64::to_bits(r),
                f64::to_bits(g),
                f64::to_bits(b),
                f64::to_bits(a),
            ],
        }
    }

    pub fn from_sfloat32(v: Vector4) -> Self {
        Self::from_sfloat32_raw(
            v.get_as_sfloat32(0),
            v.get_as_sfloat32(1),
            v.get_as_sfloat32(2),
            v.get_as_sfloat32(3),
        )
    }

    pub fn from_sfloat64(v: Vector4) -> Self {
        Self::from_sfloat64_raw(
            v.get_as_sfloat64(0),
            v.get_as_sfloat64(1),
            v.get_as_sfloat64(2),
            v.get_as_sfloat64(3),
        )
    }

    pub fn get_as_sfloat32(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> f32 {
        f32::from_bits(self.components[index] as u32)
    }

    pub fn get_as_sfloat64(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> f64 {
        f64::from_bits(self.components[index])
    }

    pub fn get_as_unorm8(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> f32 {
        // NOTE: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-fixedfpconv
        let c = self.components[index] as f32;
        let divisor = (2_u32.pow(8) - 1) as f32;
        c / divisor
    }

    pub fn get_as_unorm32(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> f32 {
        // NOTE: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-fixedfpconv
        let c = self.components[index] as f32;
        let divisor = (2_u32.pow(32) - 1) as f32;
        c / divisor
    }

    pub fn get_as_uint8(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> u8 {
        self.components[index] as u8
    }

    pub fn get_as_uint32(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> u32 {
        self.components[index] as u32
    }

    pub fn get_as_uint64(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> u64 {
        self.components[index]
    }

    pub fn to_sfloat32_bytes(
        &self,
        index: impl std::slice::SliceIndex<[u64], Output = u64>,
    ) -> [u8; 4] {
        f32::from_bits(self.components[index] as u32).to_ne_bytes()
    }

    pub fn to_unorm8_byte(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> u8 {
        let value = f32::from_bits(self.components[index] as u32);
        (value * 255.0f32).round() as u8
    }

    pub fn to_unorm16_bytes(
        &self,
        index: impl std::slice::SliceIndex<[u64], Output = u64>,
    ) -> [u8; 2] {
        let value = f32::from_bits(self.components[index] as u32);
        let value = (value * 65535.0f32).round() as u16;
        value.to_ne_bytes()
    }

    pub fn to_unorm32_bytes(
        &self,
        index: impl std::slice::SliceIndex<[u64], Output = u64>,
    ) -> [u8; 4] {
        let value = f32::from_bits(self.components[index] as u32);
        let value = (value * 4294967295.0f32).round() as u32;
        value.to_ne_bytes()
    }

    pub fn to_bytes(&self, format: Format) -> Vec<u8> {
        let mut result = vec![0u8; format.info().bytes_per_pixel as usize];
        match format {
            Format::R8Unorm => {
                result[0] = self.to_unorm8_byte(0);
            }
            Format::R8G8Unorm => {
                result[0] = self.to_unorm8_byte(0);
                result[1] = self.to_unorm8_byte(1);
            }
            Format::R8G8B8A8Unorm => {
                result[0] = self.to_unorm8_byte(0);
                result[1] = self.to_unorm8_byte(1);
                result[2] = self.to_unorm8_byte(2);
                result[3] = self.to_unorm8_byte(3);
            }
            Format::R32G32B32A32Sfloat => {
                result[0..4].copy_from_slice(&self.to_sfloat32_bytes(0));
                result[4..8].copy_from_slice(&self.to_sfloat32_bytes(1));
                result[8..12].copy_from_slice(&self.to_sfloat32_bytes(2));
                result[12..16].copy_from_slice(&self.to_sfloat32_bytes(3));
            }
            Format::A2b10g10r10UnormPack32 => {
                unimplemented!()
            }
            Format::D16Unorm => {
                result[0..2].copy_from_slice(&self.to_unorm16_bytes(0));
            }
        }
        result
    }

    pub fn from_vertex_buffer_bytes(format: Format, bytes: &[u8]) -> Self {
        struct FormatInfo {
            bytes0: Option<Range<usize>>,
            bytes1: Option<Range<usize>>,
            bytes2: Option<Range<usize>>,
            bytes3: Option<Range<usize>>,
            is_unorm: bool,
        }
        let (s0, s1, s2, s3) = (
            format.info().bytes0,
            format.info().bytes1,
            format.info().bytes2,
            format.info().bytes3,
        );
        let f = |s: Range<usize>| {
            let range_len = s.len();
            let (bytes_start, bytes_end) = (s.start.min(bytes.len()), s.end.min(bytes.len()));
            let bytes_len = bytes_end - bytes_start;
            let mut raw = [0_u8; std::mem::size_of::<u64>()];
            if cfg!(target_endian = "big") {
                let size = raw.len();
                raw[size - bytes_len..].copy_from_slice(&bytes[bytes_start..bytes_end]);
            } else {
                raw[..bytes_len].copy_from_slice(&bytes[bytes_start..bytes_end]);
            }
            match range_len {
                1 => u8::from_ne_bytes(
                    raw[..range_len]
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                ) as u64,
                2 => u16::from_ne_bytes(
                    raw[..range_len]
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                ) as u64,
                4 => u32::from_ne_bytes(
                    raw[..range_len]
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                ) as u64,
                8 => u64::from_ne_bytes(
                    raw[..range_len]
                        .try_into()
                        .unwrap_or_else(|_| unreachable!()),
                ),
                _ => unreachable!("{:#?}", &raw[..range_len]),
            }
        };
        let result = Self {
            components: [
                s0.map_or_else(|| 0, f),
                s1.map_or_else(|| 0, f),
                s2.map_or_else(|| 0, f),
                s3.map_or_else(|| 0, f),
            ],
        };
        if format.info().is_unorm {
            match format.info().bytes_per_component {
                Some(1) => result.to_unorm8(),
                Some(4) => result.to_unorm32(),
                _ => unimplemented!(),
            }
        } else {
            result
        }
    }

    pub fn to_unorm8(self) -> Self {
        Self::from_sfloat32_raw(
            self.get_as_unorm8(0),
            self.get_as_unorm8(1),
            self.get_as_unorm8(2),
            self.get_as_unorm8(3),
        )
    }

    pub fn to_unorm32(self) -> Self {
        Self::from_sfloat32_raw(
            self.get_as_unorm32(0),
            self.get_as_unorm32(1),
            self.get_as_unorm32(2),
            self.get_as_unorm32(3),
        )
    }
}

pub type Texel = Vector4;
pub type Color = Vector4;
pub type Position = Vector4;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Position,
    pub index: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Fragment {
    pub position: Position,
    pub color: Color,
}
