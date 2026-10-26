#![allow(clippy::use_self)]

use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::num::NonZeroU64;
pub use std::ptr::NonNull;

/// A dispatchable Vulkan handle.
///
/// The Vulkan loader requires the first field of the pointed-to data to be
/// `VkLoaderData`; see <https://github.com/KhronosGroup/Vulkan-Loader/blob/main/docs/LoaderDriverInterface.md>.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct VkDispatchableHandle(pub Option<NonNull<VkDispatchableHandleInner>>);

impl Hash for VkDispatchableHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0
            .map_or(0, |handle| handle.as_ptr() as usize)
            .hash(state);
    }
}

// SAFETY: The handle is an opaque identifier with an immutable inner value.
unsafe impl Send for VkDispatchableHandle {}
// SAFETY: See the `Send` implementation above.
unsafe impl Sync for VkDispatchableHandle {}

#[repr(C)]
pub struct VkDispatchableHandleInner {
    pub loader_data: VkLoaderData,
    pub key: u64,
}

impl Debug for VkDispatchableHandleInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VkDispatchableHandleInner")
            .field("key", &self.key)
            .finish()
    }
}

/// First field of a dispatchable handle, written by the Vulkan loader.
#[repr(C)]
pub union VkLoaderData {
    pub loader_magic: usize,
    pub loader_data: Option<NonNull<std::ffi::c_void>>,
}

impl VkLoaderData {
    pub const LOADER_MAGIC: usize = 0x01CD_C0DE;
}

/// A non-dispatchable Vulkan handle.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct VkNonDispatchableHandle(pub Option<NonZeroU64>);

pub const VK_NULL_HANDLE: VkNonDispatchableHandle = VkNonDispatchableHandle(None);

pub type xcb_connection_t = xcb::ffi::xcb_connection_t;
pub type xcb_window_t = u32;

/// Placeholder for FFI types that the generated bindings do not model yet.
pub(crate) type VkUnsupportedType = *const std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/codegen_vk_decls.rs"));

impl From<VkFormat> for common::math::Format {
    fn from(value: VkFormat) -> Self {
        match value {
            VkFormat::VK_FORMAT_UNDEFINED => unimplemented!(),
            VkFormat::VK_FORMAT_R4G4_UNORM_PACK8 => unimplemented!(),
            VkFormat::VK_FORMAT_R4G4B4A4_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_B4G4R4A4_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_R5G6B5_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_B5G6R5_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_R5G5B5A1_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_B5G5R5A1_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_A1R5G5B5_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_R8_UNORM => Self::R8Unorm,
            VkFormat::VK_FORMAT_R8_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R8_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R8_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R8_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R8_SRGB => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8_UNORM => Self::R8G8Unorm,
            VkFormat::VK_FORMAT_R8G8_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8_SRGB => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8_SRGB => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8_SRGB => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8A8_UNORM => Self::R8G8B8A8Unorm,
            VkFormat::VK_FORMAT_R8G8B8A8_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8A8_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8A8_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8A8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R8G8B8A8_SINT => unimplemented!(),
            // sRGB conversion is not implemented; treat the format as UNORM.
            VkFormat::VK_FORMAT_R8G8B8A8_SRGB => Self::R8G8B8A8Unorm,
            VkFormat::VK_FORMAT_B8G8R8A8_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8A8_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8A8_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8A8_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8A8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8A8_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8A8_SRGB => unimplemented!(),
            VkFormat::VK_FORMAT_A8B8G8R8_UNORM_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A8B8G8R8_SNORM_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A8B8G8R8_USCALED_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A8B8G8R8_SSCALED_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A8B8G8R8_UINT_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A8B8G8R8_SINT_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A8B8G8R8_SRGB_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2R10G10B10_UNORM_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2R10G10B10_SNORM_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2R10G10B10_USCALED_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2R10G10B10_SSCALED_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2R10G10B10_UINT_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2R10G10B10_SINT_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2B10G10R10_UNORM_PACK32 => Self::A2b10g10r10UnormPack32,
            VkFormat::VK_FORMAT_A2B10G10R10_SNORM_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2B10G10R10_USCALED_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2B10G10R10_SSCALED_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2B10G10R10_UINT_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_A2B10G10R10_SINT_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_R16_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R16_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R16_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R16_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R16_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R16_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R16_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16A16_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16A16_SNORM => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16A16_USCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16A16_SSCALED => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16A16_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16A16_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R16G16B16A16_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R32_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R32_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R32_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32B32_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32B32_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32B32_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32B32A32_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32B32A32_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT => Self::R32G32B32A32Sfloat,
            VkFormat::VK_FORMAT_R64_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R64_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R64_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64B64_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64B64_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64B64_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64B64A64_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64B64A64_SINT => unimplemented!(),
            VkFormat::VK_FORMAT_R64G64B64A64_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_B10G11R11_UFLOAT_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_D16_UNORM => Self::D16Unorm,
            VkFormat::VK_FORMAT_X8_D24_UNORM_PACK32 => unimplemented!(),
            VkFormat::VK_FORMAT_D32_SFLOAT => unimplemented!(),
            VkFormat::VK_FORMAT_S8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_D16_UNORM_S8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_D24_UNORM_S8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT => unimplemented!(),
            VkFormat::VK_FORMAT_BC1_RGB_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC1_RGB_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC1_RGBA_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC1_RGBA_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC2_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC2_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC3_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC3_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC4_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC4_SNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC5_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC5_SNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC6H_UFLOAT_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC6H_SFLOAT_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC7_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_BC7_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_EAC_R11_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_EAC_R11_SNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_EAC_R11G11_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_EAC_R11G11_SNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_4x4_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_4x4_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_5x4_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_5x4_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_5x5_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_5x5_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_6x5_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_6x5_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_6x6_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_6x6_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_8x5_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_8x5_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_8x6_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_8x6_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_8x8_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_8x8_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_10x5_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_10x5_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_10x6_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_10x6_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_10x8_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_10x8_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_10x10_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_10x10_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_12x10_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_12x10_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_12x12_UNORM_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_ASTC_12x12_SRGB_BLOCK => unimplemented!(),
            VkFormat::VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG => unimplemented!(),
            VkFormat::VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG => unimplemented!(),
            VkFormat::VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG => unimplemented!(),
            VkFormat::VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG => unimplemented!(),
            VkFormat::VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG => unimplemented!(),
            VkFormat::VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG => unimplemented!(),
            VkFormat::VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG => unimplemented!(),
            VkFormat::VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG => unimplemented!(),
            VkFormat::VK_FORMAT_R10X6_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_R10X6G10X6_UNORM_2PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_R12X4_UNORM_PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_R12X4G12X4_UNORM_2PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G8B8G8R8_422_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_B8G8R8G8_422_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G16B16G16R16_422_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_B16G16R16G16_422_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G8_B8R8_2PLANE_420_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G16_B16R16_2PLANE_420_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G8_B8R8_2PLANE_422_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G16_B16R16_2PLANE_422_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM => unimplemented!(),
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => unimplemented!(),
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM => unimplemented!(),
            VkFormat(185_u32..=1000053999_u32)
            | VkFormat(1000054008_u32..=1000155999_u32)
            | VkFormat(1000156034_u32..=u32::MAX) => unreachable!(),
        }
    }
}

