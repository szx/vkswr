//! PhysicalDevice

use crate::context::{Dispatchable, NonDispatchable};
use crate::pipeline::ShaderModule;
use common::consts::{
    MAX_VERTEX_ATTRIBUTES, MAX_VERTEX_ATTRIBUTE_OFFSET, MAX_VERTEX_BINDINGS,
    MAX_VERTEX_BINDING_STRIDE, MAX_VIEWPORTS, MAX_VIEWPORT_DIMENSIONS, VIEWPORT_BOUNDS_RANGE,
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
use lazy_static::lazy_static;
use log::*;
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

impl PhysicalDevice {
    pub fn create() -> VkDispatchableHandle {
        info!("new PhysicalDevice");
        let physical_device = Self {
            handle: VkDispatchableHandle(None),
            physical_device_name: "VkSWR physical device",
            gpu: gpu::Gpu::new(),
        };
        physical_device.register_object()
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
                extensionName: *VK_KHR_SWAPCHAIN_EXTENSION_NAME,
                specVersion: 70,
            },
            VkExtensionProperties {
                extensionName: *VK_KHR_TOOLING_INFO_NAME,
                specVersion: 1,
            },
            VkExtensionProperties {
                extensionName: *VK_KHR_DEBUG_MARKER_NAME,
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
            apiVersion: (0 << 29) | (1 << 22) | (0 << 12) | 0, // variant + major + minor + path
            driverVersion: 1,
            vendorID: 0,
            deviceID: 0,
            deviceType: VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_OTHER,
            deviceName: *DEVICE_NAME,
            pipelineCacheUUID: [
                0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0x10, 0x11, 0x12, 0x13, 0x14,
                0x15,
            ],
            limits: VkPhysicalDeviceLimits {
                maxImageDimension1D: 0,
                maxImageDimension2D: 0,
                maxImageDimension3D: 0,
                maxImageDimensionCube: 0,
                maxImageArrayLayers: 0,
                maxTexelBufferElements: 0,
                maxUniformBufferRange: 0,
                maxStorageBufferRange: 0,
                maxPushConstantsSize: 0,
                maxMemoryAllocationCount: 0,
                maxSamplerAllocationCount: 0,
                bufferImageGranularity: 0,
                sparseAddressSpaceSize: 0,
                maxBoundDescriptorSets: 0,
                maxPerStageDescriptorSamplers: 0,
                maxPerStageDescriptorUniformBuffers: 0,
                maxPerStageDescriptorStorageBuffers: 0,
                maxPerStageDescriptorSampledImages: 0,
                maxPerStageDescriptorStorageImages: 0,
                maxPerStageDescriptorInputAttachments: 0,
                maxPerStageResources: 0,
                maxDescriptorSetSamplers: 0,
                maxDescriptorSetUniformBuffers: 0,
                maxDescriptorSetUniformBuffersDynamic: 0,
                maxDescriptorSetStorageBuffers: 0,
                maxDescriptorSetStorageBuffersDynamic: 0,
                maxDescriptorSetSampledImages: 0,
                maxDescriptorSetStorageImages: 0,
                maxDescriptorSetInputAttachments: 0,
                maxVertexInputAttributes: MAX_VERTEX_ATTRIBUTES,
                maxVertexInputBindings: MAX_VERTEX_BINDINGS,
                maxVertexInputAttributeOffset: MAX_VERTEX_ATTRIBUTE_OFFSET,
                maxVertexInputBindingStride: MAX_VERTEX_BINDING_STRIDE,
                maxVertexOutputComponents: 0,
                maxTessellationGenerationLevel: 0,
                maxTessellationPatchSize: 0,
                maxTessellationControlPerVertexInputComponents: 0,
                maxTessellationControlPerVertexOutputComponents: 0,
                maxTessellationControlPerPatchOutputComponents: 0,
                maxTessellationControlTotalOutputComponents: 0,
                maxTessellationEvaluationInputComponents: 0,
                maxTessellationEvaluationOutputComponents: 0,
                maxGeometryShaderInvocations: 0,
                maxGeometryInputComponents: 0,
                maxGeometryOutputComponents: 0,
                maxGeometryOutputVertices: 0,
                maxGeometryTotalOutputComponents: 0,
                maxFragmentInputComponents: 0,
                maxFragmentOutputAttachments: 0,
                maxFragmentDualSrcAttachments: 0,
                maxFragmentCombinedOutputResources: 0,
                maxComputeSharedMemorySize: 0,
                maxComputeWorkGroupCount: [0, 0, 0],
                maxComputeWorkGroupInvocations: 0,
                maxComputeWorkGroupSize: [0, 0, 0],
                subPixelPrecisionBits: 0,
                subTexelPrecisionBits: 0,
                mipmapPrecisionBits: 0,
                maxDrawIndexedIndexValue: 0,
                maxDrawIndirectCount: 0,
                maxSamplerLodBias: 0.0,
                maxSamplerAnisotropy: 0.0,
                maxViewports: MAX_VIEWPORTS,
                maxViewportDimensions: [MAX_VIEWPORT_DIMENSIONS.0, MAX_VIEWPORT_DIMENSIONS.1],
                viewportBoundsRange: [VIEWPORT_BOUNDS_RANGE.0, VIEWPORT_BOUNDS_RANGE.0],
                viewportSubPixelBits: 0,
                minMemoryMapAlignment: 0,
                minTexelBufferOffsetAlignment: 0,
                minUniformBufferOffsetAlignment: 0,
                minStorageBufferOffsetAlignment: 0,
                minTexelOffset: 0,
                maxTexelOffset: 0,
                minTexelGatherOffset: 0,
                maxTexelGatherOffset: 0,
                minInterpolationOffset: 0.0,
                maxInterpolationOffset: 0.0,
                subPixelInterpolationOffsetBits: 0,
                maxFramebufferWidth: 0,
                maxFramebufferHeight: 0,
                maxFramebufferLayers: 0,
                framebufferColorSampleCounts: 0,
                framebufferDepthSampleCounts: 0,
                framebufferStencilSampleCounts: 0,
                framebufferNoAttachmentsSampleCounts: 0,
                maxColorAttachments: 0,
                sampledImageColorSampleCounts: 0,
                sampledImageIntegerSampleCounts: 0,
                sampledImageDepthSampleCounts: 0,
                sampledImageStencilSampleCounts: 0,
                storageImageSampleCounts: 0,
                maxSampleMaskWords: 0,
                timestampComputeAndGraphics: 0,
                timestampPeriod: 0.0,
                maxClipDistances: 0,
                maxCullDistances: 0,
                maxCombinedClipAndCullDistances: 0,
                discreteQueuePriorities: 0,
                pointSizeRange: [0.0, 0.0],
                lineWidthRange: [0.0, 0.0],
                pointSizeGranularity: 0.0,
                lineWidthGranularity: 0.0,
                strictLines: 0,
                standardSampleLocations: 0,
                optimalBufferCopyOffsetAlignment: 0,
                optimalBufferCopyRowPitchAlignment: 0,
                nonCoherentAtomSize: 1,
            },
            sparseProperties: VkPhysicalDeviceSparseProperties {
                residencyStandard2DBlockShape: 0,
                residencyStandard2DMultisampleBlockShape: 0,
                residencyStandard3DBlockShape: 0,
                residencyAlignedMipSize: 0,
                residencyNonResidentStrict: 0,
            },
        }
    }

    pub const fn tool_properties(&self) -> [VkPhysicalDeviceToolProperties; 0] {
        []
    }

    pub fn memory_properties(&self) -> VkPhysicalDeviceMemoryProperties {
        lazy_static! {
            static ref MEMORY_TYPES: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize] = {
                let mut m: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize] = [VkMemoryType {
                    propertyFlags: 0,
                    heapIndex: 0,
                };
                    VK_MAX_MEMORY_TYPES as usize];
                m[0] = VkMemoryType {
                    propertyFlags: (VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
                        | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT)
                        .into(),
                    heapIndex: 0,
                };
                m[1] = VkMemoryType {
                    propertyFlags: (VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT)
                        .into(),
                    heapIndex: 1,
                };
                m
            };
            static ref MEMORY_HEAPS: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize] = {
                let mut m: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize] =
                    [VkMemoryHeap { size: 0, flags: 0 }; VK_MAX_MEMORY_HEAPS as usize];
                m[0] = VkMemoryHeap {
                    size: gpu::Memory::memory_size_in_bytes() / 2,
                    flags: 0,
                };
                m[0] = VkMemoryHeap {
                    size: gpu::Memory::memory_size_in_bytes() / 2,
                    flags: VkMemoryHeapFlagBits::VK_MEMORY_HEAP_DEVICE_LOCAL_BIT.into(),
                };
                m
            };
        }
        VkPhysicalDeviceMemoryProperties {
            memoryTypeCount: 2,
            memoryTypes: *MEMORY_TYPES,
            memoryHeapCount: 2,
            memoryHeaps: *MEMORY_HEAPS,
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
            robustBufferAccess: VK_FALSE,
            fullDrawIndexUint32: VK_FALSE,
            imageCubeArray: VK_FALSE,
            independentBlend: VK_FALSE,
            geometryShader: VK_FALSE,
            tessellationShader: VK_FALSE,
            sampleRateShading: VK_FALSE,
            dualSrcBlend: VK_FALSE,
            logicOp: VK_FALSE,
            multiDrawIndirect: VK_FALSE,
            drawIndirectFirstInstance: VK_FALSE,
            depthClamp: VK_FALSE,
            depthBiasClamp: VK_FALSE,
            fillModeNonSolid: VK_FALSE,
            depthBounds: VK_FALSE,
            wideLines: VK_FALSE,
            largePoints: VK_FALSE,
            alphaToOne: VK_FALSE,
            multiViewport: VK_FALSE,
            samplerAnisotropy: VK_FALSE,
            textureCompressionETC2: VK_TRUE,
            textureCompressionASTC_LDR: VK_TRUE,
            textureCompressionBC: VK_TRUE,
            occlusionQueryPrecise: VK_FALSE,
            pipelineStatisticsQuery: VK_FALSE,
            vertexPipelineStoresAndAtomics: VK_FALSE,
            fragmentStoresAndAtomics: VK_FALSE,
            shaderTessellationAndGeometryPointSize: VK_FALSE,
            shaderImageGatherExtended: VK_FALSE,
            shaderStorageImageExtendedFormats: VK_FALSE,
            shaderStorageImageMultisample: VK_FALSE,
            shaderStorageImageReadWithoutFormat: VK_FALSE,
            shaderStorageImageWriteWithoutFormat: VK_FALSE,
            shaderUniformBufferArrayDynamicIndexing: VK_FALSE,
            shaderSampledImageArrayDynamicIndexing: VK_FALSE,
            shaderStorageBufferArrayDynamicIndexing: VK_FALSE,
            shaderStorageImageArrayDynamicIndexing: VK_FALSE,
            shaderClipDistance: VK_FALSE,
            shaderCullDistance: VK_FALSE,
            shaderFloat64: VK_FALSE,
            shaderInt64: VK_FALSE,
            shaderInt16: VK_FALSE,
            shaderResourceResidency: VK_FALSE,
            shaderResourceMinLod: VK_FALSE,
            sparseBinding: VK_FALSE,
            sparseResidencyBuffer: VK_FALSE,
            sparseResidencyImage2D: VK_FALSE,
            sparseResidencyImage3D: VK_FALSE,
            sparseResidency2Samples: VK_FALSE,
            sparseResidency4Samples: VK_FALSE,
            sparseResidency8Samples: VK_FALSE,
            sparseResidency16Samples: VK_FALSE,
            sparseResidencyAliased: VK_FALSE,
            variableMultisampleRate: VK_FALSE,
            inheritedQueries: VK_FALSE,
        }
    }

    pub const fn supports_features(&self, enabled_features: &VkPhysicalDeviceFeatures) -> bool {
        let supported_features = self.features();
        if enabled_features.robustBufferAccess == VK_TRUE
            && supported_features.robustBufferAccess == VK_FALSE
        {
            return false;
        }
        if enabled_features.fullDrawIndexUint32 == VK_TRUE
            && supported_features.fullDrawIndexUint32 == VK_FALSE
        {
            return false;
        }
        if enabled_features.imageCubeArray == VK_TRUE
            && supported_features.imageCubeArray == VK_FALSE
        {
            return false;
        }
        if enabled_features.independentBlend == VK_TRUE
            && supported_features.independentBlend == VK_FALSE
        {
            return false;
        }
        if enabled_features.geometryShader == VK_TRUE
            && supported_features.geometryShader == VK_FALSE
        {
            return false;
        }
        if enabled_features.tessellationShader == VK_TRUE
            && supported_features.tessellationShader == VK_FALSE
        {
            return false;
        }
        if enabled_features.sampleRateShading == VK_TRUE
            && supported_features.sampleRateShading == VK_FALSE
        {
            return false;
        }
        if enabled_features.dualSrcBlend == VK_TRUE && supported_features.dualSrcBlend == VK_FALSE {
            return false;
        }
        if enabled_features.logicOp == VK_TRUE && supported_features.logicOp == VK_FALSE {
            return false;
        }
        if enabled_features.multiDrawIndirect == VK_TRUE
            && supported_features.multiDrawIndirect == VK_FALSE
        {
            return false;
        }
        if enabled_features.drawIndirectFirstInstance == VK_TRUE
            && supported_features.drawIndirectFirstInstance == VK_FALSE
        {
            return false;
        }
        if enabled_features.depthClamp == VK_TRUE && supported_features.depthClamp == VK_FALSE {
            return false;
        }
        if enabled_features.depthBiasClamp == VK_TRUE
            && supported_features.depthBiasClamp == VK_FALSE
        {
            return false;
        }
        if enabled_features.fillModeNonSolid == VK_TRUE
            && supported_features.fillModeNonSolid == VK_FALSE
        {
            return false;
        }
        if enabled_features.depthBounds == VK_TRUE && supported_features.depthBounds == VK_FALSE {
            return false;
        }
        if enabled_features.wideLines == VK_TRUE && supported_features.wideLines == VK_FALSE {
            return false;
        }
        if enabled_features.largePoints == VK_TRUE && supported_features.largePoints == VK_FALSE {
            return false;
        }
        if enabled_features.alphaToOne == VK_TRUE && supported_features.alphaToOne == VK_FALSE {
            return false;
        }
        if enabled_features.multiViewport == VK_TRUE && supported_features.multiViewport == VK_FALSE
        {
            return false;
        }
        if enabled_features.samplerAnisotropy == VK_TRUE
            && supported_features.samplerAnisotropy == VK_FALSE
        {
            return false;
        }
        if enabled_features.textureCompressionETC2 == VK_TRUE
            && supported_features.textureCompressionETC2 == VK_FALSE
        {
            return false;
        }
        if enabled_features.textureCompressionASTC_LDR == VK_TRUE
            && supported_features.textureCompressionASTC_LDR == VK_FALSE
        {
            return false;
        }
        if enabled_features.textureCompressionBC == VK_TRUE
            && supported_features.textureCompressionBC == VK_FALSE
        {
            return false;
        }
        if enabled_features.occlusionQueryPrecise == VK_TRUE
            && supported_features.occlusionQueryPrecise == VK_FALSE
        {
            return false;
        }
        if enabled_features.pipelineStatisticsQuery == VK_TRUE
            && supported_features.pipelineStatisticsQuery == VK_FALSE
        {
            return false;
        }
        if enabled_features.vertexPipelineStoresAndAtomics == VK_TRUE
            && supported_features.vertexPipelineStoresAndAtomics == VK_FALSE
        {
            return false;
        }
        if enabled_features.fragmentStoresAndAtomics == VK_TRUE
            && supported_features.fragmentStoresAndAtomics == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderTessellationAndGeometryPointSize == VK_TRUE
            && supported_features.shaderTessellationAndGeometryPointSize == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderImageGatherExtended == VK_TRUE
            && supported_features.shaderImageGatherExtended == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderStorageImageExtendedFormats == VK_TRUE
            && supported_features.shaderStorageImageExtendedFormats == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderStorageImageMultisample == VK_TRUE
            && supported_features.shaderStorageImageMultisample == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderStorageImageReadWithoutFormat == VK_TRUE
            && supported_features.shaderStorageImageReadWithoutFormat == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderStorageImageWriteWithoutFormat == VK_TRUE
            && supported_features.shaderStorageImageWriteWithoutFormat == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderUniformBufferArrayDynamicIndexing == VK_TRUE
            && supported_features.shaderUniformBufferArrayDynamicIndexing == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderSampledImageArrayDynamicIndexing == VK_TRUE
            && supported_features.shaderSampledImageArrayDynamicIndexing == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderStorageBufferArrayDynamicIndexing == VK_TRUE
            && supported_features.shaderStorageBufferArrayDynamicIndexing == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderStorageImageArrayDynamicIndexing == VK_TRUE
            && supported_features.shaderStorageImageArrayDynamicIndexing == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderClipDistance == VK_TRUE
            && supported_features.shaderClipDistance == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderCullDistance == VK_TRUE
            && supported_features.shaderCullDistance == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderFloat64 == VK_TRUE && supported_features.shaderFloat64 == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderInt64 == VK_TRUE && supported_features.shaderInt64 == VK_FALSE {
            return false;
        }
        if enabled_features.shaderInt16 == VK_TRUE && supported_features.shaderInt16 == VK_FALSE {
            return false;
        }
        if enabled_features.shaderResourceResidency == VK_TRUE
            && supported_features.shaderResourceResidency == VK_FALSE
        {
            return false;
        }
        if enabled_features.shaderResourceMinLod == VK_TRUE
            && supported_features.shaderResourceMinLod == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseBinding == VK_TRUE && supported_features.sparseBinding == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseResidencyBuffer == VK_TRUE
            && supported_features.sparseResidencyBuffer == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseResidencyImage2D == VK_TRUE
            && supported_features.sparseResidencyImage2D == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseResidencyImage3D == VK_TRUE
            && supported_features.sparseResidencyImage3D == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseResidency2Samples == VK_TRUE
            && supported_features.sparseResidency2Samples == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseResidency4Samples == VK_TRUE
            && supported_features.sparseResidency4Samples == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseResidency8Samples == VK_TRUE
            && supported_features.sparseResidency8Samples == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseResidency16Samples == VK_TRUE
            && supported_features.sparseResidency16Samples == VK_FALSE
        {
            return false;
        }
        if enabled_features.sparseResidencyAliased == VK_TRUE
            && supported_features.sparseResidencyAliased == VK_FALSE
        {
            return false;
        }
        if enabled_features.variableMultisampleRate == VK_TRUE
            && supported_features.variableMultisampleRate == VK_FALSE
        {
            return false;
        }
        if enabled_features.inheritedQueries == VK_TRUE
            && supported_features.inheritedQueries == VK_FALSE
        {
            return false;
        }
        true
    }

    pub fn format_properties(&self, format: VkFormat) -> VkFormatProperties {
        let unsupported = VkFormatProperties {
            linearTilingFeatures: 0,
            optimalTilingFeatures: 0,
            bufferFeatures: 0,
        };
        match format {
            VkFormat::VK_FORMAT_UNDEFINED => unsupported,
            VkFormat::VK_FORMAT_R4G4_UNORM_PACK8 => unsupported,
            VkFormat::VK_FORMAT_R4G4B4A4_UNORM_PACK16 => unsupported,
            VkFormat::VK_FORMAT_B4G4R4A4_UNORM_PACK16 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_R5G6B5_UNORM_PACK16 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_B5G6R5_UNORM_PACK16 => unsupported,
            VkFormat::VK_FORMAT_R5G5B5A1_UNORM_PACK16 => unsupported,
            VkFormat::VK_FORMAT_B5G5R5A1_UNORM_PACK16 => unsupported,
            VkFormat::VK_FORMAT_A1R5G5B5_UNORM_PACK16 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_R8_UNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8_SNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8_USCALED => unsupported,
            VkFormat::VK_FORMAT_R8_SSCALED => unsupported,
            VkFormat::VK_FORMAT_R8_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8_SRGB => unsupported,
            VkFormat::VK_FORMAT_R8G8_UNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8G8_SNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8G8_USCALED => unsupported,
            VkFormat::VK_FORMAT_R8G8_SSCALED => unsupported,
            VkFormat::VK_FORMAT_R8G8_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8G8_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8G8_SRGB => unsupported,
            VkFormat::VK_FORMAT_R8G8B8_UNORM => unsupported,
            VkFormat::VK_FORMAT_R8G8B8_SNORM => unsupported,
            VkFormat::VK_FORMAT_R8G8B8_USCALED => unsupported,
            VkFormat::VK_FORMAT_R8G8B8_SSCALED => unsupported,
            VkFormat::VK_FORMAT_R8G8B8_UINT => unsupported,
            VkFormat::VK_FORMAT_R8G8B8_SINT => unsupported,
            VkFormat::VK_FORMAT_R8G8B8_SRGB => unsupported,
            VkFormat::VK_FORMAT_B8G8R8_UNORM => unsupported,
            VkFormat::VK_FORMAT_B8G8R8_SNORM => unsupported,
            VkFormat::VK_FORMAT_B8G8R8_USCALED => unsupported,
            VkFormat::VK_FORMAT_B8G8R8_SSCALED => unsupported,
            VkFormat::VK_FORMAT_B8G8R8_UINT => unsupported,
            VkFormat::VK_FORMAT_B8G8R8_SINT => unsupported,
            VkFormat::VK_FORMAT_B8G8R8_SRGB => unsupported,
            VkFormat::VK_FORMAT_R8G8B8A8_UNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8G8B8A8_SNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8G8B8A8_USCALED => unsupported,
            VkFormat::VK_FORMAT_R8G8B8A8_SSCALED => unsupported,
            VkFormat::VK_FORMAT_R8G8B8A8_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8G8B8A8_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R8G8B8A8_SRGB => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_B8G8R8A8_UNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_B8G8R8A8_SNORM => unsupported,
            VkFormat::VK_FORMAT_B8G8R8A8_USCALED => unsupported,
            VkFormat::VK_FORMAT_B8G8R8A8_SSCALED => unsupported,
            VkFormat::VK_FORMAT_B8G8R8A8_UINT => unsupported,
            VkFormat::VK_FORMAT_B8G8R8A8_SINT => unsupported,
            VkFormat::VK_FORMAT_B8G8R8A8_SRGB => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_A8B8G8R8_UNORM_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_A8B8G8R8_SNORM_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_A8B8G8R8_USCALED_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A8B8G8R8_SSCALED_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A8B8G8R8_UINT_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_A8B8G8R8_SINT_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_A8B8G8R8_SRGB_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_A2R10G10B10_UNORM_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2R10G10B10_SNORM_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2R10G10B10_USCALED_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2R10G10B10_SSCALED_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2R10G10B10_UINT_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2R10G10B10_SINT_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2B10G10R10_UNORM_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_A2B10G10R10_SNORM_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2B10G10R10_USCALED_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2B10G10R10_SSCALED_PACK32 => unsupported,
            VkFormat::VK_FORMAT_A2B10G10R10_UINT_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_A2B10G10R10_SINT_PACK32 => unsupported,
            VkFormat::VK_FORMAT_R16_UNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16_SNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16_USCALED => unsupported,
            VkFormat::VK_FORMAT_R16_SSCALED => unsupported,
            VkFormat::VK_FORMAT_R16_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16_SFLOAT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16_UNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16_SNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16_USCALED => unsupported,
            VkFormat::VK_FORMAT_R16G16_SSCALED => unsupported,
            VkFormat::VK_FORMAT_R16G16_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16_SFLOAT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16B16_UNORM => unsupported,
            VkFormat::VK_FORMAT_R16G16B16_SNORM => unsupported,
            VkFormat::VK_FORMAT_R16G16B16_USCALED => unsupported,
            VkFormat::VK_FORMAT_R16G16B16_SSCALED => unsupported,
            VkFormat::VK_FORMAT_R16G16B16_UINT => unsupported,
            VkFormat::VK_FORMAT_R16G16B16_SINT => unsupported,
            VkFormat::VK_FORMAT_R16G16B16_SFLOAT => unsupported,
            VkFormat::VK_FORMAT_R16G16B16A16_UNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16B16A16_SNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16B16A16_USCALED => unsupported,
            VkFormat::VK_FORMAT_R16G16B16A16_SSCALED => unsupported,
            VkFormat::VK_FORMAT_R16G16B16A16_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16B16A16_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R16G16B16A16_SFLOAT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32_SFLOAT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32_SFLOAT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32B32_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32B32_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32B32_SFLOAT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: 0,
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32B32A32_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32B32A32_SINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_R64_UINT => unsupported,
            VkFormat::VK_FORMAT_R64_SINT => unsupported,
            VkFormat::VK_FORMAT_R64_SFLOAT => unsupported,
            VkFormat::VK_FORMAT_R64G64_UINT => unsupported,
            VkFormat::VK_FORMAT_R64G64_SINT => unsupported,
            VkFormat::VK_FORMAT_R64G64_SFLOAT => unsupported,
            VkFormat::VK_FORMAT_R64G64B64_UINT => unsupported,
            VkFormat::VK_FORMAT_R64G64B64_SINT => unsupported,
            VkFormat::VK_FORMAT_R64G64B64_SFLOAT => unsupported,
            VkFormat::VK_FORMAT_R64G64B64A64_UINT => unsupported,
            VkFormat::VK_FORMAT_R64G64B64A64_SINT => unsupported,
            VkFormat::VK_FORMAT_R64G64B64A64_SFLOAT => unsupported,
            VkFormat::VK_FORMAT_B10G11R11_UFLOAT_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT,
                ),
            },
            VkFormat::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_D16_UNORM => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_X8_D24_UNORM_PACK32 => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_D32_SFLOAT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_S8_UINT => unsupported,
            VkFormat::VK_FORMAT_D16_UNORM_S8_UINT => unsupported,
            VkFormat::VK_FORMAT_D24_UNORM_S8_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC1_RGB_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC1_RGB_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC1_RGBA_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC1_RGBA_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC2_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC2_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC3_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC3_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC4_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC4_SNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC5_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC5_SNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC6H_UFLOAT_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC6H_SFLOAT_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC7_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_BC7_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_EAC_R11_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_EAC_R11_SNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_EAC_R11G11_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_EAC_R11G11_SNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_4x4_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_4x4_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_5x4_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_5x4_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_5x5_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_5x5_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_6x5_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_6x5_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_6x6_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_6x6_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_8x5_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_8x5_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_8x6_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_8x6_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_8x8_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_8x8_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_10x5_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_10x5_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_10x6_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_10x6_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_10x8_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_10x8_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_10x10_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_10x10_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_12x10_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_12x10_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_12x12_UNORM_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_ASTC_12x12_SRGB_BLOCK => VkFormatProperties {
                linearTilingFeatures: 0,
                optimalTilingFeatures: VkFormatFeatureFlags::from(
                    VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_SRC_BIT
                        | VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
                ),
                bufferFeatures: 0,
            },
            VkFormat::VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG => unsupported,
            VkFormat::VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG => unsupported,
            VkFormat::VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG => unsupported,
            VkFormat::VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG => unsupported,
            VkFormat::VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG => unsupported,
            VkFormat::VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG => unsupported,
            VkFormat::VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG => unsupported,
            VkFormat::VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG => unsupported,
            VkFormat::VK_FORMAT_R10X6_UNORM_PACK16 => unsupported,
            VkFormat::VK_FORMAT_R10X6G10X6_UNORM_2PACK16 => unsupported,
            VkFormat::VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16 => unsupported,
            VkFormat::VK_FORMAT_R12X4_UNORM_PACK16 => unsupported,
            VkFormat::VK_FORMAT_R12X4G12X4_UNORM_2PACK16 => unsupported,
            VkFormat::VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16 => unsupported,
            VkFormat::VK_FORMAT_G8B8G8R8_422_UNORM => unsupported,
            VkFormat::VK_FORMAT_B8G8R8G8_422_UNORM => unsupported,
            VkFormat::VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => unsupported,
            VkFormat::VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => unsupported,
            VkFormat::VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => unsupported,
            VkFormat::VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => unsupported,
            VkFormat::VK_FORMAT_G16B16G16R16_422_UNORM => unsupported,
            VkFormat::VK_FORMAT_B16G16R16G16_422_UNORM => unsupported,
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM => unsupported,
            VkFormat::VK_FORMAT_G8_B8R8_2PLANE_420_UNORM => unsupported,
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM => unsupported,
            VkFormat::VK_FORMAT_G16_B16R16_2PLANE_420_UNORM => unsupported,
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM => unsupported,
            VkFormat::VK_FORMAT_G8_B8R8_2PLANE_422_UNORM => unsupported,
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM => unsupported,
            VkFormat::VK_FORMAT_G16_B16R16_2PLANE_422_UNORM => unsupported,
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM => unsupported,
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM => unsupported,
            VkFormat::VK_FORMAT_G8_B8R8_2PLANE_444_UNORM => unsupported,
            VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => unsupported,
            VkFormat::VK_FORMAT_G16_B16R16_2PLANE_444_UNORM => unsupported,
            VkFormat::VK_FORMAT_A4R4G4B4_UNORM_PACK16 => unsupported,
            VkFormat::VK_FORMAT_A4B4G4R4_UNORM_PACK16 => unsupported,
            VkFormat::VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK => unsupported,
            VkFormat::VK_FORMAT_R16G16_S10_5_NV => unsupported,
            VkFormat(185_u32..=1000053999_u32)
            | VkFormat(1000054008_u32..=1000155999_u32)
            | VkFormat(1000156034_u32..=u32::MAX) => unreachable!(),
        }
    }

    pub fn image_format_properties(
        &self,
        format: VkFormat,
        type_: VkImageType,
        _tiling: VkImageTiling,
        usage: VkImageUsageFlags,
        _flags: VkImageCreateFlags,
    ) -> Option<VkImageFormatProperties> {
        let is_cube_compatible = (Into::<VkImageCreateFlagBits>::into(usage)
            & VkImageCreateFlagBits::VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT)
            != 0;

        let max_extent = match type_ {
            VkImageType::VK_IMAGE_TYPE_1D => VkExtent3D {
                width: 16384, // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimension1D (2).
                height: 1,
                depth: 1,
            },
            VkImageType::VK_IMAGE_TYPE_2D if is_cube_compatible => {
                VkExtent3D {
                    width: 16384,  // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimensionCube (5).
                    height: 16384, // TODO: Replace with VkPhysicalDeviceLimits::maxImageDimensionCube (6).
                    depth: 1,
                }
            }
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
            VkImageType(3_u32..=u32::MAX) => unreachable!(),
        };

        match format {
            VkFormat::VK_FORMAT_UNDEFINED => None,
            VkFormat::VK_FORMAT_R4G4_UNORM_PACK8 => None,
            VkFormat::VK_FORMAT_R4G4B4A4_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_B4G4R4A4_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_R5G6B5_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_B5G6R5_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_R5G5B5A1_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_B5G5R5A1_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_A1R5G5B5_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_R8_UNORM => None,
            VkFormat::VK_FORMAT_R8_SNORM => None,
            VkFormat::VK_FORMAT_R8_USCALED => None,
            VkFormat::VK_FORMAT_R8_SSCALED => None,
            VkFormat::VK_FORMAT_R8_UINT => None,
            VkFormat::VK_FORMAT_R8_SINT => None,
            VkFormat::VK_FORMAT_R8_SRGB => None,
            VkFormat::VK_FORMAT_R8G8_UNORM => None,
            VkFormat::VK_FORMAT_R8G8_SNORM => None,
            VkFormat::VK_FORMAT_R8G8_USCALED => None,
            VkFormat::VK_FORMAT_R8G8_SSCALED => None,
            VkFormat::VK_FORMAT_R8G8_UINT => None,
            VkFormat::VK_FORMAT_R8G8_SINT => None,
            VkFormat::VK_FORMAT_R8G8_SRGB => None,
            VkFormat::VK_FORMAT_R8G8B8_UNORM => None,
            VkFormat::VK_FORMAT_R8G8B8_SNORM => None,
            VkFormat::VK_FORMAT_R8G8B8_USCALED => None,
            VkFormat::VK_FORMAT_R8G8B8_SSCALED => None,
            VkFormat::VK_FORMAT_R8G8B8_UINT => None,
            VkFormat::VK_FORMAT_R8G8B8_SINT => None,
            VkFormat::VK_FORMAT_R8G8B8_SRGB => None,
            VkFormat::VK_FORMAT_B8G8R8_UNORM => None,
            VkFormat::VK_FORMAT_B8G8R8_SNORM => None,
            VkFormat::VK_FORMAT_B8G8R8_USCALED => None,
            VkFormat::VK_FORMAT_B8G8R8_SSCALED => None,
            VkFormat::VK_FORMAT_B8G8R8_UINT => None,
            VkFormat::VK_FORMAT_B8G8R8_SINT => None,
            VkFormat::VK_FORMAT_B8G8R8_SRGB => None,
            VkFormat::VK_FORMAT_R8G8B8A8_UNORM => Some(VkImageFormatProperties {
                maxExtent: max_extent,
                maxMipLevels: 1,
                maxArrayLayers: 1, // TODO: VkPhysicalDeviceLimits::maxImageArrayLayers
                sampleCounts: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT.into(),
                maxResourceSize: 2_u64.pow(31), // TODO: VK_ERROR_OUT_OF_DEVICE_MEMORY
            }),
            VkFormat::VK_FORMAT_R8G8B8A8_SNORM => None,
            VkFormat::VK_FORMAT_R8G8B8A8_USCALED => None,
            VkFormat::VK_FORMAT_R8G8B8A8_SSCALED => None,
            VkFormat::VK_FORMAT_R8G8B8A8_UINT => None,
            VkFormat::VK_FORMAT_R8G8B8A8_SINT => None,
            VkFormat::VK_FORMAT_R8G8B8A8_SRGB => None,
            VkFormat::VK_FORMAT_B8G8R8A8_UNORM => None,
            VkFormat::VK_FORMAT_B8G8R8A8_SNORM => None,
            VkFormat::VK_FORMAT_B8G8R8A8_USCALED => None,
            VkFormat::VK_FORMAT_B8G8R8A8_SSCALED => None,
            VkFormat::VK_FORMAT_B8G8R8A8_UINT => None,
            VkFormat::VK_FORMAT_B8G8R8A8_SINT => None,
            VkFormat::VK_FORMAT_B8G8R8A8_SRGB => None,
            VkFormat::VK_FORMAT_A8B8G8R8_UNORM_PACK32 => None,
            VkFormat::VK_FORMAT_A8B8G8R8_SNORM_PACK32 => None,
            VkFormat::VK_FORMAT_A8B8G8R8_USCALED_PACK32 => None,
            VkFormat::VK_FORMAT_A8B8G8R8_SSCALED_PACK32 => None,
            VkFormat::VK_FORMAT_A8B8G8R8_UINT_PACK32 => None,
            VkFormat::VK_FORMAT_A8B8G8R8_SINT_PACK32 => None,
            VkFormat::VK_FORMAT_A8B8G8R8_SRGB_PACK32 => None,
            VkFormat::VK_FORMAT_A2R10G10B10_UNORM_PACK32 => None,
            VkFormat::VK_FORMAT_A2R10G10B10_SNORM_PACK32 => None,
            VkFormat::VK_FORMAT_A2R10G10B10_USCALED_PACK32 => None,
            VkFormat::VK_FORMAT_A2R10G10B10_SSCALED_PACK32 => None,
            VkFormat::VK_FORMAT_A2R10G10B10_UINT_PACK32 => None,
            VkFormat::VK_FORMAT_A2R10G10B10_SINT_PACK32 => None,
            VkFormat::VK_FORMAT_A2B10G10R10_UNORM_PACK32 => None,
            VkFormat::VK_FORMAT_A2B10G10R10_SNORM_PACK32 => None,
            VkFormat::VK_FORMAT_A2B10G10R10_USCALED_PACK32 => None,
            VkFormat::VK_FORMAT_A2B10G10R10_SSCALED_PACK32 => None,
            VkFormat::VK_FORMAT_A2B10G10R10_UINT_PACK32 => None,
            VkFormat::VK_FORMAT_A2B10G10R10_SINT_PACK32 => None,
            VkFormat::VK_FORMAT_R16_UNORM => None,
            VkFormat::VK_FORMAT_R16_SNORM => None,
            VkFormat::VK_FORMAT_R16_USCALED => None,
            VkFormat::VK_FORMAT_R16_SSCALED => None,
            VkFormat::VK_FORMAT_R16_UINT => None,
            VkFormat::VK_FORMAT_R16_SINT => None,
            VkFormat::VK_FORMAT_R16_SFLOAT => None,
            VkFormat::VK_FORMAT_R16G16_UNORM => None,
            VkFormat::VK_FORMAT_R16G16_SNORM => None,
            VkFormat::VK_FORMAT_R16G16_USCALED => None,
            VkFormat::VK_FORMAT_R16G16_SSCALED => None,
            VkFormat::VK_FORMAT_R16G16_UINT => None,
            VkFormat::VK_FORMAT_R16G16_SINT => None,
            VkFormat::VK_FORMAT_R16G16_SFLOAT => None,
            VkFormat::VK_FORMAT_R16G16B16_UNORM => None,
            VkFormat::VK_FORMAT_R16G16B16_SNORM => None,
            VkFormat::VK_FORMAT_R16G16B16_USCALED => None,
            VkFormat::VK_FORMAT_R16G16B16_SSCALED => None,
            VkFormat::VK_FORMAT_R16G16B16_UINT => None,
            VkFormat::VK_FORMAT_R16G16B16_SINT => None,
            VkFormat::VK_FORMAT_R16G16B16_SFLOAT => None,
            VkFormat::VK_FORMAT_R16G16B16A16_UNORM => None,
            VkFormat::VK_FORMAT_R16G16B16A16_SNORM => None,
            VkFormat::VK_FORMAT_R16G16B16A16_USCALED => None,
            VkFormat::VK_FORMAT_R16G16B16A16_SSCALED => None,
            VkFormat::VK_FORMAT_R16G16B16A16_UINT => None,
            VkFormat::VK_FORMAT_R16G16B16A16_SINT => None,
            VkFormat::VK_FORMAT_R16G16B16A16_SFLOAT => None,
            VkFormat::VK_FORMAT_R32_UINT => None,
            VkFormat::VK_FORMAT_R32_SINT => None,
            VkFormat::VK_FORMAT_R32_SFLOAT => None,
            VkFormat::VK_FORMAT_R32G32_UINT => None,
            VkFormat::VK_FORMAT_R32G32_SINT => None,
            VkFormat::VK_FORMAT_R32G32_SFLOAT => None,
            VkFormat::VK_FORMAT_R32G32B32_UINT => None,
            VkFormat::VK_FORMAT_R32G32B32_SINT => None,
            VkFormat::VK_FORMAT_R32G32B32_SFLOAT => None,
            VkFormat::VK_FORMAT_R32G32B32A32_UINT => None,
            VkFormat::VK_FORMAT_R32G32B32A32_SINT => None,
            VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT => None,
            VkFormat::VK_FORMAT_R64_UINT => None,
            VkFormat::VK_FORMAT_R64_SINT => None,
            VkFormat::VK_FORMAT_R64_SFLOAT => None,
            VkFormat::VK_FORMAT_R64G64_UINT => None,
            VkFormat::VK_FORMAT_R64G64_SINT => None,
            VkFormat::VK_FORMAT_R64G64_SFLOAT => None,
            VkFormat::VK_FORMAT_R64G64B64_UINT => None,
            VkFormat::VK_FORMAT_R64G64B64_SINT => None,
            VkFormat::VK_FORMAT_R64G64B64_SFLOAT => None,
            VkFormat::VK_FORMAT_R64G64B64A64_UINT => None,
            VkFormat::VK_FORMAT_R64G64B64A64_SINT => None,
            VkFormat::VK_FORMAT_R64G64B64A64_SFLOAT => None,
            VkFormat::VK_FORMAT_B10G11R11_UFLOAT_PACK32 => None,
            VkFormat::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => None,
            VkFormat::VK_FORMAT_D16_UNORM => None,
            VkFormat::VK_FORMAT_X8_D24_UNORM_PACK32 => None,
            VkFormat::VK_FORMAT_D32_SFLOAT => None,
            VkFormat::VK_FORMAT_S8_UINT => None,
            VkFormat::VK_FORMAT_D16_UNORM_S8_UINT => None,
            VkFormat::VK_FORMAT_D24_UNORM_S8_UINT => None,
            VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT => None,
            VkFormat::VK_FORMAT_BC1_RGB_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC1_RGB_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_BC1_RGBA_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC1_RGBA_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_BC2_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC2_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_BC3_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC3_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_BC4_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC4_SNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC5_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC5_SNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC6H_UFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_BC6H_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_BC7_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_BC7_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_EAC_R11_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_EAC_R11_SNORM_BLOCK => None,
            VkFormat::VK_FORMAT_EAC_R11G11_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_EAC_R11G11_SNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_4x4_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_4x4_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_5x4_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_5x4_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_5x5_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_5x5_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_6x5_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_6x5_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_6x6_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_6x6_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x5_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x5_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x6_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x6_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x8_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x8_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x5_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x5_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x6_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x6_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x8_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x8_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x10_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x10_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_12x10_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_12x10_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_12x12_UNORM_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_12x12_SRGB_BLOCK => None,
            VkFormat::VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG => None,
            VkFormat::VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG => None,
            VkFormat::VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG => None,
            VkFormat::VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG => None,
            VkFormat::VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG => None,
            VkFormat::VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG => None,
            VkFormat::VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG => None,
            VkFormat::VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG => None,
            VkFormat::VK_FORMAT_R10X6_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_R10X6G10X6_UNORM_2PACK16 => None,
            VkFormat::VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16 => None,
            VkFormat::VK_FORMAT_R12X4_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_R12X4G12X4_UNORM_2PACK16 => None,
            VkFormat::VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16 => None,
            VkFormat::VK_FORMAT_G8B8G8R8_422_UNORM => None,
            VkFormat::VK_FORMAT_B8G8R8G8_422_UNORM => None,
            VkFormat::VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => None,
            VkFormat::VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => None,
            VkFormat::VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => None,
            VkFormat::VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => None,
            VkFormat::VK_FORMAT_G16B16G16R16_422_UNORM => None,
            VkFormat::VK_FORMAT_B16G16R16G16_422_UNORM => None,
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM => None,
            VkFormat::VK_FORMAT_G8_B8R8_2PLANE_420_UNORM => None,
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM => None,
            VkFormat::VK_FORMAT_G16_B16R16_2PLANE_420_UNORM => None,
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM => None,
            VkFormat::VK_FORMAT_G8_B8R8_2PLANE_422_UNORM => None,
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM => None,
            VkFormat::VK_FORMAT_G16_B16R16_2PLANE_422_UNORM => None,
            VkFormat::VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM => None,
            VkFormat::VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM => None,
            VkFormat::VK_FORMAT_G8_B8R8_2PLANE_444_UNORM => None,
            VkFormat::VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => None,
            VkFormat::VK_FORMAT_G16_B16R16_2PLANE_444_UNORM => None,
            VkFormat::VK_FORMAT_A4R4G4B4_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_A4B4G4R4_UNORM_PACK16 => None,
            VkFormat::VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK => None,
            VkFormat::VK_FORMAT_R16G16_S10_5_NV => None,
            VkFormat(185_u32..=1000053999_u32)
            | VkFormat(1000054008_u32..=1000155999_u32)
            | VkFormat(1000156034_u32..=u32::MAX) => unreachable!(),
        }
    }

    pub fn queue_family_properties(&self) -> [VkQueueFamilyProperties; 1] {
        // SPEC: If an implementation exposes any queue family that supports graphics operations,
        // at least one queue family of at least one physical device exposed by the implementation
        // must support both graphics and compute operations.
        let graphics_queue_family_properties = VkQueueFamilyProperties {
            queueFlags: (VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT
                | VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT)
                .into(),
            queueCount: 1,
            timestampValidBits: 0,
            minImageTransferGranularity: VkExtent3D {
                width: 0,
                height: 0,
                depth: 0,
            },
        };
        [graphics_queue_family_properties]
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
                width: 0xFFFFFFFF,
                height: 0xFFFFFFFF,
            },
            minImageExtent: VkExtent2D {
                width: 0,
                height: 0,
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
        }
    }
}

