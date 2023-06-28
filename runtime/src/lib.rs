use headers::vk_decls::*;
use lazy_static::lazy_static;
use std::ffi::{c_char};

/* Instance */

/// Contains system-level information and functionality
#[derive(Debug)]
pub struct Instance {
    driver_name: &'static str,
    physical_device: PhysicalDevice,
}

impl Instance {
    pub const fn physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }
}

impl Instance {
    const fn new() -> Self {
        Self {
            driver_name: "vulkan_software_rasterizer",
            physical_device: PhysicalDevice::new(),
        }
    }
}

lazy_static! {
    static ref INSTANCE: Instance = Instance::new();
}

pub fn create_instance(
    create_info: &VkInstanceCreateInfo,
    p_instance: NonNull<VkInstance>,
) -> VkResult {
    let _ = create_info;
    println!("Hello from runtime::create_instance()!");
    unsafe { set_dispatchable_handle(p_instance, &*INSTANCE) };

    VkResult::VK_SUCCESS
}

/* PhysicalDevice */

/// Performs rendering operations.
#[derive(Debug)]
pub struct PhysicalDevice {
    name: &'static str,
}

impl PhysicalDevice {
    /// Instance currently supports 1 `PhysicalDevice`.
    pub const fn count(_: &Instance) -> u32 {
        1
    }

    pub fn properties(&self) -> VkPhysicalDeviceProperties {
        lazy_static! {
            static ref DEVICE_NAME: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize] = {
                let mut s: [u8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize] =
                    [0; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize];
                let device_name = "vulkan_software_rasterizer physical device";
                s[..device_name.len()].copy_from_slice(device_name.as_bytes());
                unsafe { std::mem::transmute(s) }
            };
        }

        VkPhysicalDeviceProperties {
            apiVersion: 0,
            driverVersion: 0,
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
                maxVertexInputAttributes: 0,
                maxVertexInputBindings: 0,
                maxVertexInputAttributeOffset: 0,
                maxVertexInputBindingStride: 0,
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
                maxViewports: 0,
                maxViewportDimensions: [0, 0],
                viewportBoundsRange: [0.0, 0.0],
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
                nonCoherentAtomSize: 0,
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

    pub fn memory_properties(&self) -> VkPhysicalDeviceMemoryProperties {
        lazy_static! {
            static ref MEMORY_TYPES: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize] = {
                let mut m: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize] =
                    [VkMemoryType {propertyFlags: 0, heapIndex: 0}; VK_MAX_MEMORY_TYPES as usize];
                // TODO: Fill in memory types.
                m
            };
            static ref MEMORY_HEAPS: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize] = {
                let mut m: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize] =
                    [VkMemoryHeap {size: 0, flags: 0}; VK_MAX_MEMORY_HEAPS as usize];
                // TODO: Fill in memory heaps.
                m
            };
        }
        VkPhysicalDeviceMemoryProperties {
            memoryTypeCount: 0,
            memoryTypes: *MEMORY_TYPES,
            memoryHeapCount: 0,
            memoryHeaps: *MEMORY_HEAPS,
        }
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
            textureCompressionETC2: VK_FALSE,
            textureCompressionASTC_LDR: VK_FALSE,
            textureCompressionBC: VK_FALSE,
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
}

impl PhysicalDevice {
    const fn new() -> Self {
        Self {
            name: "PhysicalDevice",
        }
    }
}

/* LogicalDevice */

/// Identifier used to associate functions with a `PhysicalDevice`.
#[derive(Debug)]
pub struct LogicalDevice {
    driver_name: &'static str,
}

impl LogicalDevice {
    const fn new() -> Self {
        Self {
            driver_name: "vulkan_software_rasterizer",
        }
    }
}