impl From<VkClearValue> for common::math::Color {
    fn from(value: VkClearValue) -> Self {
        // SAFETY: The `uint32` variant is valid for every clear value.
        let color = unsafe { value.color.uint32 };
        Self::from_raw(
            u64::from(color[0]),
            u64::from(color[1]),
            u64::from(color[2]),
            u64::from(color[3]),
        )
    }
}

impl From<VkPolygonMode> for common::graphics::PolygonMode {
    fn from(value: VkPolygonMode) -> Self {
        match value {
            VkPolygonMode::VK_POLYGON_MODE_FILL => Self::Fill,
            VkPolygonMode::VK_POLYGON_MODE_LINE => Self::Line,
            VkPolygonMode::VK_POLYGON_MODE_POINT => Self::Point,
            VkPolygonMode::VK_POLYGON_MODE_FILL_RECTANGLE_NV => Self::FillRectangle,
            _ => unreachable!(),
        }
    }
}

impl From<VkCullModeFlagBits> for common::graphics::CullMode {
    fn from(value: VkCullModeFlagBits) -> Self {
        match value {
            VkCullModeFlagBits::VK_CULL_MODE_NONE => Self::None,
            VkCullModeFlagBits::VK_CULL_MODE_FRONT_BIT => Self::Front,
            VkCullModeFlagBits::VK_CULL_MODE_BACK_BIT => Self::Back,
            VkCullModeFlagBits::VK_CULL_MODE_FRONT_AND_BACK => Self::FrontAndBack,
            _ => unreachable!("unknown VkCullModeFlagBits value: {value:?}"),
        }
    }
}

impl From<VkFrontFace> for common::graphics::FrontFace {
    fn from(value: VkFrontFace) -> Self {
        match value {
            VkFrontFace::VK_FRONT_FACE_COUNTER_CLOCKWISE => Self::CounterClockwise,
            VkFrontFace::VK_FRONT_FACE_CLOCKWISE => Self::Clockwise,
            _ => unreachable!(),
        }
    }
}

impl VkIndexType {
    pub const fn size_in_bytes(self) -> u8 {
        match self {
            Self::VK_INDEX_TYPE_NONE_KHR => 0,
            Self::VK_INDEX_TYPE_UINT8_EXT => 1,
            Self::VK_INDEX_TYPE_UINT16 => 2,
            Self::VK_INDEX_TYPE_UINT32 => 4,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_sizes_match_registry() {
        assert_eq!(VkFormat::VK_FORMAT_UNDEFINED.bytes_per_pixel(), 0);
        assert_eq!(VkFormat::VK_FORMAT_R8G8B8A8_UNORM.bytes_per_pixel(), 4);
        assert_eq!(VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT.bytes_per_pixel(), 5);
        assert_eq!(
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM.bytes_per_pixel(),
            6
        );
        assert_eq!(VkFormat::VK_FORMAT_BC1_RGB_UNORM_BLOCK.bytes_per_pixel(), 8);
    }

    #[test]
    fn feature_subset_check() {
        let supported = VkPhysicalDeviceFeatures {
            robustBufferAccess: VK_TRUE,
            ..Default::default()
        };
        assert!(supported.is_supported_by(&supported));
        assert!(
            !supported.is_supported_by(&VkPhysicalDeviceFeatures::default()),
            "a disabled feature must not support an enabled one"
        );
        assert!(VkPhysicalDeviceFeatures::default().is_supported_by(&supported));
    }
}
