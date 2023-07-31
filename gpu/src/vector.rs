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

#[derive(Debug, Copy, Clone)]
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

    pub fn from_sfloat32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            components: [
                f32::to_bits(r) as u64,
                f32::to_bits(g) as u64,
                f32::to_bits(b) as u64,
                f32::to_bits(a) as u64,
            ],
        }
    }

    pub fn from_sfloat_64(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            components: [
                f64::to_bits(r) as u64,
                f64::to_bits(g) as u64,
                f64::to_bits(b) as u64,
                f64::to_bits(a) as u64,
            ],
        }
    }

    pub fn sfloat32(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> f32 {
        f32::from_bits(self.components[index] as u32)
    }

    pub fn unorm8(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> u8 {
        let value = f32::from_bits(self.components[index] as u32);
        (value * 255.0f32).round() as u8
    }

    pub fn unorm16(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> u16 {
        let value = f32::from_bits(self.components[index] as u32);
        (value * 65535.0f32).round() as u16
    }

    pub fn unorm32(&self, index: impl std::slice::SliceIndex<[u64], Output = u64>) -> u64 {
        let value = f64::from_bits(self.components[index]);
        (value * 4294967295.0f64).round() as u64
    }

    pub fn to_bytes(&self, format: Format) -> Vec<u8> {
        let mut result = vec![0u8; format.bytes_per_pixel() as usize];
        match format {
            Format::R8Unorm => {
                result[0] = self.unorm8(0);
            }
            Format::R8G8Unorm => {
                result[0] = self.unorm8(0);
                result[1] = self.unorm8(1);
            }
            Format::R8G8B8A8Unorm => {
                result[0] = self.unorm8(0);
                result[1] = self.unorm8(1);
                result[2] = self.unorm8(2);
                result[3] = self.unorm8(3);
            }
            Format::R32G32B32A32Sfloat => {
                result[0..4].copy_from_slice(&self.sfloat32(0).to_ne_bytes());
                result[4..8].copy_from_slice(&self.sfloat32(1).to_ne_bytes());
                result[8..12].copy_from_slice(&self.sfloat32(2).to_ne_bytes());
                result[12..16].copy_from_slice(&self.sfloat32(3).to_ne_bytes());
            }
            Format::A2b10g10r10UnormPack32 => {
                unimplemented!()
            }
            Format::D16Unorm => {
                result[0..2].copy_from_slice(&self.unorm16(0).to_ne_bytes());
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
                16 => u64::from_ne_bytes(
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

pub type Vertex = Vector4;
pub type Texel = Vector4;
pub type Color = Vector4;
pub type Position = Vector4;

#[derive(Debug, Copy, Clone)]
pub struct Fragment {
    pub position: Position,
    pub color: Color,
}