impl PhysicalDevice {
    pub unsafe fn parse_vertex_input_state(
        vertex_input_state: VkPipelineVertexInputStateCreateInfo,
    ) -> VertexInputState {
        let vk_attributes =
            vertex_input_state
                .pVertexAttributeDescriptions
                .map_or(&[] as &[_], |x| {
                    std::slice::from_raw_parts(
                        x.as_ptr(),
                        vertex_input_state.vertexAttributeDescriptionCount as usize,
                    )
                });
        let vk_bindings = vertex_input_state
            .pVertexBindingDescriptions
            .map_or(&[] as &[_], |x| {
                std::slice::from_raw_parts(
                    x.as_ptr(),
                    vertex_input_state.vertexBindingDescriptionCount as usize,
                )
            });
        let mut vertex_input_state = VertexInputState::default();
        for vk_attribute in vk_attributes {
            let Some(attribute) = vertex_input_state
                .attributes
                .get_mut(vk_attribute.location as usize)
            else {
                unreachable!()
            };
            *attribute = Some(VertexAttribute {
                location: vk_attribute.location,
                binding: VertexBindingNumber(vk_attribute.binding),
                format: vk_attribute.format.into(),
                offset: vk_attribute.offset,
            });
        }
        for vk_binding in vk_bindings {
            let Some(binding) = vertex_input_state
                .bindings
                .get_mut(vk_binding.binding as usize)
            else {
                unreachable!()
            };
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
            _ => unreachable!(),
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
            _ => unreachable!(),
        }
    }

