/* Runtime crate function wrappers. */
#![allow(non_snake_case)]

use headers::vk::*;

#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
    pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceExternalImageFormatPropertiesNV (physicalDevice , format , type_ , tiling , usage , flags , externalHandleType , pExternalImageFormatProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkFreeDescriptorSets(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
) -> VkResult {
    unimplemented!(
        "vkFreeDescriptorSets (device , descriptorPool , descriptorSetCount , pDescriptorSets ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdClearDepthStencilImage(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pDepthStencil: *const VkClearDepthStencilValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
) {
    unimplemented!("vkCmdClearDepthStencilImage (commandBuffer , image , imageLayout , pDepthStencil , rangeCount , pRanges ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSparseImageFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties2,
) {
    unimplemented!("vkGetPhysicalDeviceSparseImageFormatProperties2 (physicalDevice , pFormatInfo , pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDescriptorPool(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorPool: *mut VkDescriptorPool,
) -> VkResult {
    unimplemented!("vkCreateDescriptorPool (device , pCreateInfo , pAllocator , pDescriptorPool ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdPushConstants(
    commandBuffer: VkCommandBuffer,
    layout: VkPipelineLayout,
    stageFlags: VkShaderStageFlags,
    offset: u32,
    size: u32,
    pValues: *const std::ffi::c_void,
) {
    unimplemented!(
        "vkCmdPushConstants (commandBuffer , layout , stageFlags , offset , size , pValues ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawIndirect (commandBuffer , buffer , offset , drawCount , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateRayTracingPipelinesNV(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    unimplemented!("vkCreateRayTracingPipelinesNV (device , pipelineCache , createInfoCount , pCreateInfos , pAllocator , pPipelines ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyQueryPoolResults(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) {
    unimplemented!("vkCmdCopyQueryPoolResults (commandBuffer , queryPool , firstQuery , queryCount , dstBuffer , dstOffset , stride , flags ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSurfaceSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: *mut VkBool32,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceSupportKHR (physicalDevice , queueFamilyIndex , surface , pSupported ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkImportSemaphoreZirconHandleFUCHSIA(
    device: VkDevice,
    pImportSemaphoreZirconHandleInfo: *const VkImportSemaphoreZirconHandleInfoFUCHSIA,
) -> VkResult {
    unimplemented!(
        "vkImportSemaphoreZirconHandleFUCHSIA (device , pImportSemaphoreZirconHandleInfo ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSamplerOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkSamplerCaptureDescriptorDataInfoEXT,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetSamplerOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkWaitForPresentKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    presentId: u64,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitForPresentKHR (device , swapchain , presentId , timeout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferInfo: *const VkCopyBufferInfo2,
) {
    unimplemented!("vkCmdCopyBuffer2 (commandBuffer , pCopyBufferInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPipelinePropertiesEXT(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoEXT,
    pPipelineProperties: *mut VkBaseOutStructure,
) -> VkResult {
    unimplemented!("vkGetPipelinePropertiesEXT (device , pPipelineInfo , pPipelineProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdPushDescriptorSetKHR(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
) {
    unimplemented!("vkCmdPushDescriptorSetKHR (commandBuffer , pipelineBindPoint , layout , set , descriptorWriteCount , pDescriptorWrites ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryZirconHandlePropertiesFUCHSIA(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    zirconHandle: zx_handle_t,
    pMemoryZirconHandleProperties: *mut VkMemoryZirconHandlePropertiesFUCHSIA,
) -> VkResult {
    unimplemented!("vkGetMemoryZirconHandlePropertiesFUCHSIA (device , handleType , zirconHandle , pMemoryZirconHandleProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceCapabilitiesKHR (physicalDevice , surface , pSurfaceCapabilities ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawIndexedIndirectCount(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawIndexedIndirectCount (commandBuffer , buffer , offset , countBuffer , countBufferOffset , maxDrawCount , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetLogicOpEXT(
    commandBuffer: VkCommandBuffer,
    logicOp: VkLogicOp,
) {
    unimplemented!("vkCmdSetLogicOpEXT (commandBuffer , logicOp ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindShadersEXT(
    commandBuffer: VkCommandBuffer,
    stageCount: u32,
    pStages: *const VkShaderStageFlagBits,
    pShaders: *const VkShaderEXT,
) {
    unimplemented!("vkCmdBindShadersEXT (commandBuffer , stageCount , pStages , pShaders ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyAccelerationStructureKHR(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyAccelerationStructureKHR (device , accelerationStructure , pAllocator ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthBoundsTestEnable(
    commandBuffer: VkCommandBuffer,
    depthBoundsTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthBoundsTestEnable (commandBuffer , depthBoundsTestEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSetDeviceMemoryPriorityEXT(
    device: VkDevice,
    memory: VkDeviceMemory,
    priority: f32,
) {
    unimplemented!("vkSetDeviceMemoryPriorityEXT (device , memory , priority ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkUpdateDescriptorSetWithTemplate(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: *const std::ffi::c_void,
) {
    unimplemented!("vkUpdateDescriptorSetWithTemplate (device , descriptorSet , descriptorUpdateTemplate , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueBindSparse(
    queue: VkQueue,
    bindInfoCount: u32,
    pBindInfo: *const VkBindSparseInfo,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueBindSparse (queue , bindInfoCount , pBindInfo , fence ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkImportSemaphoreWin32HandleKHR(
    device: VkDevice,
    pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
) -> VkResult {
    unimplemented!("vkImportSemaphoreWin32HandleKHR (device , pImportSemaphoreWin32HandleInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquireProfilingLockKHR(
    device: VkDevice,
    pInfo: *const VkAcquireProfilingLockInfoKHR,
) -> VkResult {
    unimplemented!("vkAcquireProfilingLockKHR (device , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkRegisterDisplayEventEXT(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayEventInfo: *const VkDisplayEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult {
    unimplemented!(
        "vkRegisterDisplayEventEXT (device , display , pDisplayEventInfo , pAllocator , pFence ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetBufferMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!("vkGetBufferMemoryRequirements2 (device , pInfo , pMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetRepresentativeFragmentTestEnableNV(
    commandBuffer: VkCommandBuffer,
    representativeFragmentTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetRepresentativeFragmentTestEnableNV (commandBuffer , representativeFragmentTestEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pEndCodingInfo: *const VkVideoEndCodingInfoKHR,
) {
    unimplemented!("vkCmdEndVideoCodingKHR (commandBuffer , pEndCodingInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkGetEventStatus (device , event ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindInvocationMaskHUAWEI(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    unimplemented!("vkCmdBindInvocationMaskHUAWEI (commandBuffer , imageView , imageLayout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateShadersEXT(
    device: VkDevice,
    createInfoCount: u32,
    pCreateInfos: *const VkShaderCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pShaders: *mut VkShaderEXT,
) -> VkResult {
    unimplemented!(
        "vkCreateShadersEXT (device , createInfoCount , pCreateInfos , pAllocator , pShaders ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSubpassShadingHUAWEI(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdSubpassShadingHUAWEI (commandBuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyDescriptorSetLayout(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDescriptorSetLayout (device , descriptorSetLayout , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawIndexedIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDrawIndexedIndirect (commandBuffer , buffer , offset , drawCount , stride ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDescriptorUpdateTemplate(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult {
    unimplemented!("vkCreateDescriptorUpdateTemplate (device , pCreateInfo , pAllocator , pDescriptorUpdateTemplate ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyImageToBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2,
) {
    unimplemented!("vkCmdCopyImageToBuffer2 (commandBuffer , pCopyImageToBufferInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!("vkGetImageMemoryRequirements2 (device , pInfo , pMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceGroupSurfacePresentModesKHR(
    device: VkDevice,
    surface: VkSurfaceKHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupSurfacePresentModesKHR (device , surface , pModes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginConditionalRenderingEXT(
    commandBuffer: VkCommandBuffer,
    pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT,
) {
    unimplemented!(
        "vkCmdBeginConditionalRenderingEXT (commandBuffer , pConditionalRenderingBegin ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCopyMicromapToMemoryEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
) -> VkResult {
    unimplemented!("vkCopyMicromapToMemoryEXT (device , deferredOperation , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateMacOSSurfaceMVK(
    instance: VkInstance,
    pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateMacOSSurfaceMVK (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetViewportWScalingEnableNV(
    commandBuffer: VkCommandBuffer,
    viewportWScalingEnable: VkBool32,
) {
    unimplemented!("vkCmdSetViewportWScalingEnableNV (commandBuffer , viewportWScalingEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkFreeMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkFreeMemory (device , memory , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDescriptorPool (device , descriptorPool , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBlitImage2(
    commandBuffer: VkCommandBuffer,
    pBlitImageInfo: *const VkBlitImageInfo2,
) {
    unimplemented!("vkCmdBlitImage2 (commandBuffer , pBlitImageInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkFlushMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult {
    unimplemented!("vkFlushMappedMemoryRanges (device , memoryRangeCount , pMemoryRanges ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkResetFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
) -> VkResult {
    unimplemented!("vkResetFences (device , fenceCount , pFences ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMicromapBuildSizesEXT(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkMicromapBuildInfoEXT,
    pSizeInfo: *mut VkMicromapBuildSizesInfoEXT,
) {
    unimplemented!("vkGetMicromapBuildSizesEXT (device , buildType , pBuildInfo , pSizeInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdDebugMarkerEndEXT (commandBuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDisplayPlaneSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!(
        "vkCreateDisplayPlaneSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginRenderPass(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    contents: VkSubpassContents,
) {
    unimplemented!("vkCmdBeginRenderPass (commandBuffer , pRenderPassBegin , contents ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquireXlibDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireXlibDisplayEXT (physicalDevice , dpy , display ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetPerformanceMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceMarkerInfoINTEL,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceMarkerINTEL (commandBuffer , pMarkerInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetPerformanceOverrideINTEL(
    commandBuffer: VkCommandBuffer,
    pOverrideInfo: *const VkPerformanceOverrideInfoINTEL,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceOverrideINTEL (commandBuffer , pOverrideInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroySemaphore(
    device: VkDevice,
    semaphore: VkSemaphore,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySemaphore (device , semaphore , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginTransformFeedbackEXT(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
) {
    unimplemented!("vkCmdBeginTransformFeedbackEXT (commandBuffer , firstCounterBuffer , counterBufferCount , pCounterBuffers , pCounterBufferOffsets ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdNextSubpass(
    commandBuffer: VkCommandBuffer,
    contents: VkSubpassContents,
) {
    unimplemented!("vkCmdNextSubpass (commandBuffer , contents ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndConditionalRenderingEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndConditionalRenderingEXT (commandBuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkImportSemaphoreFdKHR(
    device: VkDevice,
    pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR,
) -> VkResult {
    unimplemented!("vkImportSemaphoreFdKHR (device , pImportSemaphoreFdInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateAccelerationStructureKHR(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureKHR,
) -> VkResult {
    unimplemented!("vkCreateAccelerationStructureKHR (device , pCreateInfo , pAllocator , pAccelerationStructure ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetStencilReference(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    reference: u32,
) {
    unimplemented!("vkCmdSetStencilReference (commandBuffer , faceMask , reference ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetShaderModuleCreateInfoIdentifierEXT(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
) {
    unimplemented!(
        "vkGetShaderModuleCreateInfoIdentifierEXT (device , pCreateInfo , pIdentifier ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) {
    unimplemented!("vkUnmapMemory (device , memory ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: *mut VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkGetDrmDisplayEXT (physicalDevice , drmFd , connectorId , display ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetAccelerationStructureDeviceAddressKHR(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR,
) -> VkDeviceAddress {
    unimplemented!("vkGetAccelerationStructureDeviceAddressKHR (device , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetQueueCheckpointDataNV(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointDataNV,
) {
    unimplemented!("vkGetQueueCheckpointDataNV (queue , pCheckpointDataCount , pCheckpointData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryZirconHandleFUCHSIA(
    device: VkDevice,
    pGetZirconHandleInfo: *const VkMemoryGetZirconHandleInfoFUCHSIA,
    pZirconHandle: *mut zx_handle_t,
) -> VkResult {
    unimplemented!(
        "vkGetMemoryZirconHandleFUCHSIA (device , pGetZirconHandleInfo , pZirconHandle ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceGroupPresentCapabilitiesKHR(
    device: VkDevice,
    pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR,
) -> VkResult {
    unimplemented!(
        "vkGetDeviceGroupPresentCapabilitiesKHR (device , pDeviceGroupPresentCapabilities ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCheckpointNV(
    commandBuffer: VkCommandBuffer,
    pCheckpointMarker: *const std::ffi::c_void,
) {
    unimplemented!("vkCmdSetCheckpointNV (commandBuffer , pCheckpointMarker ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEncodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pEncodeInfo: *const VkVideoEncodeInfoKHR,
) {
    unimplemented!("vkCmdEncodeVideoKHR (commandBuffer , pEncodeInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyImageView(
    device: VkDevice,
    imageView: VkImageView,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyImageView (device , imageView , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBuildMicromapsEXT(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
) {
    unimplemented!("vkCmdBuildMicromapsEXT (commandBuffer , infoCount , pInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMicromapInfoEXT,
) {
    unimplemented!("vkCmdCopyMicromapEXT (commandBuffer , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthClipNegativeOneToOneEXT(
    commandBuffer: VkCommandBuffer,
    negativeOneToOne: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClipNegativeOneToOneEXT (commandBuffer , negativeOneToOne ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetStencilWriteMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    writeMask: u32,
) {
    unimplemented!("vkCmdSetStencilWriteMask (commandBuffer , faceMask , writeMask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawMeshTasksIndirectCountNV(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksIndirectCountNV (commandBuffer , buffer , offset , countBuffer , countBufferOffset , maxDrawCount , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceProperties(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties,
) {
    unimplemented!("vkGetPhysicalDeviceProperties (physicalDevice , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdExecuteCommands(
    commandBuffer: VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
) {
    unimplemented!("vkCmdExecuteCommands (commandBuffer , commandBufferCount , pCommandBuffers ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateStreamDescriptorSurfaceGGP(
    instance: VkInstance,
    pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!(
        "vkCreateStreamDescriptorSurfaceGGP (instance , pCreateInfo , pAllocator , pSurface ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetVideoSessionMemoryRequirementsKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pMemoryRequirementsCount: *mut u32,
    pMemoryRequirements: *mut VkVideoSessionMemoryRequirementsKHR,
) -> VkResult {
    unimplemented!("vkGetVideoSessionMemoryRequirementsKHR (device , videoSession , pMemoryRequirementsCount , pMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateFence(
    device: VkDevice,
    pCreateInfo: *const VkFenceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult {
    unimplemented!("vkCreateFence (device , pCreateInfo , pAllocator , pFence ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkImportFenceWin32HandleKHR(
    device: VkDevice,
    pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
) -> VkResult {
    unimplemented!("vkImportFenceWin32HandleKHR (device , pImportFenceWin32HandleInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindDescriptorBufferEmbeddedSamplersEXT(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
) {
    unimplemented!("vkCmdBindDescriptorBufferEmbeddedSamplersEXT (commandBuffer , pipelineBindPoint , layout , set ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlanePropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceDisplayPlanePropertiesKHR (physicalDevice , pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemorySciBufNV(
    device: VkDevice,
    pGetSciBufInfo: *const VkMemoryGetSciBufInfoNV,
    pHandle: *mut NvSciBufObj,
) -> VkResult {
    unimplemented!("vkGetMemorySciBufNV (device , pGetSciBufInfo , pHandle ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkEnumeratePhysicalDeviceGroups(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: *mut u32,
    pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult {
    unimplemented!("vkEnumeratePhysicalDeviceGroups (instance , pPhysicalDeviceGroupCount , pPhysicalDeviceGroupProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDescriptorEXT(
    device: VkDevice,
    pDescriptorInfo: *const VkDescriptorGetInfoEXT,
    dataSize: isize,
    pDescriptor: *mut std::ffi::c_void,
) {
    unimplemented!("vkGetDescriptorEXT (device , pDescriptorInfo , dataSize , pDescriptor ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSwapchainGrallocUsage2ANDROID(
    device: VkDevice,
    format: VkFormat,
    imageUsage: VkImageUsageFlags,
    swapchainImageUsage: VkSwapchainImageUsageFlagsANDROID,
    grallocConsumerUsage: *mut u64,
    grallocProducerUsage: *mut u64,
) -> VkResult {
    unimplemented!("vkGetSwapchainGrallocUsage2ANDROID (device , format , imageUsage , swapchainImageUsage , grallocConsumerUsage , grallocProducerUsage ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetColorBlendEnableEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEnables: *const VkBool32,
) {
    unimplemented!("vkCmdSetColorBlendEnableEXT (commandBuffer , firstAttachment , attachmentCount , pColorBlendEnables ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyQueryPool (device , queryPool , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCopyAccelerationStructureToMemoryKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) -> VkResult {
    unimplemented!("vkCopyAccelerationStructureToMemoryKHR (device , deferredOperation , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateSampler(
    device: VkDevice,
    pCreateInfo: *const VkSamplerCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSampler: *mut VkSampler,
) -> VkResult {
    unimplemented!("vkCreateSampler (device , pCreateInfo , pAllocator , pSampler ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthTestEnable(
    commandBuffer: VkCommandBuffer,
    depthTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthTestEnable (commandBuffer , depthTestEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyMemoryToMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
) {
    unimplemented!("vkCmdCopyMemoryToMicromapEXT (commandBuffer , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyImage2(
    commandBuffer: VkCommandBuffer,
    pCopyImageInfo: *const VkCopyImageInfo2,
) {
    unimplemented!("vkCmdCopyImage2 (commandBuffer , pCopyImageInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetShaderBinaryDataEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pDataSize: *mut isize,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetShaderBinaryDataEXT (device , shader , pDataSize , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquireNextImage2KHR(
    device: VkDevice,
    pAcquireInfo: *const VkAcquireNextImageInfoKHR,
    pImageIndex: *mut u32,
) -> VkResult {
    unimplemented!("vkAcquireNextImage2KHR (device , pAcquireInfo , pImageIndex ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDiscardRectangleEnableEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDiscardRectangleEnableEXT (commandBuffer , discardRectangleEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceScreenPresentationSupportQNX(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    window: *mut _screen_window,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceScreenPresentationSupportQNX (physicalDevice , queueFamilyIndex , window ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkVideoBeginCodingInfoKHR,
) {
    unimplemented!("vkCmdBeginVideoCodingKHR (commandBuffer , pBeginInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetFragmentShadingRateKHR(
    commandBuffer: VkCommandBuffer,
    pFragmentSize: *const VkExtent2D,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2 as usize],
) {
    unimplemented!("vkCmdSetFragmentShadingRateKHR (commandBuffer , pFragmentSize , combinerOps ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWriteBufferMarkerAMD(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
) {
    unimplemented!("vkCmdWriteBufferMarkerAMD (commandBuffer , pipelineStage , dstBuffer , dstOffset , marker ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkUpdateVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR,
) -> VkResult {
    unimplemented!(
        "vkUpdateVideoSessionParametersKHR (device , videoSessionParameters , pUpdateInfo ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyCuModuleNVX(
    device: VkDevice,
    module: VkCuModuleNVX,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyCuModuleNVX (device , module , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageSparseMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
) {
    unimplemented!("vkGetImageSparseMemoryRequirements (device , image , pSparseMemoryRequirementCount , pSparseMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetFenceWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult {
    unimplemented!("vkGetFenceWin32HandleKHR (device , pGetWin32HandleInfo , pHandle ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdNextSubpass2(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
    pSubpassEndInfo: *const VkSubpassEndInfo,
) {
    unimplemented!("vkCmdNextSubpass2 (commandBuffer , pSubpassBeginInfo , pSubpassEndInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetAccelerationStructureMemoryRequirementsNV(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2KHR,
) {
    unimplemented!(
        "vkGetAccelerationStructureMemoryRequirementsNV (device , pInfo , pMemoryRequirements ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetLogicOpEnableEXT(
    commandBuffer: VkCommandBuffer,
    logicOpEnable: VkBool32,
) {
    unimplemented!("vkCmdSetLogicOpEnableEXT (commandBuffer , logicOpEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
    device: VkDevice,
    renderpass: VkRenderPass,
    pMaxWorkgroupSize: *mut VkExtent2D,
) -> VkResult {
    unimplemented!("vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI (device , renderpass , pMaxWorkgroupSize ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
) {
    unimplemented!("vkCmdBeginQuery (commandBuffer , queryPool , query , flags ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateAndroidSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateAndroidSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdPipelineBarrier(
    commandBuffer: VkCommandBuffer,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    dependencyFlags: VkDependencyFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const VkMemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const VkImageMemoryBarrier,
) {
    unimplemented!("vkCmdPipelineBarrier (commandBuffer , srcStageMask , dstStageMask , dependencyFlags , memoryBarrierCount , pMemoryBarriers , bufferMemoryBarrierCount , pBufferMemoryBarriers , imageMemoryBarrierCount , pImageMemoryBarriers ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSurfacePresentModes2EXT(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfacePresentModes2EXT (physicalDevice , pSurfaceInfo , pPresentModeCount , pPresentModes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthCompareOp(
    commandBuffer: VkCommandBuffer,
    depthCompareOp: VkCompareOp,
) {
    unimplemented!("vkCmdSetDepthCompareOp (commandBuffer , depthCompareOp ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDecompressMemoryIndirectCountNV(
    commandBuffer: VkCommandBuffer,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    stride: u32,
) {
    unimplemented!("vkCmdDecompressMemoryIndirectCountNV (commandBuffer , indirectCommandsAddress , indirectCommandsCountAddress , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkImportFenceFdKHR(
    device: VkDevice,
    pImportFenceFdInfo: *const VkImportFenceFdInfoKHR,
) -> VkResult {
    unimplemented!("vkImportFenceFdKHR (device , pImportFenceFdInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetAlphaToCoverageEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToCoverageEnable: VkBool32,
) {
    unimplemented!("vkCmdSetAlphaToCoverageEnableEXT (commandBuffer , alphaToCoverageEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWriteTimestamp2(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdWriteTimestamp2 (commandBuffer , stage , queryPool , query ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdTraceRaysKHR(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
) {
    unimplemented!("vkCmdTraceRaysKHR (commandBuffer , pRaygenShaderBindingTable , pMissShaderBindingTable , pHitShaderBindingTable , pCallableShaderBindingTable , width , height , depth ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateQueryPool(
    device: VkDevice,
    pCreateInfo: *const VkQueryPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pQueryPool: *mut VkQueryPool,
) -> VkResult {
    unimplemented!("vkCreateQueryPool (device , pCreateInfo , pAllocator , pQueryPool ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateSharedSwapchainsKHR(
    device: VkDevice,
    swapchainCount: u32,
    pCreateInfos: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchains: *mut VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkCreateSharedSwapchainsKHR (device , swapchainCount , pCreateInfos , pAllocator , pSwapchains ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetCalibratedTimestampsEXT(
    device: VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const VkCalibratedTimestampInfoEXT,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> VkResult {
    unimplemented!("vkGetCalibratedTimestampsEXT (device , timestampCount , pTimestampInfos , pTimestamps , pMaxDeviation ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueBeginDebugUtilsLabelEXT(
    queue: VkQueue,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    unimplemented!("vkQueueBeginDebugUtilsLabelEXT (queue , pLabelInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkUnmapMemory2KHR(
    device: VkDevice,
    pMemoryUnmapInfo: *const VkMemoryUnmapInfoKHR,
) -> VkResult {
    unimplemented!("vkUnmapMemory2KHR (device , pMemoryUnmapInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDiscardRectangleModeEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleMode: VkDiscardRectangleModeEXT,
) {
    unimplemented!("vkCmdSetDiscardRectangleModeEXT (commandBuffer , discardRectangleMode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyIndirectCommandsLayoutNV(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyIndirectCommandsLayoutNV (device , indirectCommandsLayout , pAllocator ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkInitializePerformanceApiINTEL(
    device: VkDevice,
    pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL,
) -> VkResult {
    unimplemented!("vkInitializePerformanceApiINTEL (device , pInitializeInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateRenderPass(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult {
    unimplemented!("vkCreateRenderPass (device , pCreateInfo , pAllocator , pRenderPass ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdRefreshObjectsKHR(
    commandBuffer: VkCommandBuffer,
    pRefreshObjects: *const VkRefreshObjectListKHR,
) {
    unimplemented!("vkCmdRefreshObjectsKHR (commandBuffer , pRefreshObjects ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDisplayModeKHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pCreateInfo: *const VkDisplayModeCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pMode: *mut VkDisplayModeKHR,
) -> VkResult {
    unimplemented!(
        "vkCreateDisplayModeKHR (physicalDevice , display , pCreateInfo , pAllocator , pMode ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDecodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pDecodeInfo: *const VkVideoDecodeInfoKHR,
) {
    unimplemented!("vkCmdDecodeVideoKHR (commandBuffer , pDecodeInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetConservativeRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
) {
    unimplemented!("vkCmdSetConservativeRasterizationModeEXT (commandBuffer , conservativeRasterizationMode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSwapchainCounterEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    counter: VkSurfaceCounterFlagBitsEXT,
    pCounterValue: *mut u64,
) -> VkResult {
    unimplemented!("vkGetSwapchainCounterEXT (device , swapchain , counter , pCounterValue ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkBuildAccelerationStructuresKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const VkAccelerationStructureBuildRangeInfoKHR,
) -> VkResult {
    unimplemented!("vkBuildAccelerationStructuresKHR (device , deferredOperation , infoCount , pInfos , ppBuildRangeInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyDescriptorUpdateTemplate(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyDescriptorUpdateTemplate (device , descriptorUpdateTemplate , pAllocator ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBuildAccelerationStructuresKHR(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const VkAccelerationStructureBuildRangeInfoKHR,
) {
    unimplemented!("vkCmdBuildAccelerationStructuresKHR (commandBuffer , infoCount , pInfos , ppBuildRangeInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdUpdateBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    dataSize: VkDeviceSize,
    pData: *const std::ffi::c_void,
) {
    unimplemented!("vkCmdUpdateBuffer (commandBuffer , dstBuffer , dstOffset , dataSize , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindDescriptorSets(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
    dynamicOffsetCount: u32,
    pDynamicOffsets: *const u32,
) {
    unimplemented!("vkCmdBindDescriptorSets (commandBuffer , pipelineBindPoint , layout , firstSet , descriptorSetCount , pDescriptorSets , dynamicOffsetCount , pDynamicOffsets ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryAndroidHardwareBufferANDROID(
    device: VkDevice,
    pInfo: *const VkMemoryGetAndroidHardwareBufferInfoANDROID,
    pBuffer: *mut AHardwareBuffer,
) -> VkResult {
    unimplemented!("vkGetMemoryAndroidHardwareBufferANDROID (device , pInfo , pBuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateIndirectCommandsLayoutNV(
    device: VkDevice,
    pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV,
) -> VkResult {
    unimplemented!("vkCreateIndirectCommandsLayoutNV (device , pCreateInfo , pAllocator , pIndirectCommandsLayout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateImagePipeSurfaceFUCHSIA(
    instance: VkInstance,
    pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!(
        "vkCreateImagePipeSurfaceFUCHSIA (instance , pCreateInfo , pAllocator , pSurface ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCopyAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
) -> VkResult {
    unimplemented!("vkCopyAccelerationStructureKHR (device , deferredOperation , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pQualityLevelInfo: *const VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR,
    pQualityLevelProperties: *mut VkVideoEncodeQualityLevelPropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR (physicalDevice , pQualityLevelInfo , pQualityLevelProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueEndDebugUtilsLabelEXT(queue: VkQueue) {
    unimplemented!("vkQueueEndDebugUtilsLabelEXT (queue ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDispatch(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDispatch (commandBuffer , groupCountX , groupCountY , groupCountZ ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetLineRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    lineRasterizationMode: VkLineRasterizationModeEXT,
) {
    unimplemented!("vkCmdSetLineRasterizationModeEXT (commandBuffer , lineRasterizationMode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkEnumerateDeviceExtensionProperties(
    physicalDevice: VkPhysicalDevice,
    pLayerName: *const std::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult {
    unimplemented!("vkEnumerateDeviceExtensionProperties (physicalDevice , pLayerName , pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateRenderPass2(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult {
    unimplemented!("vkCreateRenderPass2 (device , pCreateInfo , pAllocator , pRenderPass ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateOpticalFlowSessionNV(
    device: VkDevice,
    pCreateInfo: *const VkOpticalFlowSessionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pSession: *mut VkOpticalFlowSessionNV,
) -> VkResult {
    unimplemented!("vkCreateOpticalFlowSessionNV (device , pCreateInfo , pAllocator , pSession ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueSetPerformanceConfigurationINTEL(
    queue: VkQueue,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!("vkQueueSetPerformanceConfigurationINTEL (queue , configuration ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDebugMarkerSetObjectTagEXT(
    device: VkDevice,
    pTagInfo: *const VkDebugMarkerObjectTagInfoEXT,
) -> VkResult {
    unimplemented!("vkDebugMarkerSetObjectTagEXT (device , pTagInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDeferredOperationJoinKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkDeferredOperationJoinKHR (device , operation ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdEndQuery (commandBuffer , queryPool , query ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryHostPointerPropertiesEXT(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHostPointer: *const std::ffi::c_void,
    pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult {
    unimplemented!("vkGetMemoryHostPointerPropertiesEXT (device , handleType , pHostPointer , pMemoryHostPointerProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceDisplayProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayProperties2KHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceDisplayProperties2KHR (physicalDevice , pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdPipelineBarrier2(
    commandBuffer: VkCommandBuffer,
    pDependencyInfo: *const VkDependencyInfo,
) {
    unimplemented!("vkCmdPipelineBarrier2 (commandBuffer , pDependencyInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceCapabilities2EXT (physicalDevice , surface , pSurfaceCapabilities ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSurfaceFormats2KHR(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormat2KHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceFormats2KHR (physicalDevice , pSurfaceInfo , pSurfaceFormatCount , pSurfaceFormats ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDeferredOperationKHR(
    device: VkDevice,
    pAllocator: *const VkAllocationCallbacks,
    pDeferredOperation: *mut VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkCreateDeferredOperationKHR (device , pAllocator , pDeferredOperation ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDescriptorBufferOffsetsEXT(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    setCount: u32,
    pBufferIndices: *const u32,
    pOffsets: *const VkDeviceSize,
) {
    unimplemented!("vkCmdSetDescriptorBufferOffsetsEXT (commandBuffer , pipelineBindPoint , layout , firstSet , setCount , pBufferIndices , pOffsets ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSurfaceFormatsKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormatKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceFormatsKHR (physicalDevice , surface , pSurfaceFormatCount , pSurfaceFormats ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeferredOperationMaxConcurrencyKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> u32 {
    unimplemented!("vkGetDeferredOperationMaxConcurrencyKHR (device , operation ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetTessellationDomainOriginEXT(
    commandBuffer: VkCommandBuffer,
    domainOrigin: VkTessellationDomainOrigin,
) {
    unimplemented!("vkCmdSetTessellationDomainOriginEXT (commandBuffer , domainOrigin ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateSwapchainKHR(
    device: VkDevice,
    pCreateInfo: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchain: *mut VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkCreateSwapchainKHR (device , pCreateInfo , pAllocator , pSwapchain ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceWin32PresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceWin32PresentationSupportKHR (physicalDevice , queueFamilyIndex ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueInsertDebugUtilsLabelEXT(
    queue: VkQueue,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    unimplemented!("vkQueueInsertDebugUtilsLabelEXT (queue , pLabelInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSetHdrMetadataEXT(
    device: VkDevice,
    swapchainCount: u32,
    pSwapchains: *const VkSwapchainKHR,
    pMetadata: *const VkHdrMetadataEXT,
) {
    unimplemented!("vkSetHdrMetadataEXT (device , swapchainCount , pSwapchains , pMetadata ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCompileDeferredNV(
    device: VkDevice,
    pipeline: VkPipeline,
    shader: u32,
) -> VkResult {
    unimplemented!("vkCompileDeferredNV (device , pipeline , shader ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindVertexBuffers(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
) {
    unimplemented!("vkCmdBindVertexBuffers (commandBuffer , firstBinding , bindingCount , pBuffers , pOffsets ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateVideoSessionKHR(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSession: *mut VkVideoSessionKHR,
) -> VkResult {
    unimplemented!("vkCreateVideoSessionKHR (device , pCreateInfo , pAllocator , pVideoSession ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCopyMemoryToMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
) -> VkResult {
    unimplemented!("vkCopyMemoryToMicromapEXT (device , deferredOperation , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceQueue(
    device: VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: *mut VkQueue,
) {
    unimplemented!("vkGetDeviceQueue (device , queueFamilyIndex , queueIndex , pQueue ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCoverageToColorLocationNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorLocation: u32,
) {
    unimplemented!("vkCmdSetCoverageToColorLocationNV (commandBuffer , coverageToColorLocation ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawMeshTasksIndirectEXT(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDrawMeshTasksIndirectEXT (commandBuffer , buffer , offset , drawCount , stride ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageViewHandleNVX(
    device: VkDevice,
    pInfo: *const VkImageViewHandleInfoNVX,
) -> u32 {
    unimplemented!("vkGetImageViewHandleNVX (device , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyPrivateDataSlot(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyPrivateDataSlot (device , privateDataSlot , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDescriptorSetLayoutSupport(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pSupport: *mut VkDescriptorSetLayoutSupport,
) {
    unimplemented!("vkGetDescriptorSetLayoutSupport (device , pCreateInfo , pSupport ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    unimplemented!("vkCmdBeginDebugUtilsLabelEXT (commandBuffer , pLabelInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkWriteAccelerationStructuresPropertiesKHR(
    device: VkDevice,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    dataSize: isize,
    pData: *mut std::ffi::c_void,
    stride: isize,
) -> VkResult {
    unimplemented!("vkWriteAccelerationStructuresPropertiesKHR (device , accelerationStructureCount , pAccelerationStructures , queryType , dataSize , pData , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceCooperativeMatrixPropertiesNV (physicalDevice , pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkTrimCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
) {
    unimplemented!("vkTrimCommandPool (device , commandPool , flags ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkEnumerateInstanceLayerProperties(
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult {
    unimplemented!("vkEnumerateInstanceLayerProperties (pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetSampleMaskEXT(
    commandBuffer: VkCommandBuffer,
    samples: VkSampleCountFlagBits,
    pSampleMask: *const VkSampleMask,
) {
    unimplemented!("vkCmdSetSampleMaskEXT (commandBuffer , samples , pSampleMask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyEvent(
    device: VkDevice,
    event: VkEvent,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyEvent (device , event , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetValidationCacheDataEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pDataSize: *mut isize,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetValidationCacheDataEXT (device , validationCache , pDataSize , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateImageView(
    device: VkDevice,
    pCreateInfo: *const VkImageViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkImageView,
) -> VkResult {
    unimplemented!("vkCreateImageView (device , pCreateInfo , pAllocator , pView ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetViewportWithCount(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const VkViewport,
) {
    unimplemented!("vkCmdSetViewportWithCount (commandBuffer , viewportCount , pViewports ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetExclusiveScissorNV(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissors: *const VkRect2D,
) {
    unimplemented!("vkCmdSetExclusiveScissorNV (commandBuffer , firstExclusiveScissor , exclusiveScissorCount , pExclusiveScissors ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyRenderPass(
    device: VkDevice,
    renderPass: VkRenderPass,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyRenderPass (device , renderPass , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlaneProperties2KHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceDisplayPlaneProperties2KHR (physicalDevice , pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateInstance(
    pCreateInfo: *const VkInstanceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pInstance: *mut VkInstance,
) -> VkResult {
    // TODO: Dereference unsafe pointers before sending them to runtime?
    runtime::create_instance(pCreateInfo, pAllocator, pInstance)
}

#[no_mangle]
pub(crate) extern "C" fn vkDestroyInstance(
    instance: VkInstance,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyInstance (instance , pAllocator ,)")
}

#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceToolProperties(
    physicalDevice: VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolProperties,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceToolProperties (physicalDevice , pToolCount , pToolProperties ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindPipeline(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
) {
    unimplemented!("vkCmdBindPipeline (commandBuffer , pipelineBindPoint , pipeline ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindTransformFeedbackBuffersEXT(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
) {
    unimplemented!("vkCmdBindTransformFeedbackBuffersEXT (commandBuffer , firstBinding , bindingCount , pBuffers , pOffsets , pSizes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetBufferDeviceAddress(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress {
    unimplemented!("vkGetBufferDeviceAddress (device , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyDeferredOperationKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDeferredOperationKHR (device , operation , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDescriptorSetLayoutSizeEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    pLayoutSizeInBytes: *mut VkDeviceSize,
) {
    unimplemented!("vkGetDescriptorSetLayoutSizeEXT (device , layout , pLayoutSizeInBytes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> VkResult {
    unimplemented!("vkEnumerateInstanceVersion (pApiVersion ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetPerformanceStreamMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceStreamMarkerINTEL (commandBuffer , pMarkerInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroySwapchainKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySwapchainKHR (device , swapchain , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCopyMemoryToAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) -> VkResult {
    unimplemented!("vkCopyMemoryToAccelerationStructureKHR (device , deferredOperation , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetQueryPoolResults(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dataSize: isize,
    pData: *mut std::ffi::c_void,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) -> VkResult {
    unimplemented!("vkGetQueryPoolResults (device , queryPool , firstQuery , queryCount , dataSize , pData , stride , flags ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkMergeValidationCachesEXT(
    device: VkDevice,
    dstCache: VkValidationCacheEXT,
    srcCacheCount: u32,
    pSrcCaches: *const VkValidationCacheEXT,
) -> VkResult {
    unimplemented!("vkMergeValidationCachesEXT (device , dstCache , srcCacheCount , pSrcCaches ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDisplayPlaneCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    mode: VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayPlaneCapabilitiesKHR (physicalDevice , mode , planeIndex , pCapabilities ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawMeshTasksNV(
    commandBuffer: VkCommandBuffer,
    taskCount: u32,
    firstTask: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksNV (commandBuffer , taskCount , firstTask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPerformanceParameterINTEL(
    device: VkDevice,
    parameter: VkPerformanceParameterTypeINTEL,
    pValue: *mut VkPerformanceValueINTEL,
) -> VkResult {
    unimplemented!("vkGetPerformanceParameterINTEL (device , parameter , pValue ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDecompressMemoryNV(
    commandBuffer: VkCommandBuffer,
    decompressRegionCount: u32,
    pDecompressMemoryRegions: *const VkDecompressMemoryRegionNV,
) {
    unimplemented!("vkCmdDecompressMemoryNV (commandBuffer , decompressRegionCount , pDecompressMemoryRegions ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    dfb: *mut IDirectFB,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceDirectFBPresentationSupportEXT (physicalDevice , queueFamilyIndex , dfb ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetFenceSciSyncFenceNV(
    device: VkDevice,
    pGetSciSyncHandleInfo: *const VkFenceGetSciSyncInfoNV,
    pHandle: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetFenceSciSyncFenceNV (device , pGetSciSyncHandleInfo , pHandle ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut VkPerformanceCounterKHR,
    pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR,
) -> VkResult {
    unimplemented!("vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR (physicalDevice , queueFamilyIndex , pCounterCount , pCounters , pCounterDescriptions ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateBuffer(
    device: VkDevice,
    pCreateInfo: *const VkBufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pBuffer: *mut VkBuffer,
) -> VkResult {
    unimplemented!("vkCreateBuffer (device , pCreateInfo , pAllocator , pBuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetQueueCheckpointData2NV(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointData2NV,
) {
    unimplemented!("vkGetQueueCheckpointData2NV (queue , pCheckpointDataCount , pCheckpointData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSwapchainImagesKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut VkImage,
) -> VkResult {
    unimplemented!(
        "vkGetSwapchainImagesKHR (device , swapchain , pSwapchainImageCount , pSwapchainImages ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
    physicalDevice: VkPhysicalDevice,
    pCombinationCount: *mut u32,
    pCombinations: *mut VkFramebufferMixedSamplesCombinationNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV (physicalDevice , pCombinationCount , pCombinations ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateShaderModule(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pShaderModule: *mut VkShaderModule,
) -> VkResult {
    unimplemented!("vkCreateShaderModule (device , pCreateInfo , pAllocator , pShaderModule ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandleKHR (device , pGetWin32HandleInfo , pHandle ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceMicromapCompatibilityEXT(
    device: VkDevice,
    pVersionInfo: *const VkMicromapVersionInfoEXT,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
) {
    unimplemented!("vkGetDeviceMicromapCompatibilityEXT (device , pVersionInfo , pCompatibility ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateImage(
    device: VkDevice,
    pCreateInfo: *const VkImageCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pImage: *mut VkImage,
) -> VkResult {
    unimplemented!("vkCreateImage (device , pCreateInfo , pAllocator , pImage ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkReleaseDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkReleaseDisplayEXT (physicalDevice , display ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawMeshTasksEXT(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!(
        "vkCmdDrawMeshTasksEXT (commandBuffer , groupCountX , groupCountY , groupCountZ ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkImportFenceSciSyncFenceNV(
    device: VkDevice,
    pImportFenceSciSyncInfo: *const VkImportFenceSciSyncInfoNV,
) -> VkResult {
    unimplemented!("vkImportFenceSciSyncFenceNV (device , pImportFenceSciSyncInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetBufferOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> u64 {
    unimplemented!("vkGetBufferOpaqueCaptureAddress (device , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetStencilCompareMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    compareMask: u32,
) {
    unimplemented!("vkCmdSetStencilCompareMask (commandBuffer , faceMask , compareMask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceGroupPeerMemoryFeatures(
    device: VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags,
) {
    unimplemented!("vkGetDeviceGroupPeerMemoryFeatures (device , heapIndex , localDeviceIndex , remoteDeviceIndex , pPeerMemoryFeatures ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdResolveImage2(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: *const VkResolveImageInfo2,
) {
    unimplemented!("vkCmdResolveImage2 (commandBuffer , pResolveImageInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceOpticalFlowImageFormatsNV(
    physicalDevice: VkPhysicalDevice,
    pOpticalFlowImageFormatInfo: *const VkOpticalFlowImageFormatInfoNV,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut VkOpticalFlowImageFormatPropertiesNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceOpticalFlowImageFormatsNV (physicalDevice , pOpticalFlowImageFormatInfo , pFormatCount , pImageFormatProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDisplayPowerControlEXT(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayPowerInfo: *const VkDisplayPowerInfoEXT,
) -> VkResult {
    unimplemented!("vkDisplayPowerControlEXT (device , display , pDisplayPowerInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCoverageModulationTableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableCount: u32,
    pCoverageModulationTable: *const f32,
) {
    unimplemented!("vkCmdSetCoverageModulationTableNV (commandBuffer , coverageModulationTableCount , pCoverageModulationTable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetPatchControlPointsEXT(
    commandBuffer: VkCommandBuffer,
    patchControlPoints: u32,
) {
    unimplemented!("vkCmdSetPatchControlPointsEXT (commandBuffer , patchControlPoints ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyVideoSessionKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyVideoSessionKHR (device , videoSession , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyMicromapEXT(
    device: VkDevice,
    micromap: VkMicromapEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyMicromapEXT (device , micromap , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueWaitIdle(queue: VkQueue) -> VkResult {
    unimplemented!("vkQueueWaitIdle (queue ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdTraceRaysNV(
    commandBuffer: VkCommandBuffer,
    raygenShaderBindingTableBuffer: VkBuffer,
    raygenShaderBindingOffset: VkDeviceSize,
    missShaderBindingTableBuffer: VkBuffer,
    missShaderBindingOffset: VkDeviceSize,
    missShaderBindingStride: VkDeviceSize,
    hitShaderBindingTableBuffer: VkBuffer,
    hitShaderBindingOffset: VkDeviceSize,
    hitShaderBindingStride: VkDeviceSize,
    callableShaderBindingTableBuffer: VkBuffer,
    callableShaderBindingOffset: VkDeviceSize,
    callableShaderBindingStride: VkDeviceSize,
    width: u32,
    height: u32,
    depth: u32,
) {
    unimplemented!("vkCmdTraceRaysNV (commandBuffer , raygenShaderBindingTableBuffer , raygenShaderBindingOffset , missShaderBindingTableBuffer , missShaderBindingOffset , missShaderBindingStride , hitShaderBindingTableBuffer , hitShaderBindingOffset , hitShaderBindingStride , callableShaderBindingTableBuffer , callableShaderBindingOffset , callableShaderBindingStride , width , height , depth ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetFragmentShadingRateEnumNV(
    commandBuffer: VkCommandBuffer,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2 as usize],
) {
    unimplemented!(
        "vkCmdSetFragmentShadingRateEnumNV (commandBuffer , shadingRate , combinerOps ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetViewport(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: *const VkViewport,
) {
    unimplemented!(
        "vkCmdSetViewport (commandBuffer , firstViewport , viewportCount , pViewports ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkBindBufferMemory(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    unimplemented!("vkBindBufferMemory (device , buffer , memory , memoryOffset ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkRegisterDeviceEventEXT(
    device: VkDevice,
    pDeviceEventInfo: *const VkDeviceEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult {
    unimplemented!("vkRegisterDeviceEventEXT (device , pDeviceEventInfo , pAllocator , pFence ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: *mut VkMemoryRequirements,
) {
    unimplemented!("vkGetImageMemoryRequirements (device , image , pMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyValidationCacheEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyValidationCacheEXT (device , validationCache , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawIndirectCount(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawIndirectCount (commandBuffer , buffer , offset , countBuffer , countBufferOffset , maxDrawCount , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyShaderEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyShaderEXT (device , shader , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawMultiIndexedEXT(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pIndexInfo: *const VkMultiDrawIndexedInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
    pVertexOffset: *const i32,
) {
    unimplemented!("vkCmdDrawMultiIndexedEXT (commandBuffer , drawCount , pIndexInfo , instanceCount , firstInstance , stride , pVertexOffset ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkWaitForFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
    waitAll: VkBool32,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitForFences (device , fenceCount , pFences , waitAll , timeout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeferredOperationResultKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkGetDeferredOperationResultKHR (device , operation ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthClipEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthClipEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClipEnableEXT (commandBuffer , depthClipEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateSamplerYcbcrConversion(
    device: VkDevice,
    pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult {
    unimplemented!(
        "vkCreateSamplerYcbcrConversion (device , pCreateInfo , pAllocator , pYcbcrConversion ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetPrimitiveTopology(
    commandBuffer: VkCommandBuffer,
    primitiveTopology: VkPrimitiveTopology,
) {
    unimplemented!("vkCmdSetPrimitiveTopology (commandBuffer , primitiveTopology ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetFenceFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkFenceGetFdInfoKHR,
    pFd: *mut int,
) -> VkResult {
    unimplemented!("vkGetFenceFdKHR (device , pGetFdInfo , pFd ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndRenderPass (commandBuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdExecuteGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
) {
    unimplemented!("vkCmdExecuteGeneratedCommandsNV (commandBuffer , isPreprocessed , pGeneratedCommandsInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryWin32HandlePropertiesKHR(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandlePropertiesKHR (device , handleType , handle , pMemoryWin32HandleProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetBufferMemoryRequirements(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: *mut VkMemoryRequirements,
) {
    unimplemented!("vkGetBufferMemoryRequirements (device , buffer , pMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawMeshTasksIndirectNV(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDrawMeshTasksIndirectNV (commandBuffer , buffer , offset , drawCount , stride ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageViewOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkImageViewCaptureDescriptorDataInfoEXT,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetImageViewOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdResetQueryPool(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    unimplemented!("vkCmdResetQueryPool (commandBuffer , queryPool , firstQuery , queryCount ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceProperties2(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties2,
) {
    unimplemented!("vkGetPhysicalDeviceProperties2 (physicalDevice , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginRenderPass2(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
) {
    unimplemented!("vkCmdBeginRenderPass2 (commandBuffer , pRenderPassBegin , pSubpassBeginInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetSampleLocationsEnableEXT(
    commandBuffer: VkCommandBuffer,
    sampleLocationsEnable: VkBool32,
) {
    unimplemented!("vkCmdSetSampleLocationsEnableEXT (commandBuffer , sampleLocationsEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPrivateData(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: *mut u64,
) {
    unimplemented!(
        "vkGetPrivateData (device , objectType , objectHandle , privateDataSlot , pData ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateCuModuleNVX(
    device: VkDevice,
    pCreateInfo: *const VkCuModuleCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pModule: *mut VkCuModuleNVX,
) -> VkResult {
    unimplemented!("vkCreateCuModuleNVX (device , pCreateInfo , pAllocator , pModule ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetBufferOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkBufferCaptureDescriptorDataInfoEXT,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetBufferOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceFaultInfoEXT(
    device: VkDevice,
    pFaultCounts: *mut VkDeviceFaultCountsEXT,
    pFaultInfo: *mut VkDeviceFaultInfoEXT,
) -> VkResult {
    unimplemented!("vkGetDeviceFaultInfoEXT (device , pFaultCounts , pFaultInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetSampleLocationsEXT(
    commandBuffer: VkCommandBuffer,
    pSampleLocationsInfo: *const VkSampleLocationsInfoEXT,
) {
    unimplemented!("vkCmdSetSampleLocationsEXT (commandBuffer , pSampleLocationsInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWriteTimestamp(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdWriteTimestamp (commandBuffer , pipelineStage , queryPool , query ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryRemoteAddressNV(
    device: VkDevice,
    pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV,
    pAddress: *mut VkRemoteAddressNV,
) -> VkResult {
    unimplemented!("vkGetMemoryRemoteAddressNV (device , pMemoryGetRemoteAddressInfo , pAddress ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceQueue2(
    device: VkDevice,
    pQueueInfo: *const VkDeviceQueueInfo2,
    pQueue: *mut VkQueue,
) {
    unimplemented!("vkGetDeviceQueue2 (device , pQueueInfo , pQueue ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetStencilOp(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
) {
    unimplemented!("vkCmdSetStencilOp (commandBuffer , faceMask , failOp , passOp , depthFailOp , compareOp ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCullMode(
    commandBuffer: VkCommandBuffer,
    cullMode: VkCullModeFlags,
) {
    unimplemented!("vkCmdSetCullMode (commandBuffer , cullMode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthBias(
    commandBuffer: VkCommandBuffer,
    depthBiasConstantFactor: f32,
    depthBiasClamp: f32,
    depthBiasSlopeFactor: f32,
) {
    unimplemented!("vkCmdSetDepthBias (commandBuffer , depthBiasConstantFactor , depthBiasClamp , depthBiasSlopeFactor ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkFreeCommandBuffers(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
) {
    unimplemented!(
        "vkFreeCommandBuffers (device , commandPool , commandBufferCount , pCommandBuffers ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetBufferCollectionPropertiesFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pProperties: *mut VkBufferCollectionPropertiesFUCHSIA,
) -> VkResult {
    unimplemented!("vkGetBufferCollectionPropertiesFUCHSIA (device , collection , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetStencilTestEnable(
    commandBuffer: VkCommandBuffer,
    stencilTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetStencilTestEnable (commandBuffer , stencilTestEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDispatchIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    unimplemented!("vkCmdDispatchIndirect (commandBuffer , buffer , offset ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    physicalDevice: VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut VkTimeDomainEXT,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceCalibrateableTimeDomainsEXT (physicalDevice , pTimeDomainCount , pTimeDomains ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindPipelineShaderGroupNV(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    groupIndex: u32,
) {
    unimplemented!("vkCmdBindPipelineShaderGroupNV (commandBuffer , pipelineBindPoint , pipeline , groupIndex ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetViewportSwizzleNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportSwizzles: *const VkViewportSwizzleNV,
) {
    unimplemented!("vkCmdSetViewportSwizzleNV (commandBuffer , firstViewport , viewportCount , pViewportSwizzles ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndRenderPass2(
    commandBuffer: VkCommandBuffer,
    pSubpassEndInfo: *const VkSubpassEndInfo,
) {
    unimplemented!("vkCmdEndRenderPass2 (commandBuffer , pSubpassEndInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroySamplerYcbcrConversion(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySamplerYcbcrConversion (device , ycbcrConversion , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetLineStippleEXT(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    unimplemented!(
        "vkCmdSetLineStippleEXT (commandBuffer , lineStippleFactor , lineStipplePattern ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDisplayModePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModePropertiesKHR,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayModePropertiesKHR (physicalDevice , display , pPropertyCount , pProperties ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateMetalSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: *const VkMetalSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateMetalSurfaceEXT (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceBufferMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!("vkGetDeviceBufferMemoryRequirements (device , pInfo , pMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateWin32SurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateWin32SurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSemaphoreWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult {
    unimplemented!("vkGetSemaphoreWin32HandleKHR (device , pGetWin32HandleInfo , pHandle ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSciSyncAttributesNV(
    physicalDevice: VkPhysicalDevice,
    pSciSyncAttributesInfo: *const VkSciSyncAttributesInfoNV,
    pAttributes: NvSciSyncAttrList,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSciSyncAttributesNV (physicalDevice , pSciSyncAttributesInfo , pAttributes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageSubresourceLayout(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource,
    pLayout: *mut VkSubresourceLayout,
) {
    unimplemented!("vkGetImageSubresourceLayout (device , image , pSubresource , pLayout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindDescriptorBuffersEXT(
    commandBuffer: VkCommandBuffer,
    bufferCount: u32,
    pBindingInfos: *const VkDescriptorBufferBindingInfoEXT,
) {
    unimplemented!("vkCmdBindDescriptorBuffersEXT (commandBuffer , bufferCount , pBindingInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetFenceSciSyncObjNV(
    device: VkDevice,
    pGetSciSyncHandleInfo: *const VkFenceGetSciSyncInfoNV,
    pHandle: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetFenceSciSyncObjNV (device , pGetSciSyncHandleInfo , pHandle ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageSparseMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
) {
    unimplemented!("vkGetImageSparseMemoryRequirements2 (device , pInfo , pSparseMemoryRequirementCount , pSparseMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindShadingRateImageNV(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    unimplemented!("vkCmdBindShadingRateImageNV (commandBuffer , imageView , imageLayout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
    physicalDevice: VkPhysicalDevice,
    pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR,
    pNumPasses: *mut u32,
) {
    unimplemented!("vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR (physicalDevice , pPerformanceQueryCreateInfo , pNumPasses ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyImage(
    device: VkDevice,
    image: VkImage,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyImage (device , image , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkResetCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolResetFlags,
) -> VkResult {
    unimplemented!("vkResetCommandPool (device , commandPool , flags ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSwapchainStatusKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkGetSwapchainStatusKHR (device , swapchain ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdResetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
) {
    unimplemented!("vkCmdResetEvent2 (commandBuffer , event , stageMask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyDebugReportCallbackEXT(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDebugReportCallbackEXT (instance , callback , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties,
) {
    unimplemented!("vkGetPhysicalDeviceQueueFamilyProperties (physicalDevice , pQueueFamilyPropertyCount , pQueueFamilyProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyBuffer(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferCopy,
) {
    unimplemented!(
        "vkCmdCopyBuffer (commandBuffer , srcBuffer , dstBuffer , regionCount , pRegions ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetColorBlendAdvancedEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendAdvanced: *const VkColorBlendAdvancedEXT,
) {
    unimplemented!("vkCmdSetColorBlendAdvancedEXT (commandBuffer , firstAttachment , attachmentCount , pColorBlendAdvanced ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyOpticalFlowSessionNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyOpticalFlowSessionNV (device , session , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateComputePipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkComputePipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    unimplemented!("vkCreateComputePipelines (device , pipelineCache , createInfoCount , pCreateInfos , pAllocator , pPipelines ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateGraphicsPipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkGraphicsPipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    unimplemented!("vkCreateGraphicsPipelines (device , pipelineCache , createInfoCount , pCreateInfos , pAllocator , pPipelines ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWaitEvents(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const VkMemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const VkImageMemoryBarrier,
) {
    unimplemented!("vkCmdWaitEvents (commandBuffer , eventCount , pEvents , srcStageMask , dstStageMask , memoryBarrierCount , pMemoryBarriers , bufferMemoryBarrierCount , pBufferMemoryBarriers , imageMemoryBarrierCount , pImageMemoryBarriers ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceExternalSemaphoreProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
    pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
) {
    unimplemented!("vkGetPhysicalDeviceExternalSemaphoreProperties (physicalDevice , pExternalSemaphoreInfo , pExternalSemaphoreProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkImportFenceSciSyncObjNV(
    device: VkDevice,
    pImportFenceSciSyncInfo: *const VkImportFenceSciSyncInfoNV,
) -> VkResult {
    unimplemented!("vkImportFenceSciSyncObjNV (device , pImportFenceSciSyncInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetRefreshCycleDurationGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE,
) -> VkResult {
    unimplemented!(
        "vkGetRefreshCycleDurationGOOGLE (device , swapchain , pDisplayTimingProperties ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetFrontFace(
    commandBuffer: VkCommandBuffer,
    frontFace: VkFrontFace,
) {
    unimplemented!("vkCmdSetFrontFace (commandBuffer , frontFace ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkBuildMicromapsEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
) -> VkResult {
    unimplemented!("vkBuildMicromapsEXT (device , deferredOperation , infoCount , pInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDescriptorSetLayoutBindingOffsetEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    binding: u32,
    pOffset: *mut VkDeviceSize,
) {
    unimplemented!(
        "vkGetDescriptorSetLayoutBindingOffsetEXT (device , layout , binding , pOffset ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetFramebufferTilePropertiesQCOM(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pPropertiesCount: *mut u32,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult {
    unimplemented!("vkGetFramebufferTilePropertiesQCOM (device , framebuffer , pPropertiesCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSparseImageFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    samples: VkSampleCountFlagBits,
    usage: VkImageUsageFlags,
    tiling: VkImageTiling,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties,
) {
    unimplemented!("vkGetPhysicalDeviceSparseImageFormatProperties (physicalDevice , format , type_ , samples , usage , tiling , pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyMemoryIndirectNV(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdCopyMemoryIndirectNV (commandBuffer , copyBufferAddress , copyCount , stride ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkEnumerateInstanceExtensionProperties(
    pLayerName: *const std::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult {
    println!("Hello from vkEnumerateInstanceExtensionProperties()!");
    assert_eq!(pLayerName, std::ptr::null());
    unsafe {
        println!("*pPropertyCount = {}", *pPropertyCount);
        println!("*pProperties = {:?}", pProperties);
        if pProperties == std::ptr::null_mut() {
            *pPropertyCount = 0;
        }
    }
    VkResult::VK_SUCCESS
}
#[no_mangle]
pub(crate) extern "C" fn vkSubmitDebugUtilsMessageEXT(
    instance: VkInstance,
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
) {
    unimplemented!("vkSubmitDebugUtilsMessageEXT (instance , messageSeverity , messageTypes , pCallbackData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateVideoSessionParametersKHR(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionParametersCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSessionParameters: *mut VkVideoSessionParametersKHR,
) -> VkResult {
    unimplemented!("vkCreateVideoSessionParametersKHR (device , pCreateInfo , pAllocator , pVideoSessionParameters ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceFragmentShadingRatesKHR(
    physicalDevice: VkPhysicalDevice,
    pFragmentShadingRateCount: *mut u32,
    pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceFragmentShadingRatesKHR (physicalDevice , pFragmentShadingRateCount , pFragmentShadingRates ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetGeneratedCommandsMemoryRequirementsNV(
    device: VkDevice,
    pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!(
        "vkGetGeneratedCommandsMemoryRequirementsNV (device , pInfo , pMemoryRequirements ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCoverageToColorEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorEnable: VkBool32,
) {
    unimplemented!("vkCmdSetCoverageToColorEnableNV (commandBuffer , coverageToColorEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkQueuePresentKHR(
    queue: VkQueue,
    pPresentInfo: *const VkPresentInfoKHR,
) -> VkResult {
    unimplemented!("vkQueuePresentKHR (queue , pPresentInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquireDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireDrmDisplayEXT (physicalDevice , drmFd , display ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreatePipelineLayout(
    device: VkDevice,
    pCreateInfo: *const VkPipelineLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineLayout: *mut VkPipelineLayout,
) -> VkResult {
    unimplemented!("vkCreatePipelineLayout (device , pCreateInfo , pAllocator , pPipelineLayout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceProcAddr(
    device: VkDevice,
    pName: *const std::ffi::c_char,
) -> PFN_vkVoidFunction {
    unimplemented!("vkGetDeviceProcAddr (device , pName ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateXlibSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateXlibSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDebugMarkerSetObjectNameEXT(
    device: VkDevice,
    pNameInfo: *const VkDebugMarkerObjectNameInfoEXT,
) -> VkResult {
    unimplemented!("vkDebugMarkerSetObjectNameEXT (device , pNameInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSetDebugUtilsObjectNameEXT(
    device: VkDevice,
    pNameInfo: *const VkDebugUtilsObjectNameInfoEXT,
) -> VkResult {
    unimplemented!("vkSetDebugUtilsObjectNameEXT (device , pNameInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceMemoryCommitment(
    device: VkDevice,
    memory: VkDeviceMemory,
    pCommittedMemoryInBytes: *mut VkDeviceSize,
) {
    unimplemented!("vkGetDeviceMemoryCommitment (device , memory , pCommittedMemoryInBytes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    index: u32,
) {
    unimplemented!("vkCmdEndQueryIndexedEXT (commandBuffer , queryPool , query , index ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: *const VkDependencyInfo,
) {
    unimplemented!("vkCmdSetEvent2 (commandBuffer , event , pDependencyInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkResetEvent (device , event ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyAccelerationStructureToMemoryKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) {
    unimplemented!("vkCmdCopyAccelerationStructureToMemoryKHR (commandBuffer , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: NvSciBufObj,
    pMemorySciBufProperties: *mut VkMemorySciBufPropertiesNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV (physicalDevice , handleType , handle , pMemorySciBufProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceRefreshableObjectTypesKHR(
    physicalDevice: VkPhysicalDevice,
    pRefreshableObjectTypeCount: *mut u32,
    pRefreshableObjectTypes: *mut VkObjectType,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceRefreshableObjectTypesKHR (physicalDevice , pRefreshableObjectTypeCount , pRefreshableObjectTypes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateCuFunctionNVX(
    device: VkDevice,
    pCreateInfo: *const VkCuFunctionCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pFunction: *mut VkCuFunctionNVX,
) -> VkResult {
    unimplemented!("vkCreateCuFunctionNVX (device , pCreateInfo , pAllocator , pFunction ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetWinrtDisplayNV(
    physicalDevice: VkPhysicalDevice,
    deviceRelativeId: u32,
    pDisplay: *mut VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkGetWinrtDisplayNV (physicalDevice , deviceRelativeId , pDisplay ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetShadingRateImageEnableNV(
    commandBuffer: VkCommandBuffer,
    shadingRateImageEnable: VkBool32,
) {
    unimplemented!("vkCmdSetShadingRateImageEnableNV (commandBuffer , shadingRateImageEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyBuffer(
    device: VkDevice,
    buffer: VkBuffer,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyBuffer (device , buffer , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceMemoryProperties2(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
) {
    unimplemented!("vkGetPhysicalDeviceMemoryProperties2 (physicalDevice , pMemoryProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetExclusiveScissorEnableNV(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissorEnables: *const VkBool32,
) {
    unimplemented!("vkCmdSetExclusiveScissorEnableNV (commandBuffer , firstExclusiveScissor , exclusiveScissorCount , pExclusiveScissorEnables ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkImportSemaphoreSciSyncObjNV(
    device: VkDevice,
    pImportSemaphoreSciSyncInfo: *const VkImportSemaphoreSciSyncInfoNV,
) -> VkResult {
    unimplemented!("vkImportSemaphoreSciSyncObjNV (device , pImportSemaphoreSciSyncInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroySurfaceKHR(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySurfaceKHR (instance , surface , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDirectFBSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateDirectFBSurfaceEXT (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkBindImageMemory(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    unimplemented!("vkBindImageMemory (device , image , memory , memoryOffset ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdControlVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pCodingControlInfo: *const VkVideoCodingControlInfoKHR,
) {
    unimplemented!("vkCmdControlVideoCodingKHR (commandBuffer , pCodingControlInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginRendering(
    commandBuffer: VkCommandBuffer,
    pRenderingInfo: *const VkRenderingInfo,
) {
    unimplemented!("vkCmdBeginRendering (commandBuffer , pRenderingInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties2,
) {
    unimplemented!(
        "vkGetPhysicalDeviceFormatProperties2 (physicalDevice , format , pFormatProperties ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdInsertDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    unimplemented!("vkCmdInsertDebugUtilsLabelEXT (commandBuffer , pLabelInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndTransformFeedbackEXT(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
) {
    unimplemented!("vkCmdEndTransformFeedbackEXT (commandBuffer , firstCounterBuffer , counterBufferCount , pCounterBuffers , pCounterBufferOffsets ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdPreprocessGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
) {
    unimplemented!("vkCmdPreprocessGeneratedCommandsNV (commandBuffer , pGeneratedCommandsInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthBiasEnable(
    commandBuffer: VkCommandBuffer,
    depthBiasEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthBiasEnable (commandBuffer , depthBiasEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueSubmit(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueSubmit (queue , submitCount , pSubmits , fence ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdResolveImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageResolve,
) {
    unimplemented!("vkCmdResolveImage (commandBuffer , srcImage , srcImageLayout , dstImage , dstImageLayout , regionCount , pRegions ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: isize,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetRayTracingCaptureReplayShaderGroupHandlesKHR (device , pipeline , firstGroup , groupCount , dataSize , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkBindOpticalFlowSessionImageNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    bindingPoint: VkOpticalFlowSessionBindingPointNV,
    view: VkImageView,
    layout: VkImageLayout,
) -> VkResult {
    unimplemented!(
        "vkBindOpticalFlowSessionImageNV (device , session , bindingPoint , view , layout ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquireWinrtDisplayNV(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireWinrtDisplayNV (physicalDevice , display ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceGroupSurfacePresentModes2EXT(
    device: VkDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupSurfacePresentModes2EXT (device , pSurfaceInfo , pModes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDevicePresentRectanglesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pRectCount: *mut u32,
    pRects: *mut VkRect2D,
) -> VkResult {
    unimplemented!("vkGetPhysicalDevicePresentRectanglesKHR (physicalDevice , surface , pRectCount , pRects ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateHeadlessSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateHeadlessSurfaceEXT (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDisplayModeProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModeProperties2KHR,
) -> VkResult {
    unimplemented!("vkGetDisplayModeProperties2KHR (physicalDevice , display , pPropertyCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceImageFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
    pImageFormatProperties: *mut VkImageFormatProperties2,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceImageFormatProperties2 (physicalDevice , pImageFormatInfo , pImageFormatProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCuLaunchKernelNVX(
    commandBuffer: VkCommandBuffer,
    pLaunchInfo: *const VkCuLaunchInfoNVX,
) {
    unimplemented!("vkCmdCuLaunchKernelNVX (commandBuffer , pLaunchInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties2(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2,
) {
    unimplemented!("vkGetPhysicalDeviceQueueFamilyProperties2 (physicalDevice , pQueueFamilyPropertyCount , pQueueFamilyProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindVertexBuffers2(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
    pStrides: *const VkDeviceSize,
) {
    unimplemented!("vkCmdBindVertexBuffers2 (commandBuffer , firstBinding , bindingCount , pBuffers , pOffsets , pSizes , pStrides ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkBindBufferMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindBufferMemoryInfo,
) -> VkResult {
    unimplemented!("vkBindBufferMemory2 (device , bindInfoCount , pBindInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateIOSSurfaceMVK(
    instance: VkInstance,
    pCreateInfo: *const VkIOSSurfaceCreateInfoMVK,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateIOSSurfaceMVK (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyPipeline(
    device: VkDevice,
    pipeline: VkPipeline,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyPipeline (device , pipeline , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetExtraPrimitiveOverestimationSizeEXT(
    commandBuffer: VkCommandBuffer,
    extraPrimitiveOverestimationSize: f32,
) {
    unimplemented!("vkCmdSetExtraPrimitiveOverestimationSizeEXT (commandBuffer , extraPrimitiveOverestimationSize ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
) {
    unimplemented!("vkCmdCopyAccelerationStructureKHR (commandBuffer , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageViewAddressNVX(
    device: VkDevice,
    imageView: VkImageView,
    pProperties: *mut VkImageViewAddressPropertiesNVX,
) -> VkResult {
    unimplemented!("vkGetImageViewAddressNVX (device , imageView , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyCuFunctionNVX(
    device: VkDevice,
    function: VkCuFunctionNVX,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyCuFunctionNVX (device , function , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdClearAttachments(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pAttachments: *const VkClearAttachment,
    rectCount: u32,
    pRects: *const VkClearRect,
) {
    unimplemented!("vkCmdClearAttachments (commandBuffer , attachmentCount , pAttachments , rectCount , pRects ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkInvalidateMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult {
    unimplemented!("vkInvalidateMappedMemoryRanges (device , memoryRangeCount , pMemoryRanges ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkSetEvent (device , event ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthBounds(
    commandBuffer: VkCommandBuffer,
    minDepthBounds: f32,
    maxDepthBounds: f32,
) {
    unimplemented!("vkCmdSetDepthBounds (commandBuffer , minDepthBounds , maxDepthBounds ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyBufferView(
    device: VkDevice,
    bufferView: VkBufferView,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyBufferView (device , bufferView , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdResetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    unimplemented!("vkCmdResetEvent (commandBuffer , event , stageMask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryWin32HandleNV(
    device: VkDevice,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    pHandle: *mut HANDLE,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandleNV (device , memory , handleType , pHandle ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSetBufferCollectionImageConstraintsFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pImageConstraintsInfo: *const VkImageConstraintsInfoFUCHSIA,
) -> VkResult {
    unimplemented!("vkSetBufferCollectionImageConstraintsFUCHSIA (device , collection , pImageConstraintsInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateAccelerationStructureNV(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureNV,
) -> VkResult {
    unimplemented!("vkCreateAccelerationStructureNV (device , pCreateInfo , pAllocator , pAccelerationStructure ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageCopy,
) {
    unimplemented!("vkCmdCopyImage (commandBuffer , srcImage , srcImageLayout , dstImage , dstImageLayout , regionCount , pRegions ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCoverageModulationModeNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationMode: VkCoverageModulationModeNV,
) {
    unimplemented!("vkCmdSetCoverageModulationModeNV (commandBuffer , coverageModulationMode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceExternalFenceProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
    pExternalFenceProperties: *mut VkExternalFenceProperties,
) {
    unimplemented!("vkGetPhysicalDeviceExternalFenceProperties (physicalDevice , pExternalFenceInfo , pExternalFenceProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAllocateCommandBuffers(
    device: VkDevice,
    pAllocateInfo: *const VkCommandBufferAllocateInfo,
    pCommandBuffers: *mut VkCommandBuffer,
) -> VkResult {
    unimplemented!("vkAllocateCommandBuffers (device , pAllocateInfo , pCommandBuffers ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetShaderInfoAMD(
    device: VkDevice,
    pipeline: VkPipeline,
    shaderStage: VkShaderStageFlagBits,
    infoType: VkShaderInfoTypeAMD,
    pInfoSize: *mut isize,
    pInfo: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!(
        "vkGetShaderInfoAMD (device , pipeline , shaderStage , infoType , pInfoSize , pInfo ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkBindAccelerationStructureMemoryNV(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV,
) -> VkResult {
    unimplemented!("vkBindAccelerationStructureMemoryNV (device , bindInfoCount , pBindInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyDebugUtilsMessengerEXT(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDebugUtilsMessengerEXT (instance , messenger , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceMemoryOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
    unimplemented!("vkGetDeviceMemoryOpaqueCaptureAddress (device , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetInstanceProcAddr(
    instance: VkInstance,
    pName: *const std::ffi::c_char,
) -> PFN_vkVoidFunction {
    unimplemented!("vkGetInstanceProcAddr (instance , pName ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkUninitializePerformanceApiINTEL(device: VkDevice) {
    unimplemented!("vkUninitializePerformanceApiINTEL (device ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateSemaphoreSciSyncPoolNV(
    device: VkDevice,
    pCreateInfo: *const VkSemaphoreSciSyncPoolCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pSemaphorePool: *mut VkSemaphoreSciSyncPoolNV,
) -> VkResult {
    unimplemented!(
        "vkCreateSemaphoreSciSyncPoolNV (device , pCreateInfo , pAllocator , pSemaphorePool ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyCommandPool (device , commandPool , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdTraceRaysIndirectKHR(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    indirectDeviceAddress: VkDeviceAddress,
) {
    unimplemented!("vkCmdTraceRaysIndirectKHR (commandBuffer , pRaygenShaderBindingTable , pMissShaderBindingTable , pHitShaderBindingTable , pCallableShaderBindingTable , indirectDeviceAddress ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawMeshTasksIndirectCountEXT(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksIndirectCountEXT (commandBuffer , buffer , offset , countBuffer , countBufferOffset , maxDrawCount , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyPipelineCache(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyPipelineCache (device , pipelineCache , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceVideoCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoProfile: *const VkVideoProfileInfoKHR,
    pCapabilities: *mut VkVideoCapabilitiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceVideoCapabilitiesKHR (physicalDevice , pVideoProfile , pCapabilities ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetViewportWScalingNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportWScalings: *const VkViewportWScalingNV,
) {
    unimplemented!("vkCmdSetViewportWScalingNV (commandBuffer , firstViewport , viewportCount , pViewportWScalings ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDescriptorSetHostMappingVALVE(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    ppData: *mut std::ffi::c_void,
) {
    unimplemented!("vkGetDescriptorSetHostMappingVALVE (device , descriptorSet , ppData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32) {
    unimplemented!("vkCmdSetDeviceMask (commandBuffer , deviceMask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawClusterHUAWEI(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!(
        "vkCmdDrawClusterHUAWEI (commandBuffer , groupCountX , groupCountY , groupCountZ ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetColorWriteEnableEXT(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pColorWriteEnables: *const VkBool32,
) {
    unimplemented!(
        "vkCmdSetColorWriteEnableEXT (commandBuffer , attachmentCount , pColorWriteEnables ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetLineStippleEnableEXT(
    commandBuffer: VkCommandBuffer,
    stippledLineEnable: VkBool32,
) {
    unimplemented!("vkCmdSetLineStippleEnableEXT (commandBuffer , stippledLineEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetRayTracingPipelineStackSizeKHR(
    commandBuffer: VkCommandBuffer,
    pipelineStackSize: u32,
) {
    unimplemented!("vkCmdSetRayTracingPipelineStackSizeKHR (commandBuffer , pipelineStackSize ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBeginQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
    index: u32,
) {
    unimplemented!(
        "vkCmdBeginQueryIndexedEXT (commandBuffer , queryPool , query , flags , index ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCreatePrivateDataSlot(
    device: VkDevice,
    pCreateInfo: *const VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPrivateDataSlot: *mut VkPrivateDataSlot,
) -> VkResult {
    unimplemented!(
        "vkCreatePrivateDataSlot (device , pCreateInfo , pAllocator , pPrivateDataSlot ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueSubmit2(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo2,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueSubmit2 (queue , submitCount , pSubmits , fence ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateFramebuffer(
    device: VkDevice,
    pCreateInfo: *const VkFramebufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFramebuffer: *mut VkFramebuffer,
) -> VkResult {
    unimplemented!("vkCreateFramebuffer (device , pCreateInfo , pAllocator , pFramebuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceDisplayPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPropertiesKHR,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayPropertiesKHR (physicalDevice , pPropertyCount , pProperties ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyAccelerationStructureNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyAccelerationStructureNV (device , accelerationStructure , pAllocator ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthWriteEnable(
    commandBuffer: VkCommandBuffer,
    depthWriteEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthWriteEnable (commandBuffer , depthWriteEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkMapMemory2KHR(
    device: VkDevice,
    pMemoryMapInfo: *const VkMemoryMapInfoKHR,
    ppData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkMapMemory2KHR (device , pMemoryMapInfo , ppData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreatePipelineCache(
    device: VkDevice,
    pCreateInfo: *const VkPipelineCacheCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineCache: *mut VkPipelineCache,
) -> VkResult {
    unimplemented!("vkCreatePipelineCache (device , pCreateInfo , pAllocator , pPipelineCache ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetCommandPoolMemoryConsumption(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBuffer: VkCommandBuffer,
    pConsumption: *mut VkCommandPoolMemoryConsumption,
) {
    unimplemented!(
        "vkGetCommandPoolMemoryConsumption (device , commandPool , commandBuffer , pConsumption ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkReleasePerformanceConfigurationINTEL(
    device: VkDevice,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!("vkReleasePerformanceConfigurationINTEL (device , configuration ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetPolygonModeEXT(
    commandBuffer: VkCommandBuffer,
    polygonMode: VkPolygonMode,
) {
    unimplemented!("vkCmdSetPolygonModeEXT (commandBuffer , polygonMode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceExternalBufferProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
    pExternalBufferProperties: *mut VkExternalBufferProperties,
) {
    unimplemented!("vkGetPhysicalDeviceExternalBufferProperties (physicalDevice , pExternalBufferInfo , pExternalBufferProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkReleaseProfilingLockKHR(device: VkDevice) {
    unimplemented!("vkReleaseProfilingLockKHR (device ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWriteMicromapsPropertiesEXT(
    commandBuffer: VkCommandBuffer,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!("vkCmdWriteMicromapsPropertiesEXT (commandBuffer , micromapCount , pMicromaps , queryType , queryPool , firstQuery ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawMultiEXT(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pVertexInfo: *const VkMultiDrawInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawMultiEXT (commandBuffer , drawCount , pVertexInfo , instanceCount , firstInstance , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetAccelerationStructureBuildSizesKHR(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pMaxPrimitiveCounts: *const u32,
    pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
) {
    unimplemented!("vkGetAccelerationStructureBuildSizesKHR (device , buildType , pBuildInfo , pMaxPrimitiveCounts , pSizeInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetFaultData(
    device: VkDevice,
    faultQueryBehavior: VkFaultQueryBehavior,
    pUnrecordedFaults: *mut VkBool32,
    pFaultCount: *mut u32,
    pFaults: *mut VkFaultData,
) -> VkResult {
    unimplemented!("vkGetFaultData (device , faultQueryBehavior , pUnrecordedFaults , pFaultCount , pFaults ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyMicromapToMemoryEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
) {
    unimplemented!("vkCmdCopyMicromapToMemoryEXT (commandBuffer , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDebugMarkerInsertEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
) {
    unimplemented!("vkCmdDebugMarkerInsertEXT (commandBuffer , pMarkerInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPipelineExecutableStatisticsKHR(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pStatisticCount: *mut u32,
    pStatistics: *mut VkPipelineExecutableStatisticKHR,
) -> VkResult {
    unimplemented!("vkGetPipelineExecutableStatisticsKHR (device , pExecutableInfo , pStatisticCount , pStatistics ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceMultisamplePropertiesEXT(
    physicalDevice: VkPhysicalDevice,
    samples: VkSampleCountFlagBits,
    pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
) {
    unimplemented!("vkGetPhysicalDeviceMultisamplePropertiesEXT (physicalDevice , samples , pMultisampleProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkBindVideoSessionMemoryKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    bindSessionMemoryInfoCount: u32,
    pBindSessionMemoryInfos: *const VkBindVideoSessionMemoryInfoKHR,
) -> VkResult {
    unimplemented!("vkBindVideoSessionMemoryKHR (device , videoSession , bindSessionMemoryInfoCount , pBindSessionMemoryInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkBindImageMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindImageMemoryInfo,
) -> VkResult {
    unimplemented!("vkBindImageMemory2 (device , bindInfoCount , pBindInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkResetQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    unimplemented!("vkResetQueryPool (device , queryPool , firstQuery , queryCount ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkMergePipelineCaches(
    device: VkDevice,
    dstCache: VkPipelineCache,
    srcCacheCount: u32,
    pSrcCaches: *const VkPipelineCache,
) -> VkResult {
    unimplemented!("vkMergePipelineCaches (device , dstCache , srcCacheCount , pSrcCaches ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDiscardRectangleEXT(
    commandBuffer: VkCommandBuffer,
    firstDiscardRectangle: u32,
    discardRectangleCount: u32,
    pDiscardRectangles: *const VkRect2D,
) {
    unimplemented!("vkCmdSetDiscardRectangleEXT (commandBuffer , firstDiscardRectangle , discardRectangleCount , pDiscardRectangles ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyBufferToImage2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2,
) {
    unimplemented!("vkCmdCopyBufferToImage2 (commandBuffer , pCopyBufferToImageInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetScissor(
    commandBuffer: VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: *const VkRect2D,
) {
    unimplemented!("vkCmdSetScissor (commandBuffer , firstScissor , scissorCount , pScissors ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32) {
    unimplemented!("vkCmdSetLineWidth (commandBuffer , lineWidth ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDebugMarkerBeginEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
) {
    unimplemented!("vkCmdDebugMarkerBeginEXT (commandBuffer , pMarkerInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoFormatInfo: *const VkPhysicalDeviceVideoFormatInfoKHR,
    pVideoFormatPropertyCount: *mut u32,
    pVideoFormatProperties: *mut VkVideoFormatPropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceVideoFormatPropertiesKHR (physicalDevice , pVideoFormatInfo , pVideoFormatPropertyCount , pVideoFormatProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetRandROutputDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    rrOutput: RROutput,
    pDisplay: *mut VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkGetRandROutputDisplayEXT (physicalDevice , dpy , rrOutput , pDisplay ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSemaphoreFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkSemaphoreGetFdInfoKHR,
    pFd: *mut int,
) -> VkResult {
    unimplemented!("vkGetSemaphoreFdKHR (device , pGetFdInfo , pFd ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceCapabilities2KHR (physicalDevice , pSurfaceInfo , pSurfaceCapabilities ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyAccelerationStructureNV(
    commandBuffer: VkCommandBuffer,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    mode: VkCopyAccelerationStructureModeKHR,
) {
    unimplemented!("vkCmdCopyAccelerationStructureNV (commandBuffer , dst , src , mode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquireFullScreenExclusiveModeEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkAcquireFullScreenExclusiveModeEXT (device , swapchain ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateEvent(
    device: VkDevice,
    pCreateInfo: *const VkEventCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pEvent: *mut VkEvent,
) -> VkResult {
    unimplemented!("vkCreateEvent (device , pCreateInfo , pAllocator , pEvent ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyPipelineLayout(
    device: VkDevice,
    pipelineLayout: VkPipelineLayout,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyPipelineLayout (device , pipelineLayout , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkEnumerateDeviceLayerProperties(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult {
    unimplemented!(
        "vkEnumerateDeviceLayerProperties (physicalDevice , pPropertyCount , pProperties ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateWaylandSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateWaylandSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetProvokingVertexModeEXT(
    commandBuffer: VkCommandBuffer,
    provokingVertexMode: VkProvokingVertexModeEXT,
) {
    unimplemented!("vkCmdSetProvokingVertexModeEXT (commandBuffer , provokingVertexMode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetDepthClampEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthClampEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClampEnableEXT (commandBuffer , depthClampEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkWriteMicromapsPropertiesEXT(
    device: VkDevice,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    dataSize: isize,
    pData: *mut std::ffi::c_void,
    stride: isize,
) -> VkResult {
    unimplemented!("vkWriteMicromapsPropertiesEXT (device , micromapCount , pMicromaps , queryType , dataSize , pData , stride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAllocateMemory(
    device: VkDevice,
    pAllocateInfo: *const VkMemoryAllocateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pMemory: *mut VkDeviceMemory,
) -> VkResult {
    unimplemented!("vkAllocateMemory (device , pAllocateInfo , pAllocator , pMemory ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyBufferCollectionFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyBufferCollectionFUCHSIA (device , collection , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSurfacePresentModesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfacePresentModesKHR (physicalDevice , surface , pPresentModeCount , pPresentModes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCopyMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapInfoEXT,
) -> VkResult {
    unimplemented!("vkCopyMicromapEXT (device , deferredOperation , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceXlibPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    dpy: *mut Display,
    visualID: VisualID,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceXlibPresentationSupportKHR (physicalDevice , queueFamilyIndex , dpy , visualID ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCoarseSampleOrderNV(
    commandBuffer: VkCommandBuffer,
    sampleOrderType: VkCoarseSampleOrderTypeNV,
    customSampleOrderCount: u32,
    pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
) {
    unimplemented!("vkCmdSetCoarseSampleOrderNV (commandBuffer , sampleOrderType , customSampleOrderCount , pCustomSampleOrders ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateBufferView(
    device: VkDevice,
    pCreateInfo: *const VkBufferViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkBufferView,
) -> VkResult {
    unimplemented!("vkCreateBufferView (device , pCreateInfo , pAllocator , pView ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyShaderModule(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyShaderModule (device , shaderModule , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDebugUtilsMessengerEXT(
    instance: VkInstance,
    pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMessenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult {
    unimplemented!(
        "vkCreateDebugUtilsMessengerEXT (instance , pCreateInfo , pAllocator , pMessenger ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkReleaseFullScreenExclusiveModeEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkReleaseFullScreenExclusiveModeEXT (device , swapchain ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdPushDescriptorSetWithTemplateKHR(
    commandBuffer: VkCommandBuffer,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const std::ffi::c_void,
) {
    unimplemented!("vkCmdPushDescriptorSetWithTemplateKHR (commandBuffer , descriptorUpdateTemplate , layout , set , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyBufferToImage(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
) {
    unimplemented!("vkCmdCopyBufferToImage (commandBuffer , srcBuffer , dstImage , dstImageLayout , regionCount , pRegions ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawIndexed(
    commandBuffer: VkCommandBuffer,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
) {
    unimplemented!("vkCmdDrawIndexed (commandBuffer , indexCount , instanceCount , firstIndex , vertexOffset , firstInstance ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetRasterizationStreamEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationStream: u32,
) {
    unimplemented!("vkCmdSetRasterizationStreamEXT (commandBuffer , rasterizationStream ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetRayTracingShaderGroupStackSizeKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    group: u32,
    groupShader: VkShaderGroupShaderKHR,
) -> VkDeviceSize {
    unimplemented!(
        "vkGetRayTracingShaderGroupStackSizeKHR (device , pipeline , group , groupShader ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageSubresourceLayout2EXT(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource2EXT,
    pLayout: *mut VkSubresourceLayout2EXT,
) {
    unimplemented!("vkGetImageSubresourceLayout2EXT (device , image , pSubresource , pLayout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDispatchBase(
    commandBuffer: VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDispatchBase (commandBuffer , baseGroupX , baseGroupY , baseGroupZ , groupCountX , groupCountY , groupCountZ ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetAndroidHardwareBufferPropertiesANDROID(
    device: VkDevice,
    buffer: *const AHardwareBuffer,
    pProperties: *mut VkAndroidHardwareBufferPropertiesANDROID,
) -> VkResult {
    unimplemented!("vkGetAndroidHardwareBufferPropertiesANDROID (device , buffer , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDescriptorSetLayout(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSetLayout: *mut VkDescriptorSetLayout,
) -> VkResult {
    unimplemented!("vkCreateDescriptorSetLayout (device , pCreateInfo , pAllocator , pSetLayout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateScreenSurfaceQNX(
    instance: VkInstance,
    pCreateInfo: *const VkScreenSurfaceCreateInfoQNX,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateScreenSurfaceQNX (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetAttachmentFeedbackLoopEnableEXT(
    commandBuffer: VkCommandBuffer,
    aspectMask: VkImageAspectFlags,
) {
    unimplemented!("vkCmdSetAttachmentFeedbackLoopEnableEXT (commandBuffer , aspectMask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkWaitSemaphores(
    device: VkDevice,
    pWaitInfo: *const VkSemaphoreWaitInfo,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitSemaphores (device , pWaitInfo , timeout ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceImageFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    pImageFormatProperties: *mut VkImageFormatProperties,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceImageFormatProperties (physicalDevice , format , type_ , tiling , usage , flags , pImageFormatProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBuildAccelerationStructureNV(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkAccelerationStructureInfoNV,
    instanceData: VkBuffer,
    instanceOffset: VkDeviceSize,
    update: VkBool32,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    scratch: VkBuffer,
    scratchOffset: VkDeviceSize,
) {
    unimplemented!("vkCmdBuildAccelerationStructureNV (commandBuffer , pInfo , instanceData , instanceOffset , update , dst , src , scratch , scratchOffset ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceAccelerationStructureCompatibilityKHR(
    device: VkDevice,
    pVersionInfo: *const VkAccelerationStructureVersionInfoKHR,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
) {
    unimplemented!("vkGetDeviceAccelerationStructureCompatibilityKHR (device , pVersionInfo , pCompatibility ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetRasterizationSamplesEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationSamples: VkSampleCountFlagBits,
) {
    unimplemented!("vkCmdSetRasterizationSamplesEXT (commandBuffer , rasterizationSamples ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAllocateDescriptorSets(
    device: VkDevice,
    pAllocateInfo: *const VkDescriptorSetAllocateInfo,
    pDescriptorSets: *mut VkDescriptorSet,
) -> VkResult {
    unimplemented!("vkAllocateDescriptorSets (device , pAllocateInfo , pDescriptorSets ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetViewportShadingRatePaletteNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pShadingRatePalettes: *const VkShadingRatePaletteNV,
) {
    unimplemented!("vkCmdSetViewportShadingRatePaletteNV (commandBuffer , firstViewport , viewportCount , pShadingRatePalettes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetAccelerationStructureHandleNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    dataSize: isize,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!(
        "vkGetAccelerationStructureHandleNV (device , accelerationStructure , dataSize , pData ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetScissorWithCount(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const VkRect2D,
) {
    unimplemented!("vkCmdSetScissorWithCount (commandBuffer , scissorCount , pScissors ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDevice(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: *const VkDeviceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDevice: *mut VkDevice,
) -> VkResult {
    unimplemented!("vkCreateDevice (physicalDevice , pCreateInfo , pAllocator , pDevice ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBlitImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageBlit,
    filter: VkFilter,
) {
    unimplemented!("vkCmdBlitImage (commandBuffer , srcImage , srcImageLayout , dstImage , dstImageLayout , regionCount , pRegions , filter ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetEncodedVideoSessionParametersKHR(
    device: VkDevice,
    pVideoSessionParametersInfo: *const VkVideoEncodeSessionParametersGetInfoKHR,
    pFeedbackInfo: *mut VkVideoEncodeSessionParametersFeedbackInfoKHR,
    pDataSize: *mut isize,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetEncodedVideoSessionParametersKHR (device , pVideoSessionParametersInfo , pFeedbackInfo , pDataSize , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyVideoSessionParametersKHR (device , videoSessionParameters , pAllocator ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkImageCaptureDescriptorDataInfoEXT,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetImageOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSetBufferCollectionBufferConstraintsFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pBufferConstraintsInfo: *const VkBufferConstraintsInfoFUCHSIA,
) -> VkResult {
    unimplemented!("vkSetBufferCollectionBufferConstraintsFUCHSIA (device , collection , pBufferConstraintsInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetColorWriteMaskEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorWriteMasks: *const VkColorComponentFlags,
) {
    unimplemented!("vkCmdSetColorWriteMaskEXT (commandBuffer , firstAttachment , attachmentCount , pColorWriteMasks ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPipelineCacheData(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pDataSize: *mut isize,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetPipelineCacheData (device , pipelineCache , pDataSize , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSemaphoreSciSyncObjNV(
    device: VkDevice,
    pGetSciSyncInfo: *const VkSemaphoreGetSciSyncInfoNV,
    pHandle: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetSemaphoreSciSyncObjNV (device , pGetSciSyncInfo , pHandle ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawIndirectByteCountEXT(
    commandBuffer: VkCommandBuffer,
    instanceCount: u32,
    firstInstance: u32,
    counterBuffer: VkBuffer,
    counterBufferOffset: VkDeviceSize,
    counterOffset: u32,
    vertexStride: u32,
) {
    unimplemented!("vkCmdDrawIndirectByteCountEXT (commandBuffer , instanceCount , firstInstance , counterBuffer , counterBufferOffset , counterOffset , vertexStride ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWriteAccelerationStructuresPropertiesKHR(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!("vkCmdWriteAccelerationStructuresPropertiesKHR (commandBuffer , accelerationStructureCount , pAccelerationStructures , queryType , queryPool , firstQuery ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDraw(
    commandBuffer: VkCommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
) {
    unimplemented!(
        "vkCmdDraw (commandBuffer , vertexCount , instanceCount , firstVertex , firstInstance ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetColorBlendEquationEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEquations: *const VkColorBlendEquationEXT,
) {
    unimplemented!("vkCmdSetColorBlendEquationEXT (commandBuffer , firstAttachment , attachmentCount , pColorBlendEquations ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    unimplemented!("vkCmdSetEvent (commandBuffer , event , stageMask ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceImageMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!("vkGetDeviceImageMemoryRequirements (device , pInfo , pMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateValidationCacheEXT(
    device: VkDevice,
    pCreateInfo: *const VkValidationCacheCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pValidationCache: *mut VkValidationCacheEXT,
) -> VkResult {
    unimplemented!(
        "vkCreateValidationCacheEXT (device , pCreateInfo , pAllocator , pValidationCache ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdTraceRaysIndirect2KHR(
    commandBuffer: VkCommandBuffer,
    indirectDeviceAddress: VkDeviceAddress,
) {
    unimplemented!("vkCmdTraceRaysIndirect2KHR (commandBuffer , indirectDeviceAddress ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetPrimitiveRestartEnable(
    commandBuffer: VkCommandBuffer,
    primitiveRestartEnable: VkBool32,
) {
    unimplemented!("vkCmdSetPrimitiveRestartEnable (commandBuffer , primitiveRestartEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetBlendConstants(
    commandBuffer: VkCommandBuffer,
    blendConstants: [f32; 4 as usize],
) {
    unimplemented!("vkCmdSetBlendConstants (commandBuffer , blendConstants ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetAlphaToOneEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToOneEnable: VkBool32,
) {
    unimplemented!("vkCmdSetAlphaToOneEnableEXT (commandBuffer , alphaToOneEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSemaphoreCounterValue(
    device: VkDevice,
    semaphore: VkSemaphore,
    pValue: *mut u64,
) -> VkResult {
    unimplemented!("vkGetSemaphoreCounterValue (device , semaphore , pValue ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDisplayPlaneCapabilities2KHR(
    physicalDevice: VkPhysicalDevice,
    pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR,
    pCapabilities: *mut VkDisplayPlaneCapabilities2KHR,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayPlaneCapabilities2KHR (physicalDevice , pDisplayPlaneInfo , pCapabilities ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceFeatures(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures,
) {
    unimplemented!("vkGetPhysicalDeviceFeatures (physicalDevice , pFeatures ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkUpdateDescriptorSets(
    device: VkDevice,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
    descriptorCopyCount: u32,
    pDescriptorCopies: *const VkCopyDescriptorSet,
) {
    unimplemented!("vkUpdateDescriptorSets (device , descriptorWriteCount , pDescriptorWrites , descriptorCopyCount , pDescriptorCopies ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDisplayPlaneSupportedDisplaysKHR(
    physicalDevice: VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: *mut u32,
    pDisplays: *mut VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkGetDisplayPlaneSupportedDisplaysKHR (physicalDevice , planeIndex , pDisplayCount , pDisplays ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPipelineExecutablePropertiesKHR(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoKHR,
    pExecutableCount: *mut u32,
    pProperties: *mut VkPipelineExecutablePropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetPipelineExecutablePropertiesKHR (device , pPipelineInfo , pExecutableCount , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult {
    unimplemented!("vkGetFenceStatus (device , fence ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureCaptureDescriptorDataInfoEXT,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!(
        "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    display: *mut wl_display,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceWaylandPresentationSupportKHR (physicalDevice , queueFamilyIndex , display ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateMicromapEXT(
    device: VkDevice,
    pCreateInfo: *const VkMicromapCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMicromap: *mut VkMicromapEXT,
) -> VkResult {
    unimplemented!("vkCreateMicromapEXT (device , pCreateInfo , pAllocator , pMicromap ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceMemoryProperties(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
) {
    unimplemented!("vkGetPhysicalDeviceMemoryProperties (physicalDevice , pMemoryProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroySampler(
    device: VkDevice,
    sampler: VkSampler,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySampler (device , sampler , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetRasterizerDiscardEnable(
    commandBuffer: VkCommandBuffer,
    rasterizerDiscardEnable: VkBool32,
) {
    unimplemented!("vkCmdSetRasterizerDiscardEnable (commandBuffer , rasterizerDiscardEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateXcbSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateXcbSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquireImageANDROID(
    device: VkDevice,
    image: VkImage,
    nativeFenceFd: int,
    semaphore: VkSemaphore,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkAcquireImageANDROID (device , image , nativeFenceFd , semaphore , fence ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyMemoryToImageIndirectNV(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pImageSubresources: *const VkImageSubresourceLayers,
) {
    unimplemented!("vkCmdCopyMemoryToImageIndirectNV (commandBuffer , copyBufferAddress , copyCount , stride , dstImage , dstImageLayout , pImageSubresources ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSetDebugUtilsObjectTagEXT(
    device: VkDevice,
    pTagInfo: *const VkDebugUtilsObjectTagInfoEXT,
) -> VkResult {
    unimplemented!("vkSetDebugUtilsObjectTagEXT (device , pTagInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetShaderModuleIdentifierEXT(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
) {
    unimplemented!("vkGetShaderModuleIdentifierEXT (device , shaderModule , pIdentifier ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkMemoryGetFdInfoKHR,
    pFd: *mut int,
) -> VkResult {
    unimplemented!("vkGetMemoryFdKHR (device , pGetFdInfo , pFd ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndDebugUtilsLabelEXT (commandBuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCoverageReductionModeNV(
    commandBuffer: VkCommandBuffer,
    coverageReductionMode: VkCoverageReductionModeNV,
) {
    unimplemented!("vkCmdSetCoverageReductionModeNV (commandBuffer , coverageReductionMode ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdEndRendering(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndRendering (commandBuffer ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyFramebuffer(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyFramebuffer (device , framebuffer , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBuildAccelerationStructuresIndirectKHR(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pIndirectDeviceAddresses: *const VkDeviceAddress,
    pIndirectStrides: *const u32,
    ppMaxPrimitiveCounts: *const u32,
) {
    unimplemented!("vkCmdBuildAccelerationStructuresIndirectKHR (commandBuffer , infoCount , pInfos , pIndirectDeviceAddresses , pIndirectStrides , ppMaxPrimitiveCounts ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkMapMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    ppData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkMapMemory (device , memory , offset , size , flags , ppData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroySemaphoreSciSyncPoolNV(
    device: VkDevice,
    semaphorePool: VkSemaphoreSciSyncPoolNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySemaphoreSciSyncPoolNV (device , semaphorePool , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties,
) {
    unimplemented!(
        "vkGetPhysicalDeviceFormatProperties (physicalDevice , format , pFormatProperties ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyImageToBuffer(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
) {
    unimplemented!("vkCmdCopyImageToBuffer (commandBuffer , srcImage , srcImageLayout , dstBuffer , regionCount , pRegions ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceXcbPresentationSupportKHR (physicalDevice , queueFamilyIndex , connection , visual_id ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceFeatures2(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures2,
) {
    unimplemented!("vkGetPhysicalDeviceFeatures2 (physicalDevice , pFeatures ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateRayTracingPipelinesKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    unimplemented!("vkCreateRayTracingPipelinesKHR (device , deferredOperation , pipelineCache , createInfoCount , pCreateInfos , pAllocator , pPipelines ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWriteBufferMarker2AMD(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
) {
    unimplemented!(
        "vkCmdWriteBufferMarker2AMD (commandBuffer , stage , dstBuffer , dstOffset , marker ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateSemaphore(
    device: VkDevice,
    pCreateInfo: *const VkSemaphoreCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSemaphore: *mut VkSemaphore,
) -> VkResult {
    unimplemented!("vkCreateSemaphore (device , pCreateInfo , pAllocator , pSemaphore ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSwapchainGrallocUsageANDROID(
    device: VkDevice,
    format: VkFormat,
    imageUsage: VkImageUsageFlags,
    grallocUsage: *mut int,
) -> VkResult {
    unimplemented!(
        "vkGetSwapchainGrallocUsageANDROID (device , format , imageUsage , grallocUsage ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateBufferCollectionFUCHSIA(
    device: VkDevice,
    pCreateInfo: *const VkBufferCollectionCreateInfoFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
    pCollection: *mut VkBufferCollectionFUCHSIA,
) -> VkResult {
    unimplemented!(
        "vkCreateBufferCollectionFUCHSIA (device , pCreateInfo , pAllocator , pCollection ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdDrawClusterIndirectHUAWEI(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    unimplemented!("vkCmdDrawClusterIndirectHUAWEI (commandBuffer , buffer , offset ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquirePerformanceConfigurationINTEL(
    device: VkDevice,
    pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL,
    pConfiguration: *mut VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!(
        "vkAcquirePerformanceConfigurationINTEL (device , pAcquireInfo , pConfiguration ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkDebugReportMessageEXT(
    instance: VkInstance,
    flags: VkDebugReportFlagsEXT,
    objectType: VkDebugReportObjectTypeEXT,
    object: u64,
    location: isize,
    messageCode: i32,
    pLayerPrefix: *const std::ffi::c_char,
    pMessage: *const std::ffi::c_char,
) {
    unimplemented!("vkDebugReportMessageEXT (instance , flags , objectType , object , location , messageCode , pLayerPrefix , pMessage ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPipelineExecutableInternalRepresentationsKHR(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pInternalRepresentationCount: *mut u32,
    pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
) -> VkResult {
    unimplemented!("vkGetPipelineExecutableInternalRepresentationsKHR (device , pExecutableInfo , pInternalRepresentationCount , pInternalRepresentations ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSetPrivateData(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
) -> VkResult {
    unimplemented!(
        "vkSetPrivateData (device , objectType , objectHandle , privateDataSlot , data ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDescriptorSetLayoutHostMappingInfoVALVE(
    device: VkDevice,
    pBindingReference: *const VkDescriptorSetBindingReferenceVALVE,
    pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE,
) {
    unimplemented!("vkGetDescriptorSetLayoutHostMappingInfoVALVE (device , pBindingReference , pHostMapping ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateDebugReportCallbackEXT(
    instance: VkInstance,
    pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pCallback: *mut VkDebugReportCallbackEXT,
) -> VkResult {
    unimplemented!(
        "vkCreateDebugReportCallbackEXT (instance , pCreateInfo , pAllocator , pCallback ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkSetLocalDimmingAMD(
    device: VkDevice,
    swapChain: VkSwapchainKHR,
    localDimmingEnable: VkBool32,
) {
    unimplemented!("vkSetLocalDimmingAMD (device , swapChain , localDimmingEnable ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDynamicRenderingTilePropertiesQCOM(
    device: VkDevice,
    pRenderingInfo: *const VkRenderingInfo,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult {
    unimplemented!(
        "vkGetDynamicRenderingTilePropertiesQCOM (device , pRenderingInfo , pProperties ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWaitEvents2(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    pDependencyInfos: *const VkDependencyInfo,
) {
    unimplemented!("vkCmdWaitEvents2 (commandBuffer , eventCount , pEvents , pDependencyInfos ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetSemaphoreZirconHandleFUCHSIA(
    device: VkDevice,
    pGetZirconHandleInfo: *const VkSemaphoreGetZirconHandleInfoFUCHSIA,
    pZirconHandle: *mut zx_handle_t,
) -> VkResult {
    unimplemented!(
        "vkGetSemaphoreZirconHandleFUCHSIA (device , pGetZirconHandleInfo , pZirconHandle ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkEnumeratePhysicalDevices(
    instance: VkInstance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut VkPhysicalDevice,
) -> VkResult {
    unimplemented!(
        "vkEnumeratePhysicalDevices (instance , pPhysicalDeviceCount , pPhysicalDevices ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdBindIndexBuffer(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    indexType: VkIndexType,
) {
    unimplemented!("vkCmdBindIndexBuffer (commandBuffer , buffer , offset , indexType ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdClearColorImage(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pColor: *const VkClearColorValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
) {
    unimplemented!("vkCmdClearColorImage (commandBuffer , image , imageLayout , pColor , rangeCount , pRanges ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetRenderAreaGranularity(
    device: VkDevice,
    renderPass: VkRenderPass,
    pGranularity: *mut VkExtent2D,
) {
    unimplemented!("vkGetRenderAreaGranularity (device , renderPass , pGranularity ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetVertexInputEXT(
    commandBuffer: VkCommandBuffer,
    vertexBindingDescriptionCount: u32,
    pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT,
    vertexAttributeDescriptionCount: u32,
    pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT,
) {
    unimplemented!("vkCmdSetVertexInputEXT (commandBuffer , vertexBindingDescriptionCount , pVertexBindingDescriptions , vertexAttributeDescriptionCount , pVertexAttributeDescriptions ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkExportMetalObjectsEXT(
    device: VkDevice,
    pMetalObjectsInfo: *mut VkExportMetalObjectsInfoEXT,
) {
    unimplemented!("vkExportMetalObjectsEXT (device , pMetalObjectsInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateCommandPool(
    device: VkDevice,
    pCreateInfo: *const VkCommandPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pCommandPool: *mut VkCommandPool,
) -> VkResult {
    unimplemented!("vkCreateCommandPool (device , pCreateInfo , pAllocator , pCommandPool ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetMemoryFdPropertiesKHR(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: int,
    pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetMemoryFdPropertiesKHR (device , handleType , fd , pMemoryFdProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetImageDrmFormatModifierPropertiesEXT(
    device: VkDevice,
    image: VkImage,
    pProperties: *mut VkImageDrmFormatModifierPropertiesEXT,
) -> VkResult {
    unimplemented!("vkGetImageDrmFormatModifierPropertiesEXT (device , image , pProperties ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetDeviceImageSparseMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
) {
    unimplemented!("vkGetDeviceImageSparseMemoryRequirements (device , pInfo , pSparseMemoryRequirementCount , pSparseMemoryRequirements ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdOpticalFlowExecuteNV(
    commandBuffer: VkCommandBuffer,
    session: VkOpticalFlowSessionNV,
    pExecuteInfo: *const VkOpticalFlowExecuteInfoNV,
) {
    unimplemented!("vkCmdOpticalFlowExecuteNV (commandBuffer , session , pExecuteInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkAcquireNextImageKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    pImageIndex: *mut u32,
) -> VkResult {
    unimplemented!(
        "vkAcquireNextImageKHR (device , swapchain , timeout , semaphore , fence , pImageIndex ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkCreateViSurfaceNN(
    instance: VkInstance,
    pCreateInfo: *const VkViSurfaceCreateInfoNN,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateViSurfaceNN (instance , pCreateInfo , pAllocator , pSurface ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPhysicalDeviceSciBufAttributesNV(
    physicalDevice: VkPhysicalDevice,
    pAttributes: NvSciBufAttrList,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSciBufAttributesNV (physicalDevice , pAttributes ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetRayTracingShaderGroupHandlesKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: isize,
    pData: *mut std::ffi::c_void,
) -> VkResult {
    unimplemented!("vkGetRayTracingShaderGroupHandlesKHR (device , pipeline , firstGroup , groupCount , dataSize , pData ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkGetPastPresentationTimingGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pPresentationTimingCount: *mut u32,
    pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
) -> VkResult {
    unimplemented!("vkGetPastPresentationTimingGOOGLE (device , swapchain , pPresentationTimingCount , pPresentationTimings ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdFillBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
) {
    unimplemented!("vkCmdFillBuffer (commandBuffer , dstBuffer , dstOffset , size , data ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdWriteAccelerationStructuresPropertiesNV(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureNV,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!("vkCmdWriteAccelerationStructuresPropertiesNV (commandBuffer , accelerationStructureCount , pAccelerationStructures , queryType , queryPool , firstQuery ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdCopyMemoryToAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) {
    unimplemented!("vkCmdCopyMemoryToAccelerationStructureKHR (commandBuffer , pInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyFence(
    device: VkDevice,
    fence: VkFence,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyFence (device , fence , pAllocator ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkSignalSemaphore(
    device: VkDevice,
    pSignalInfo: *const VkSemaphoreSignalInfo,
) -> VkResult {
    unimplemented!("vkSignalSemaphore (device , pSignalInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkCmdSetCoverageModulationTableEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableEnable: VkBool32,
) {
    unimplemented!(
        "vkCmdSetCoverageModulationTableEnableNV (commandBuffer , coverageModulationTableEnable ,)"
    )
}
#[no_mangle]
pub(crate) extern "C" fn vkReleaseSwapchainImagesEXT(
    device: VkDevice,
    pReleaseInfo: *const VkReleaseSwapchainImagesInfoEXT,
) -> VkResult {
    unimplemented!("vkReleaseSwapchainImagesEXT (device , pReleaseInfo ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkQueueSignalReleaseImageANDROID(
    queue: VkQueue,
    waitSemaphoreCount: u32,
    pWaitSemaphores: *const VkSemaphore,
    image: VkImage,
    pNativeFenceFd: *mut int,
) -> VkResult {
    unimplemented!("vkQueueSignalReleaseImageANDROID (queue , waitSemaphoreCount , pWaitSemaphores , image , pNativeFenceFd ,)")
}
#[no_mangle]
pub(crate) extern "C" fn vkBeginCommandBuffer(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkCommandBufferBeginInfo,
) -> VkResult {
    unimplemented!("vkBeginCommandBuffer(commandBuffer, pBeginInfo)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDestroyDevice(
    device: VkDevice,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDevice(device, pAllocator)")
}
#[no_mangle]
pub(crate) extern "C" fn vkResetDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    flags: VkDescriptorPoolResetFlags,
) -> VkResult {
    unimplemented!("vkResetDescriptorPool(device, descriptorPool, flags)")
}
#[no_mangle]
pub(crate) extern "C" fn vkResetCommandBuffer(
    commandBuffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlags,
) -> VkResult {
    unimplemented!("vkResetCommandBuffer(commandBuffer, flags)")
}
#[no_mangle]
pub(crate) extern "C" fn vkDeviceWaitIdle(device: VkDevice) -> VkResult {
    unimplemented!("vkDeviceWaitIdle(device)")
}
#[no_mangle]
pub(crate) extern "C" fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult {
    unimplemented!("vkEndCommandBuffer(commandBuffer)")
}
