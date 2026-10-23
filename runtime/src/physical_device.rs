//! PhysicalDevice

use crate::context::VulkanObject;
use crate::pipeline::ShaderModule;
use common::consts::{
    MAX_VERTEX_ATTRIBUTE_OFFSET, MAX_VERTEX_ATTRIBUTES, MAX_VERTEX_BINDING_STRIDE,
    MAX_VERTEX_BINDINGS, MAX_VIEWPORT_DIMENSIONS, MAX_VIEWPORTS, VIEWPORT_BOUNDS_RANGE,
};
use common::graphics::{
    VertexAttribute, VertexBinding, VertexBindingNumber, VertexInputRate, VertexInputState,
};
use common::math::{Extent2, Offset2, Range2};
use gpu::{
    InputAssemblyState, PrimitiveTopology, RasterizationState, RenderArea, Scissor, Viewport,
    ViewportState,
};
use headers::c_char_array;
use headers::vk_decls::*;
use log::info;
use shader::glsl::{Shader, ShaderState};
use std::fmt::{Debug, Formatter};

/// Performs rendering operations.
pub struct PhysicalDevice {
    pub(crate) handle: VkDispatchableHandle,
    physical_device_name: &'static str,
    pub(crate) gpu: gpu::Gpu,
}

impl Debug for PhysicalDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhysicalDevice")
            .field("handle", &self.handle)
            .field("physical_device_name", &self.physical_device_name)
            .finish()
    }
}

/// Device-local and host-visible memory types exposed to Vulkan clients.
const MEMORY_TYPES: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize] = {
    let mut memory_types = [VkMemoryType {
        propertyFlags: 0,
        heapIndex: 0,
    }; VK_MAX_MEMORY_TYPES as usize];
    memory_types[0] = VkMemoryType {
        propertyFlags: VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
            | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0,
        heapIndex: 0,
    };
    memory_types[1] = VkMemoryType {
        propertyFlags: VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0,
        heapIndex: 1,
    };
    memory_types
};

/// Memory heaps backing [`MEMORY_TYPES`].
const MEMORY_HEAPS: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize] = {
    let mut memory_heaps = [VkMemoryHeap { size: 0, flags: 0 }; VK_MAX_MEMORY_HEAPS as usize];
    let heap_size = gpu::Memory::memory_size_in_bytes() / 2;
    memory_heaps[0] = VkMemoryHeap {
        size: heap_size,
        flags: 0,
    };
    memory_heaps[1] = VkMemoryHeap {
        size: heap_size,
        flags: VkMemoryHeapFlagBits::VK_MEMORY_HEAP_DEVICE_LOCAL_BIT.0,
    };
    memory_heaps
};

// The format table below advertises a deliberately small set of features.
// Naming the individual bits once keeps the table readable and makes each
// combination self-documenting.
const SAMPLED_IMAGE_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT.0;
const STORAGE_IMAGE_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT.0;
const STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT.0;
const UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT.0;
const STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT.0;
const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT.0;
const VERTEX_BUFFER_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT.0;
const COLOR_ATTACHMENT_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT.0;
const COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT.0;
const DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT.0;
const BLIT_SRC_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT.0;
const BLIT_DST_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT.0;
const FILTER_LINEAR_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT.0;
const TRANSFER_SRC_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT.0;
const TRANSFER_DST_BIT: VkFormatFeatureFlags =
    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT.0;

const TRANSFER: VkFormatFeatureFlags = TRANSFER_SRC_BIT | TRANSFER_DST_BIT;

/// Sampled, linearly filtered, blittable, and transferable.
const SAMPLED_BLIT_TRANSFER: VkFormatFeatureFlags =
    SAMPLED_IMAGE_BIT | BLIT_SRC_BIT | FILTER_LINEAR_BIT | TRANSFER;

/// Sampled color attachments that can also be blitted and transferred.
const SAMPLED_COLOR_ATTACHMENT: VkFormatFeatureFlags =
    SAMPLED_IMAGE_BIT | COLOR_ATTACHMENT_BIT | BLIT_SRC_BIT | BLIT_DST_BIT | TRANSFER;

/// Blendable sampled color attachments that can be blitted and transferred.
const SAMPLED_BLENDABLE_COLOR_ATTACHMENT: VkFormatFeatureFlags = SAMPLED_IMAGE_BIT
    | COLOR_ATTACHMENT_BIT
    | COLOR_ATTACHMENT_BLEND_BIT
    | BLIT_SRC_BIT
    | BLIT_DST_BIT
    | FILTER_LINEAR_BIT
    | TRANSFER;

/// Sampled storage images that are also color attachments.
const SAMPLED_STORAGE_COLOR_ATTACHMENT: VkFormatFeatureFlags = SAMPLED_IMAGE_BIT
    | STORAGE_IMAGE_BIT
    | COLOR_ATTACHMENT_BIT
    | BLIT_SRC_BIT
    | BLIT_DST_BIT
    | TRANSFER;

/// Blendable color attachments that are also sampled storage images.
const SAMPLED_STORAGE_BLENDABLE_COLOR_ATTACHMENT: VkFormatFeatureFlags = SAMPLED_IMAGE_BIT
    | STORAGE_IMAGE_BIT
    | COLOR_ATTACHMENT_BIT
    | COLOR_ATTACHMENT_BLEND_BIT
    | BLIT_SRC_BIT
    | BLIT_DST_BIT
    | FILTER_LINEAR_BIT
    | TRANSFER;

/// Color attachments with atomic storage image support.
const SAMPLED_STORAGE_ATOMIC_COLOR_ATTACHMENT: VkFormatFeatureFlags = SAMPLED_IMAGE_BIT
    | STORAGE_IMAGE_BIT
    | STORAGE_IMAGE_ATOMIC_BIT
    | COLOR_ATTACHMENT_BIT
    | BLIT_SRC_BIT
    | BLIT_DST_BIT
    | TRANSFER;