    pub unsafe fn parse_viewport_state(
        viewport_state: VkPipelineViewportStateCreateInfo,
    ) -> ViewportState {
        let vk_viewports = viewport_state.pViewports.map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), viewport_state.viewportCount as usize)
        });
        let vk_scissors = viewport_state.pScissors.map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), viewport_state.scissorCount as usize)
        });

        let mut viewport_state = ViewportState::default();
        for (i, vk_viewport) in vk_viewports.iter().enumerate() {
            let Some(viewport) = viewport_state.viewports.get_mut(i) else {
                unreachable!()
            };
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
        for (i, vk_scissor) in vk_scissors.iter().enumerate() {
            let Some(scissor) = viewport_state.scissors.get_mut(i) else {
                unreachable!()
            };
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
            cull_mode: VkFlag::new(rasterization_state.cullMode).into(),
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
            assert_eq!(shader_stage.flags, 0);
            let name = shader_stage.pName.unwrap_or_else(|| unreachable!());
            let name = unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }
                .to_str()
                .unwrap_or_else(|_| unreachable!())
                .to_string();
            assert_eq!(shader_stage.pSpecializationInfo, None);
            let module =
                ShaderModule::from_handle(shader_stage.module).unwrap_or_else(|| unreachable!());
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
                _ => unimplemented!(),
            }
        }
        Ok(shader_state)
    }
}