/// Sampled storage images that can also be blitted.
const SAMPLED_STORAGE_BLIT_TRANSFER: VkFormatFeatureFlags =
    SAMPLED_IMAGE_BIT | STORAGE_IMAGE_BIT | BLIT_SRC_BIT | FILTER_LINEAR_BIT | TRANSFER;

/// Sampled depth/stencil formats that can be blitted.
const SAMPLED_DEPTH_STENCIL_TRANSFER: VkFormatFeatureFlags =
    SAMPLED_IMAGE_BIT | DEPTH_STENCIL_ATTACHMENT_BIT | BLIT_SRC_BIT | TRANSFER;

const UNIFORM_VERTEX_BUFFER: VkFormatFeatureFlags = UNIFORM_TEXEL_BUFFER_BIT | VERTEX_BUFFER_BIT;
const UNIFORM_STORAGE_VERTEX_BUFFER: VkFormatFeatureFlags =
    UNIFORM_TEXEL_BUFFER_BIT | STORAGE_TEXEL_BUFFER_BIT | VERTEX_BUFFER_BIT;
const UNIFORM_STORAGE_ATOMIC_VERTEX_BUFFER: VkFormatFeatureFlags = UNIFORM_TEXEL_BUFFER_BIT
    | STORAGE_TEXEL_BUFFER_BIT
    | STORAGE_TEXEL_BUFFER_ATOMIC_BIT
    | VERTEX_BUFFER_BIT;
const VERTEX_BUFFER: VkFormatFeatureFlags = VERTEX_BUFFER_BIT;
const UNIFORM_TEXEL_BUFFER: VkFormatFeatureFlags = UNIFORM_TEXEL_BUFFER_BIT;
const NO_FORMAT_FEATURES: VkFormatFeatureFlags = 0;

/// Builds the [`VkFormatProperties`] advertised for a format.
const fn format_features(
    optimal_tiling_features: VkFormatFeatureFlags,
    buffer_features: VkFormatFeatureFlags,
) -> VkFormatProperties {
    VkFormatProperties {
        linearTilingFeatures: NO_FORMAT_FEATURES,
        optimalTilingFeatures: optimal_tiling_features,
        bufferFeatures: buffer_features,
    }
}

/// Format properties for formats that are not usable at all.
const UNSUPPORTED_FORMAT: VkFormatProperties =
    format_features(NO_FORMAT_FEATURES, NO_FORMAT_FEATURES);

/// All device features disabled.
///
/// `VkPhysicalDeviceFeatures::default()` is not `const`, but
/// [`PhysicalDevice::features`] is a `const fn`, so spell the zero value out
/// here.
const NO_DEVICE_FEATURES: VkPhysicalDeviceFeatures = {
    // SAFETY: `VkPhysicalDeviceFeatures` is a `#[repr(C)]` plain-old-data
    // struct whose fields are all `VkBool32`, for which zero is valid.
    unsafe { std::mem::zeroed() }
};

/// Returns whether every feature requested in `$enabled` is in `$supported`.
///
/// This mirrors the Vulkan rule that an application may only enable a feature
/// the physical device advertises.
macro_rules! all_features_supported {
    ($supported:expr, $enabled:expr, [$first:ident $(, $rest:ident)* $(,)?]) => {
        ($enabled.$first != VK_TRUE || $supported.$first != VK_FALSE)
            $(&& ($enabled.$rest != VK_TRUE || $supported.$rest != VK_FALSE))*
    };
}

impl PhysicalDevice {
    pub fn create() -> VkDispatchableHandle {
        info!("new PhysicalDevice");
        let physical_device = Self {
            handle: VkDispatchableHandle(None),
            physical_device_name: "VkSWR physical device",
            gpu: gpu::Gpu::default(),
        };
        physical_device.register()
    }

    pub fn extension_count() -> usize {
        Self::extension_properties().len()
    }

    pub fn extension_properties() -> [VkExtensionProperties; 3] {
        c_char_array!(
            VK_KHR_SWAPCHAIN_EXTENSION_NAME,
            VK_MAX_EXTENSION_NAME_SIZE,
            "VK_KHR_swapchain"
        );
        c_char_array!(
            VK_KHR_TOOLING_INFO_NAME,
            VK_MAX_EXTENSION_NAME_SIZE,
            "VK_EXT_tooling_info"
        );
        c_char_array!(
            VK_KHR_DEBUG_MARKER_NAME,
            VK_MAX_EXTENSION_NAME_SIZE,
            "VK_EXT_debug_marker"
        );
        [
            VkExtensionProperties {
                extensionName: VK_KHR_SWAPCHAIN_EXTENSION_NAME,
                specVersion: 70,
            },
            VkExtensionProperties {
                extensionName: VK_KHR_TOOLING_INFO_NAME,
                specVersion: 1,
            },
            VkExtensionProperties {
                extensionName: VK_KHR_DEBUG_MARKER_NAME,
                specVersion: 4,
            },
        ]
    }

    pub fn properties(&self) -> VkPhysicalDeviceProperties {
        c_char_array!(
            DEVICE_NAME,
            VK_MAX_PHYSICAL_DEVICE_NAME_SIZE,
            "VkSWR physical device"
        );

        VkPhysicalDeviceProperties {
            apiVersion: 1 << 22, // Vulkan 1.0
            driverVersion: 1,
            deviceName: DEVICE_NAME,
            pipelineCacheUUID: [
                0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0x10, 0x11, 0x12, 0x13, 0x14,
                0x15,
            ],
            limits: VkPhysicalDeviceLimits {
                maxVertexInputAttributes: MAX_VERTEX_ATTRIBUTES,
                maxVertexInputBindings: MAX_VERTEX_BINDINGS,
                maxVertexInputAttributeOffset: MAX_VERTEX_ATTRIBUTE_OFFSET,
                maxVertexInputBindingStride: MAX_VERTEX_BINDING_STRIDE,
                maxViewports: MAX_VIEWPORTS,
                maxViewportDimensions: [MAX_VIEWPORT_DIMENSIONS.0, MAX_VIEWPORT_DIMENSIONS.1],
                viewportBoundsRange: [VIEWPORT_BOUNDS_RANGE.0, VIEWPORT_BOUNDS_RANGE.1],
                nonCoherentAtomSize: 1,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub const fn tool_properties(&self) -> [VkPhysicalDeviceToolProperties; 0] {
        []
    }

    pub const fn memory_properties(&self) -> VkPhysicalDeviceMemoryProperties {
        VkPhysicalDeviceMemoryProperties {
            memoryTypeCount: 2,
            memoryTypes: MEMORY_TYPES,
            memoryHeapCount: 2,
            memoryHeaps: MEMORY_HEAPS,
        }
    }

    pub const fn memory_type_bits_for_buffer(&self) -> u32 {
        (1 << 0) | (1 << 1)
    }

    pub const fn memory_type_bits_for_image(&self) -> u32 {
        (1 << 0) | (1 << 1)
    }

    pub const fn features(&self) -> VkPhysicalDeviceFeatures {
        VkPhysicalDeviceFeatures {
            textureCompressionETC2: VK_TRUE,
            textureCompressionASTC_LDR: VK_TRUE,
            textureCompressionBC: VK_TRUE,
            ..NO_DEVICE_FEATURES
        }
    }

    pub const fn supports_features(&self, enabled_features: &VkPhysicalDeviceFeatures) -> bool {
        let supported_features = self.features();
        all_features_supported!(
            supported_features,
            enabled_features,
            [
                robustBufferAccess,
                fullDrawIndexUint32,
                imageCubeArray,
                independentBlend,
                geometryShader,
                tessellationShader,
                sampleRateShading,
                dualSrcBlend,
                logicOp,
                multiDrawIndirect,
                drawIndirectFirstInstance,
                depthClamp,
                depthBiasClamp,
                fillModeNonSolid,
                depthBounds,
                wideLines,
                largePoints,
                alphaToOne,
                multiViewport,
                samplerAnisotropy,
                textureCompressionETC2,
                textureCompressionASTC_LDR,
                textureCompressionBC,
                occlusionQueryPrecise,
                pipelineStatisticsQuery,
                vertexPipelineStoresAndAtomics,
                fragmentStoresAndAtomics,
                shaderTessellationAndGeometryPointSize,
                shaderImageGatherExtended,
                shaderStorageImageExtendedFormats,
                shaderStorageImageMultisample,
                shaderStorageImageReadWithoutFormat,
                shaderStorageImageWriteWithoutFormat,
                shaderUniformBufferArrayDynamicIndexing,
                shaderSampledImageArrayDynamicIndexing,
                shaderStorageBufferArrayDynamicIndexing,
                shaderStorageImageArrayDynamicIndexing,
                shaderClipDistance,
                shaderCullDistance,
                shaderFloat64,
                shaderInt64,
                shaderInt16,
                shaderResourceResidency,
                shaderResourceMinLod,
                sparseBinding,
                sparseResidencyBuffer,
                sparseResidencyImage2D,
                sparseResidencyImage3D,
                sparseResidency2Samples,
                sparseResidency4Samples,
                sparseResidency8Samples,
                sparseResidency16Samples,
                sparseResidencyAliased,
                variableMultisampleRate,
                inheritedQueries,
            ]
        )
    }

    pub const fn format_properties(&self, format: VkFormat) -> VkFormatProperties {
        match format {
            // Sampled, linearly filtered, blittable formats with no buffer features.
            VkFormat::VK_FORMAT_B4G4R4A4_UNORM_PACK16
            | VkFormat::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32
            | VkFormat::VK_FORMAT_BC1_RGB_UNORM_BLOCK
            | VkFormat::VK_FORMAT_BC1_RGB_SRGB_BLOCK
            | VkFormat::VK_FORMAT_BC1_RGBA_UNORM_BLOCK
            | VkFormat::VK_FORMAT_BC1_RGBA_SRGB_BLOCK
            | VkFormat::VK_FORMAT_BC2_UNORM_BLOCK
            | VkFormat::VK_FORMAT_BC2_SRGB_BLOCK
            | VkFormat::VK_FORMAT_BC3_UNORM_BLOCK
            | VkFormat::VK_FORMAT_BC3_SRGB_BLOCK
            | VkFormat::VK_FORMAT_BC4_UNORM_BLOCK
            | VkFormat::VK_FORMAT_BC4_SNORM_BLOCK
            | VkFormat::VK_FORMAT_BC5_UNORM_BLOCK
            | VkFormat::VK_FORMAT_BC5_SNORM_BLOCK
            | VkFormat::VK_FORMAT_BC6H_UFLOAT_BLOCK
            | VkFormat::VK_FORMAT_BC6H_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_BC7_UNORM_BLOCK
            | VkFormat::VK_FORMAT_BC7_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK
            | VkFormat::VK_FORMAT_EAC_R11_UNORM_BLOCK
            | VkFormat::VK_FORMAT_EAC_R11_SNORM_BLOCK
            | VkFormat::VK_FORMAT_EAC_R11G11_UNORM_BLOCK
            | VkFormat::VK_FORMAT_EAC_R11G11_SNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_4x4_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_4x4_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_5x4_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_5x4_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_5x5_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_5x5_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_6x5_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_6x5_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_6x6_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_6x6_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x5_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x5_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x6_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x6_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x8_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x8_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x5_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x5_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x6_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x6_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x8_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x8_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x10_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x10_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_12x10_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_12x10_SRGB_BLOCK
            | VkFormat::VK_FORMAT_ASTC_12x12_UNORM_BLOCK
            | VkFormat::VK_FORMAT_ASTC_12x12_SRGB_BLOCK => {
                format_features(SAMPLED_BLIT_TRANSFER, NO_FORMAT_FEATURES)
            }

            // Sampled, blendable color attachments with no buffer features.
            VkFormat::VK_FORMAT_R5G6B5_UNORM_PACK16
            | VkFormat::VK_FORMAT_A1R5G5B5_UNORM_PACK16
            | VkFormat::VK_FORMAT_R8G8B8A8_SRGB
            | VkFormat::VK_FORMAT_B8G8R8A8_SRGB
            | VkFormat::VK_FORMAT_A8B8G8R8_SRGB_PACK32 => {
                format_features(SAMPLED_BLENDABLE_COLOR_ATTACHMENT, NO_FORMAT_FEATURES)
            }

            // Sampled, blendable color attachments usable as texel and vertex buffers.
            VkFormat::VK_FORMAT_R8_UNORM
            | VkFormat::VK_FORMAT_R8G8_UNORM
            | VkFormat::VK_FORMAT_B8G8R8A8_UNORM
            | VkFormat::VK_FORMAT_A2B10G10R10_UNORM_PACK32
            | VkFormat::VK_FORMAT_R16_SFLOAT
            | VkFormat::VK_FORMAT_R16G16_SFLOAT => {
                format_features(SAMPLED_BLENDABLE_COLOR_ATTACHMENT, UNIFORM_VERTEX_BUFFER)
            }

            // Sampled, linearly filtered, blittable formats usable as texel and vertex buffers.
            VkFormat::VK_FORMAT_R8_SNORM | VkFormat::VK_FORMAT_R8G8_SNORM => {
                format_features(SAMPLED_BLIT_TRANSFER, UNIFORM_VERTEX_BUFFER)
            }

            // Sampled color attachments usable as texel and vertex buffers.
            VkFormat::VK_FORMAT_R8_UINT
            | VkFormat::VK_FORMAT_R8_SINT
            | VkFormat::VK_FORMAT_R8G8_UINT
            | VkFormat::VK_FORMAT_R8G8_SINT
            | VkFormat::VK_FORMAT_R16_UINT
            | VkFormat::VK_FORMAT_R16_SINT
            | VkFormat::VK_FORMAT_R16G16_UINT
            | VkFormat::VK_FORMAT_R16G16_SINT => {
                format_features(SAMPLED_COLOR_ATTACHMENT, UNIFORM_VERTEX_BUFFER)
            }

            // Sampled, blendable color attachments with storage image and texel buffer support.
            VkFormat::VK_FORMAT_R8G8B8A8_UNORM | VkFormat::VK_FORMAT_R16G16B16A16_SFLOAT => {
                format_features(
                    SAMPLED_STORAGE_BLENDABLE_COLOR_ATTACHMENT,
                    UNIFORM_STORAGE_VERTEX_BUFFER,
                )
            }

            // Sampled storage images usable as texel and vertex buffers.
            VkFormat::VK_FORMAT_R8G8B8A8_SNORM => {
                format_features(SAMPLED_STORAGE_BLIT_TRANSFER, UNIFORM_STORAGE_VERTEX_BUFFER)
            }

            // Sampled storage images usable as color attachments and texel buffers.
            VkFormat::VK_FORMAT_R8G8B8A8_UINT
            | VkFormat::VK_FORMAT_R8G8B8A8_SINT
            | VkFormat::VK_FORMAT_R16G16B16A16_UINT
            | VkFormat::VK_FORMAT_R16G16B16A16_SINT
            | VkFormat::VK_FORMAT_R32_SFLOAT
            | VkFormat::VK_FORMAT_R32G32_UINT
            | VkFormat::VK_FORMAT_R32G32_SINT
            | VkFormat::VK_FORMAT_R32G32_SFLOAT
            | VkFormat::VK_FORMAT_R32G32B32A32_UINT
            | VkFormat::VK_FORMAT_R32G32B32A32_SINT
            | VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT => format_features(
                SAMPLED_STORAGE_COLOR_ATTACHMENT,
                UNIFORM_STORAGE_VERTEX_BUFFER,
            ),

            // Sampled, blendable color attachments with texel buffer support.
            VkFormat::VK_FORMAT_A8B8G8R8_UNORM_PACK32 => format_features(
                SAMPLED_BLENDABLE_COLOR_ATTACHMENT,
                UNIFORM_STORAGE_VERTEX_BUFFER,
            ),

            // Sampled, linearly filtered, blittable formats with texel buffer support.
            VkFormat::VK_FORMAT_A8B8G8R8_SNORM_PACK32 => {
                format_features(SAMPLED_BLIT_TRANSFER, UNIFORM_STORAGE_VERTEX_BUFFER)
            }

            // Sampled color attachments with texel buffer support.
            VkFormat::VK_FORMAT_A8B8G8R8_UINT_PACK32 | VkFormat::VK_FORMAT_A8B8G8R8_SINT_PACK32 => {
                format_features(SAMPLED_COLOR_ATTACHMENT, UNIFORM_STORAGE_VERTEX_BUFFER)
            }

            // Sampled color attachments usable as uniform texel buffers.
            VkFormat::VK_FORMAT_A2B10G10R10_UINT_PACK32 => {
                format_features(SAMPLED_COLOR_ATTACHMENT, UNIFORM_TEXEL_BUFFER)
            }

            // Vertex-buffer-only formats.
            VkFormat::VK_FORMAT_R16_UNORM
            | VkFormat::VK_FORMAT_R16_SNORM
            | VkFormat::VK_FORMAT_R16G16_UNORM
            | VkFormat::VK_FORMAT_R16G16_SNORM
            | VkFormat::VK_FORMAT_R32G32B32_UINT
            | VkFormat::VK_FORMAT_R32G32B32_SINT
            | VkFormat::VK_FORMAT_R32G32B32_SFLOAT => {
                format_features(NO_FORMAT_FEATURES, VERTEX_BUFFER)
            }

            // Uniform texel and vertex buffer formats.
            VkFormat::VK_FORMAT_R16G16B16A16_UNORM | VkFormat::VK_FORMAT_R16G16B16A16_SNORM => {
                format_features(NO_FORMAT_FEATURES, UNIFORM_VERTEX_BUFFER)
            }

            // Atomic storage image and color attachment formats with texel buffer support.
            VkFormat::VK_FORMAT_R32_UINT | VkFormat::VK_FORMAT_R32_SINT => format_features(
                SAMPLED_STORAGE_ATOMIC_COLOR_ATTACHMENT,
                UNIFORM_STORAGE_ATOMIC_VERTEX_BUFFER,
            ),

            // Sampled, linearly filtered, blittable formats usable as uniform texel buffers.
            VkFormat::VK_FORMAT_B10G11R11_UFLOAT_PACK32 => {
                format_features(SAMPLED_BLIT_TRANSFER, UNIFORM_TEXEL_BUFFER)
            }

            // Sampled depth/stencil formats that can be blitted.
            VkFormat::VK_FORMAT_D16_UNORM => {
                format_features(SAMPLED_DEPTH_STENCIL_TRANSFER, NO_FORMAT_FEATURES)
            }

            // Depth/stencil attachment formats.
            VkFormat::VK_FORMAT_X8_D24_UNORM_PACK32
            | VkFormat::VK_FORMAT_D32_SFLOAT
            | VkFormat::VK_FORMAT_D24_UNORM_S8_UINT
            | VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT => {
                format_features(DEPTH_STENCIL_ATTACHMENT_BIT, NO_FORMAT_FEATURES)
            }

            // Formats without any advertised tiling or buffer features.
            VkFormat::VK_FORMAT_UNDEFINED
            | VkFormat::VK_FORMAT_R4G4_UNORM_PACK8
            | VkFormat::VK_FORMAT_R4G4B4A4_UNORM_PACK16
            | VkFormat::VK_FORMAT_B5G6R5_UNORM_PACK16
            | VkFormat::VK_FORMAT_R5G5B5A1_UNORM_PACK16
            | VkFormat::VK_FORMAT_B5G5R5A1_UNORM_PACK16
            | VkFormat::VK_FORMAT_R8_USCALED
            | VkFormat::VK_FORMAT_R8_SSCALED
            | VkFormat::VK_FORMAT_R8_SRGB
            | VkFormat::VK_FORMAT_R8G8_USCALED
            | VkFormat::VK_FORMAT_R8G8_SSCALED
            | VkFormat::VK_FORMAT_R8G8_SRGB
            | VkFormat::VK_FORMAT_R8G8B8_UNORM
            | VkFormat::VK_FORMAT_R8G8B8_SNORM
            | VkFormat::VK_FORMAT_R8G8B8_USCALED
            | VkFormat::VK_FORMAT_R8G8B8_SSCALED
            | VkFormat::VK_FORMAT_R8G8B8_UINT
            | VkFormat::VK_FORMAT_R8G8B8_SINT
            | VkFormat::VK_FORMAT_R8G8B8_SRGB
            | VkFormat::VK_FORMAT_B8G8R8_UNORM
            | VkFormat::VK_FORMAT_B8G8R8_SNORM
            | VkFormat::VK_FORMAT_B8G8R8_USCALED
            | VkFormat::VK_FORMAT_B8G8R8_SSCALED
            | VkFormat::VK_FORMAT_B8G8R8_UINT
            | VkFormat::VK_FORMAT_B8G8R8_SINT
            | VkFormat::VK_FORMAT_B8G8R8_SRGB
            | VkFormat::VK_FORMAT_R8G8B8A8_USCALED
            | VkFormat::VK_FORMAT_R8G8B8A8_SSCALED
            | VkFormat::VK_FORMAT_B8G8R8A8_SNORM
            | VkFormat::VK_FORMAT_B8G8R8A8_USCALED
            | VkFormat::VK_FORMAT_B8G8R8A8_SSCALED
            | VkFormat::VK_FORMAT_B8G8R8A8_UINT
            | VkFormat::VK_FORMAT_B8G8R8A8_SINT
            | VkFormat::VK_FORMAT_A8B8G8R8_USCALED_PACK32
            | VkFormat::VK_FORMAT_A8B8G8R8_SSCALED_PACK32
            | VkFormat::VK_FORMAT_A2R10G10B10_UNORM_PACK32
            | VkFormat::VK_FORMAT_A2R10G10B10_SNORM_PACK32
            | VkFormat::VK_FORMAT_A2R10G10B10_USCALED_PACK32
            | VkFormat::VK_FORMAT_A2R10G10B10_SSCALED_PACK32
            | VkFormat::VK_FORMAT_A2R10G10B10_UINT_PACK32
            | VkFormat::VK_FORMAT_A2R10G10B10_SINT_PACK32
            | VkFormat::VK_FORMAT_A2B10G10R10_SNORM_PACK32
            | VkFormat::VK_FORMAT_A2B10G10R10_USCALED_PACK32
            | VkFormat::VK_FORMAT_A2B10G10R10_SSCALED_PACK32
            | VkFormat::VK_FORMAT_A2B10G10R10_SINT_PACK32
            | VkFormat::VK_FORMAT_R16_USCALED
            | VkFormat::VK_FORMAT_R16_SSCALED
            | VkFormat::VK_FORMAT_R16G16_USCALED
            | VkFormat::VK_FORMAT_R16G16_SSCALED
            | VkFormat::VK_FORMAT_R16G16B16_UNORM
            | VkFormat::VK_FORMAT_R16G16B16_SNORM
            | VkFormat::VK_FORMAT_R16G16B16_USCALED
            | VkFormat::VK_FORMAT_R16G16B16_SSCALED
            | VkFormat::VK_FORMAT_R16G16B16_UINT
            | VkFormat::VK_FORMAT_R16G16B16_SINT
            | VkFormat::VK_FORMAT_R16G16B16_SFLOAT
            | VkFormat::VK_FORMAT_R16G16B16A16_USCALED
            | VkFormat::VK_FORMAT_R16G16B16A16_SSCALED
            | VkFormat::VK_FORMAT_R64_UINT
            | VkFormat::VK_FORMAT_R64_SINT
            | VkFormat::VK_FORMAT_R64_SFLOAT
            | VkFormat::VK_FORMAT_R64G64_UINT
            | VkFormat::VK_FORMAT_R64G64_SINT
            | VkFormat::VK_FORMAT_R64G64_SFLOAT
            | VkFormat::VK_FORMAT_R64G64B64_UINT
            | VkFormat::VK_FORMAT_R64G64B64_SINT
            | VkFormat::VK_FORMAT_R64G64B64_SFLOAT
            | VkFormat::VK_FORMAT_R64G64B64A64_UINT
            | VkFormat::VK_FORMAT_R64G64B64A64_SINT
            | VkFormat::VK_FORMAT_R64G64B64A64_SFLOAT
            | VkFormat::VK_FORMAT_S8_UINT
            | VkFormat::VK_FORMAT_D16_UNORM_S8_UINT
            | VkFormat::VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG
            | VkFormat::VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG
            | VkFormat::VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG
            | VkFormat::VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG
            | VkFormat::VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG
            | VkFormat::VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG
            | VkFormat::VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG
            | VkFormat::VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG
            | VkFormat::VK_FORMAT_R10X6_UNORM_PACK16
            | VkFormat::VK_FORMAT_R10X6G10X6_UNORM_2PACK16
            | VkFormat::VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16
            | VkFormat::VK_FORMAT_R12X4_UNORM_PACK16
            | VkFormat::VK_FORMAT_R12X4G12X4_UNORM_2PACK16
            | VkFormat::VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16
            | VkFormat::VK_FORMAT_G8B8G8R8_422_UNORM
            | VkFormat::VK_FORMAT_B8G8R8G8_422_UNORM
            | VkFormat::VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
            | VkFormat::VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
            | VkFormat::VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
            | VkFormat::VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
            | VkFormat::VK_FORMAT_G16B16G16R16_422_UNORM
            | VkFormat::VK_FORMAT_B16G16R16G16_422_UNORM
            | VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM
            | VkFormat::VK_FORMAT_G8_B8R8_2PLANE_420_UNORM
            | VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM
            | VkFormat::VK_FORMAT_G16_B16R16_2PLANE_420_UNORM
            | VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM
            | VkFormat::VK_FORMAT_G8_B8R8_2PLANE_422_UNORM
            | VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM
            | VkFormat::VK_FORMAT_G16_B16R16_2PLANE_422_UNORM
            | VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM
            | VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM
            | VkFormat::VK_FORMAT_G8_B8R8_2PLANE_444_UNORM
            | VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16
            | VkFormat::VK_FORMAT_G16_B16R16_2PLANE_444_UNORM
            | VkFormat::VK_FORMAT_A4R4G4B4_UNORM_PACK16
            | VkFormat::VK_FORMAT_A4B4G4R4_UNORM_PACK16
            | VkFormat::VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK
            | VkFormat::VK_FORMAT_R16G16_S10_5_NV => UNSUPPORTED_FORMAT,
            // Formats added to Vulkan after this crate's registry snapshot.
            _ => UNSUPPORTED_FORMAT,
        }
    }

    fn supports_image_usage(
        &self,
        format: VkFormat,
        tiling: VkImageTiling,
        usage: VkImageUsageFlags,
    ) -> bool {
        let properties = self.format_properties(format);
        let features = match tiling {
            VkImageTiling::VK_IMAGE_TILING_LINEAR => properties.linearTilingFeatures,
            VkImageTiling::VK_IMAGE_TILING_OPTIMAL => properties.optimalTilingFeatures,
            _ => return false,
        };
        let color_or_depth = COLOR_ATTACHMENT_BIT | DEPTH_STENCIL_ATTACHMENT_BIT;
        let checks = [
            (
                VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT.0,
                SAMPLED_IMAGE_BIT,
            ),
            (
                VkImageUsageFlagBits::VK_IMAGE_USAGE_STORAGE_BIT.0,
                STORAGE_IMAGE_BIT,
            ),
            (
                VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT.0,
                COLOR_ATTACHMENT_BIT,
            ),
            (
                VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT.0,
                DEPTH_STENCIL_ATTACHMENT_BIT,
            ),
            (
                VkImageUsageFlagBits::VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT.0,
                color_or_depth,
            ),
        ];
        checks.iter().all(|&(usage_bit, feature_bits)| {
            usage & usage_bit == 0 || features & feature_bits != 0
        })
    }

    pub fn image_format_properties(
        &self,
        format: VkFormat,
        type_: VkImageType,
        tiling: VkImageTiling,
        usage: VkImageUsageFlags,
        flags: VkImageCreateFlags,
    ) -> Option<VkImageFormatProperties> {
        if !self.supports_image_usage(format, tiling, usage) {
            return None;
        }

        let is_cube_compatible =
            (flags & VkImageCreateFlagBits::VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT.0) != 0;

        let max_extent = match type_ {
            VkImageType::VK_IMAGE_TYPE_1D => VkExtent3D {
                width: 16384, // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimension1D (2).
                height: 1,
                depth: 1,
            },
            VkImageType::VK_IMAGE_TYPE_2D if is_cube_compatible => VkExtent3D {
                width: 16384,  // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimensionCube (5).
                height: 16384, // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimensionCube (6).
                depth: 1,
            },
            VkImageType::VK_IMAGE_TYPE_2D => VkExtent3D {
                width: 16384,  // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimension2D (3).
                height: 16384, // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimension2D (4).
                depth: 1,
            },
            VkImageType::VK_IMAGE_TYPE_3D => VkExtent3D {
                width: 16384,  // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimension3D (7).
                height: 16384, // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimension3D (8).
                depth: 16384, // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimension3D (9).
            },
            _ => return None,
        };

        match format {
            VkFormat::VK_FORMAT_R8G8B8A8_UNORM => Some(VkImageFormatProperties {
                maxExtent: max_extent,
                maxMipLevels: 1,
                maxArrayLayers: 1, // TODO: VkPhysicalDeviceLimits::maxImageArrayLayers
                sampleCounts: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT.into(),
                maxResourceSize: 1_u64 << 31, // TODO: VK_ERROR_OUT_OF_DEVICE_MEMORY
            }),
            _ => None,
        }
    }

    pub fn queue_family_properties(&self) -> [VkQueueFamilyProperties; 1] {
        // SPEC: If an implementation exposes any queue family that supports graphics operations,
        // at least one queue family of at least one physical device exposed by the implementation
        // must support both graphics and compute operations.
        [VkQueueFamilyProperties {
            queueFlags: (VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT
                | VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT)
                .into(),
            queueCount: 1,
            ..Default::default()
        }]
    }

    pub const fn surface_support(&self, queue_family_index: u32, _surface: VkSurfaceKHR) -> bool {
        queue_family_index == 0
    }

    pub const fn present_modes(&self) -> [VkPresentModeKHR; 1] {
        [VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR]
    }

    pub const fn surface_formats(&self) -> [VkSurfaceFormatKHR; 2] {
        [
            VkSurfaceFormatKHR {
                format: VkFormat::VK_FORMAT_R8G8B8A8_UNORM,
                colorSpace: VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            },
            VkSurfaceFormatKHR {
                format: VkFormat::VK_FORMAT_R8G8B8A8_SRGB,
                colorSpace: VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            },
        ]
    }

    pub fn surface_capabilities(&self) -> VkSurfaceCapabilitiesKHR {
        VkSurfaceCapabilitiesKHR {
            minImageCount: 1,
            maxImageCount: 2,
            currentExtent: VkExtent2D {
                width: u32::MAX,
                height: u32::MAX,
            },
            maxImageExtent: VkExtent2D {
                width: 16384, // TODO: Replace with maxImageDimension2D (1).
                height: 16384,
            },
            maxImageArrayLayers: 1,
            supportedTransforms:
                VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR.into(),
            currentTransform: VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
            supportedCompositeAlpha: VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR
                .into(),
            supportedUsageFlags: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT.into(),
            ..Default::default()
        }
    }
}

impl PhysicalDevice {
    /// Converts a Vulkan vertex input state into the renderer's representation.
    ///
    /// # Safety
    ///
    /// The pointers in `vertex_input_state` must be valid for their respective
    /// counts, as required by the Vulkan specification.
    pub unsafe fn parse_vertex_input_state(
        vertex_input_state: VkPipelineVertexInputStateCreateInfo,
    ) -> VertexInputState {
        let vk_attributes =
            vertex_input_state
                .pVertexAttributeDescriptions
                .map_or(&[] as &[_], |descriptions| {
                    let count = usize::try_from(vertex_input_state.vertexAttributeDescriptionCount)
                        .expect("vertex attribute description count fits in usize");
                    // SAFETY: The caller guarantees that `pVertexAttributeDescriptions`
                    // points to `vertexAttributeDescriptionCount` valid descriptions.
                    unsafe { std::slice::from_raw_parts(descriptions.as_ptr(), count) }
                });
        let vk_bindings =
            vertex_input_state
                .pVertexBindingDescriptions
                .map_or(&[] as &[_], |descriptions| {
                    let count = usize::try_from(vertex_input_state.vertexBindingDescriptionCount)
                        .expect("vertex binding description count fits in usize");
                    // SAFETY: The caller guarantees that `pVertexBindingDescriptions`
                    // points to `vertexBindingDescriptionCount` valid descriptions.
                    unsafe { std::slice::from_raw_parts(descriptions.as_ptr(), count) }
                });
        let mut vertex_input_state = VertexInputState::default();
        for vk_attribute in vk_attributes {
            let location = usize::try_from(vk_attribute.location)
                .expect("vertex attribute location fits in usize");
            let attribute = vertex_input_state
                .attributes
                .get_mut(location)
                .expect("vertex attribute location is within MAX_VERTEX_ATTRIBUTES");
            *attribute = Some(VertexAttribute {
                location: vk_attribute.location,
                binding: VertexBindingNumber(vk_attribute.binding),
                format: vk_attribute.format.into(),
                offset: vk_attribute.offset,
            });
        }
        for vk_binding in vk_bindings {
            let binding_number =
                usize::try_from(vk_binding.binding).expect("vertex binding number fits in usize");
            let binding = vertex_input_state
                .bindings
                .get_mut(binding_number)
                .expect("vertex binding number is within MAX_VERTEX_BINDINGS");
            *binding = Some(VertexBinding {
                number: VertexBindingNumber(vk_binding.binding),
                stride: vk_binding.stride,
                input_rate: Self::parse_vertex_input_rate(vk_binding.inputRate),
            });
        }
        vertex_input_state
    }

    pub(crate) fn parse_vertex_input_rate(vertex_input_rate: VkVertexInputRate) -> VertexInputRate {
        match vertex_input_rate {
            VkVertexInputRate::VK_VERTEX_INPUT_RATE_VERTEX => VertexInputRate::Vertex,
            VkVertexInputRate::VK_VERTEX_INPUT_RATE_INSTANCE => VertexInputRate::Instance,
            _ => panic!("unsupported vertex input rate: {vertex_input_rate:?}"),
        }
    }

    pub fn parse_input_assembly_state(
        input_assembly_state: VkPipelineInputAssemblyStateCreateInfo,
    ) -> InputAssemblyState {
        InputAssemblyState {
            topology: Self::parse_primitive_topology(input_assembly_state.topology),
            primitive_restart: input_assembly_state.primitiveRestartEnable != 0,
        }
    }

    pub(crate) fn parse_primitive_topology(topology: VkPrimitiveTopology) -> PrimitiveTopology {
        match topology {
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_POINT_LIST => PrimitiveTopology::PointList,
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_LINE_LIST => PrimitiveTopology::LineList,
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP => PrimitiveTopology::LineStrip,
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST => {
                PrimitiveTopology::TriangleList
            }
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP => {
                PrimitiveTopology::TriangleStrip
            }
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN => {
                PrimitiveTopology::TriangleFan
            }
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY => {
                PrimitiveTopology::LineListWithAdjacency
            }
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY => {
                PrimitiveTopology::LineStripWithAdjacency
            }
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY => {
                PrimitiveTopology::TriangleListWithAdjacency
            }
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY => {
                PrimitiveTopology::TriangleStripWithAdjacency
            }
            VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_PATCH_LIST => PrimitiveTopology::PatchList,
            _ => panic!("unsupported primitive topology: {topology:?}"),
        }
    }

    /// Converts a Vulkan viewport state into the renderer's representation.
    ///
    /// # Safety
    ///
    /// The pointers in `viewport_state` must be valid for their respective
    /// counts, as required by the Vulkan specification.
    pub unsafe fn parse_viewport_state(
        viewport_state: VkPipelineViewportStateCreateInfo,
    ) -> ViewportState {
        let vk_viewports = viewport_state.pViewports.map_or(&[] as &[_], |viewports| {
            let count = usize::try_from(viewport_state.viewportCount)
                .expect("viewport count fits in usize");
            // SAFETY: The caller guarantees that `pViewports` points to
            // `viewportCount` valid viewports.
            unsafe { std::slice::from_raw_parts(viewports.as_ptr(), count) }
        });
        let vk_scissors = viewport_state.pScissors.map_or(&[] as &[_], |scissors| {
            let count =
                usize::try_from(viewport_state.scissorCount).expect("scissor count fits in usize");
            // SAFETY: The caller guarantees that `pScissors` points to
            // `scissorCount` valid scissors.
            unsafe { std::slice::from_raw_parts(scissors.as_ptr(), count) }
        });

        let mut viewport_state = ViewportState::default();
        for (index, vk_viewport) in vk_viewports.iter().enumerate() {
            let viewport = viewport_state
                .viewports
                .get_mut(index)
                .expect("viewport index is within MAX_VIEWPORTS");
            *viewport = Some(Viewport {
                offset: Offset2 {
                    x: vk_viewport.x,
                    y: vk_viewport.y,
                },
                extent: Extent2 {
                    width: vk_viewport.width,
                    height: vk_viewport.height,
                },
                depth: Range2 {
                    min: vk_viewport.minDepth,
                    max: vk_viewport.maxDepth,
                },
            });
        }
        for (index, vk_scissor) in vk_scissors.iter().enumerate() {
            let scissor = viewport_state
                .scissors
                .get_mut(index)
                .expect("scissor index is within MAX_VIEWPORTS");
            *scissor = Some(Scissor {
                render_area: RenderArea {
                    extent: Extent2 {
                        width: vk_scissor.extent.width,
                        height: vk_scissor.extent.height,
                    },
                    offset: Offset2 {
                        x: vk_scissor.offset.x,
                        y: vk_scissor.offset.y,
                    },
                },
            });
        }
        viewport_state
    }

    pub fn parse_rasterization_state(
        rasterization_state: VkPipelineRasterizationStateCreateInfo,
    ) -> RasterizationState {
        RasterizationState {
            depth_clamp_enable: rasterization_state.depthClampEnable != 0,
            rasterizer_discard_enable: rasterization_state.rasterizerDiscardEnable != 0,
            polygon_mode: rasterization_state.polygonMode.into(),
            cull_mode: VkCullModeFlagBits::from(rasterization_state.cullMode).into(),
            front_face: rasterization_state.frontFace.into(),
            depth_bias_enable: rasterization_state.depthBiasEnable != 0,
            depth_bias_constant_factor: rasterization_state.depthBiasConstantFactor,
            depth_bias_clamp: rasterization_state.depthBiasClamp,
            depth_bias_slope_factor: rasterization_state.depthBiasSlopeFactor,
            line_width: rasterization_state.lineWidth,
        }
    }

    pub fn parse_shader_stages(
        shader_stages: &[VkPipelineShaderStageCreateInfo],
    ) -> Result<ShaderState, VkResult> {
        let mut shader_state = ShaderState::default();
        for shader_stage in shader_stages {
            assert_eq!(
                shader_stage.flags, 0,
                "shader stage flags are not supported"
            );
            let name = shader_stage
                .pName
                .expect("shader stage must have an entry point name");
            // SAFETY: Vulkan requires `pName` to point to a null-terminated
            // string that remains valid for the duration of the call.
            let name = unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) };
            let name = name
                .to_str()
                .map_err(|_| VkResult::VK_ERROR_INVALID_SHADER_NV)?
                .to_owned();
            assert!(
                shader_stage.pSpecializationInfo.is_none(),
                "specialization info is not supported"
            );
            let module = ShaderModule::from_handle(shader_stage.module)
                .expect("shader module handle is registered");
            let code = module.lock().code.clone();

            let shader =
                Shader::new(&name, code).map_err(|_| VkResult::VK_ERROR_INVALID_SHADER_NV)?;

            match shader_stage.stage {
                VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT => {
                    shader_state.vertex_shader = Some(shader);
                }
                VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT => {
                    shader_state.fragment_shader = Some(shader);
                }
                stage => panic!("unsupported shader stage: {stage:?}"),
            }
        }
        Ok(shader_state)
    }
}
