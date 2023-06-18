#![allow(non_snake_case)]

use crate::codegen::*;

pub(crate) fn impl_vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
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
pub(crate) fn impl_vkFreeDescriptorSets(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
) -> VkResult {
    unimplemented!(
        "vkFreeDescriptorSets (device , descriptorPool , descriptorSetCount , pDescriptorSets ,)"
    )
}
pub(crate) fn impl_vkCmdClearDepthStencilImage(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pDepthStencil: *const VkClearDepthStencilValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
) {
    unimplemented!("vkCmdClearDepthStencilImage (commandBuffer , image , imageLayout , pDepthStencil , rangeCount , pRanges ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSparseImageFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties2,
) {
    unimplemented!("vkGetPhysicalDeviceSparseImageFormatProperties2 (physicalDevice , pFormatInfo , pPropertyCount , pProperties ,)")
}
pub(crate) fn impl_vkCreateDescriptorPool(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorPool: *mut VkDescriptorPool,
) -> VkResult {
    unimplemented!("vkCreateDescriptorPool (device , pCreateInfo , pAllocator , pDescriptorPool ,)")
}
pub(crate) fn impl_vkCmdPushConstants(
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
pub(crate) fn impl_vkCmdDrawIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawIndirect (commandBuffer , buffer , offset , drawCount , stride ,)")
}
pub(crate) fn impl_vkCreateRayTracingPipelinesNV(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    unimplemented!("vkCreateRayTracingPipelinesNV (device , pipelineCache , createInfoCount , pCreateInfos , pAllocator , pPipelines ,)")
}
pub(crate) fn impl_vkCmdCopyQueryPoolResults(
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
pub(crate) fn impl_vkGetPhysicalDeviceSurfaceSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: *mut VkBool32,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceSupportKHR (physicalDevice , queueFamilyIndex , surface , pSupported ,)")
}
pub(crate) fn impl_vkImportSemaphoreZirconHandleFUCHSIA(
    device: VkDevice,
    pImportSemaphoreZirconHandleInfo: *const VkImportSemaphoreZirconHandleInfoFUCHSIA,
) -> VkResult {
    unimplemented!(
        "vkImportSemaphoreZirconHandleFUCHSIA (device , pImportSemaphoreZirconHandleInfo ,)"
    )
}
pub(crate) fn impl_vkGetSamplerOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkSamplerCaptureDescriptorDataInfoEXT,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetSamplerOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)")
}
pub(crate) fn impl_vkWaitForPresentKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    presentId: u64,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitForPresentKHR (device , swapchain , presentId , timeout ,)")
}
pub(crate) fn impl_vkCmdCopyBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferInfo: *const VkCopyBufferInfo2,
) {
    unimplemented!("vkCmdCopyBuffer2 (commandBuffer , pCopyBufferInfo ,)")
}
pub(crate) fn impl_vkGetPipelinePropertiesEXT(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoEXT,
    pPipelineProperties: *mut VkBaseOutStructure,
) -> VkResult {
    unimplemented!("vkGetPipelinePropertiesEXT (device , pPipelineInfo , pPipelineProperties ,)")
}
pub(crate) fn impl_vkCmdPushDescriptorSetKHR(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
) {
    unimplemented!("vkCmdPushDescriptorSetKHR (commandBuffer , pipelineBindPoint , layout , set , descriptorWriteCount , pDescriptorWrites ,)")
}
pub(crate) fn impl_vkGetMemoryZirconHandlePropertiesFUCHSIA(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    zirconHandle: zx_handle_t,
    pMemoryZirconHandleProperties: *mut VkMemoryZirconHandlePropertiesFUCHSIA,
) -> VkResult {
    unimplemented!("vkGetMemoryZirconHandlePropertiesFUCHSIA (device , handleType , zirconHandle , pMemoryZirconHandleProperties ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceCapabilitiesKHR (physicalDevice , surface , pSurfaceCapabilities ,)")
}
pub(crate) fn impl_vkCmdDrawIndexedIndirectCount(
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
pub(crate) fn impl_vkCmdSetLogicOpEXT(commandBuffer: VkCommandBuffer, logicOp: VkLogicOp) {
    unimplemented!("vkCmdSetLogicOpEXT (commandBuffer , logicOp ,)")
}
pub(crate) fn impl_vkCmdBindShadersEXT(
    commandBuffer: VkCommandBuffer,
    stageCount: u32,
    pStages: *const VkShaderStageFlagBits,
    pShaders: *const VkShaderEXT,
) {
    unimplemented!("vkCmdBindShadersEXT (commandBuffer , stageCount , pStages , pShaders ,)")
}
pub(crate) fn impl_vkDestroyAccelerationStructureKHR(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyAccelerationStructureKHR (device , accelerationStructure , pAllocator ,)"
    )
}
pub(crate) fn impl_vkCmdSetDepthBoundsTestEnable(
    commandBuffer: VkCommandBuffer,
    depthBoundsTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthBoundsTestEnable (commandBuffer , depthBoundsTestEnable ,)")
}
pub(crate) fn impl_vkSetDeviceMemoryPriorityEXT(
    device: VkDevice,
    memory: VkDeviceMemory,
    priority: f32,
) {
    unimplemented!("vkSetDeviceMemoryPriorityEXT (device , memory , priority ,)")
}
pub(crate) fn impl_vkUpdateDescriptorSetWithTemplate(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: *const std::ffi::c_void,
) {
    unimplemented!("vkUpdateDescriptorSetWithTemplate (device , descriptorSet , descriptorUpdateTemplate , pData ,)")
}
pub(crate) fn impl_vkQueueBindSparse(
    queue: VkQueue,
    bindInfoCount: u32,
    pBindInfo: *const VkBindSparseInfo,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueBindSparse (queue , bindInfoCount , pBindInfo , fence ,)")
}
pub(crate) fn impl_vkImportSemaphoreWin32HandleKHR(
    device: VkDevice,
    pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
) -> VkResult {
    unimplemented!("vkImportSemaphoreWin32HandleKHR (device , pImportSemaphoreWin32HandleInfo ,)")
}
pub(crate) fn impl_vkAcquireProfilingLockKHR(
    device: VkDevice,
    pInfo: *const VkAcquireProfilingLockInfoKHR,
) -> VkResult {
    unimplemented!("vkAcquireProfilingLockKHR (device , pInfo ,)")
}
pub(crate) fn impl_vkRegisterDisplayEventEXT(
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
pub(crate) fn impl_vkGetBufferMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!("vkGetBufferMemoryRequirements2 (device , pInfo , pMemoryRequirements ,)")
}
pub(crate) fn impl_vkCmdSetRepresentativeFragmentTestEnableNV(
    commandBuffer: VkCommandBuffer,
    representativeFragmentTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetRepresentativeFragmentTestEnableNV (commandBuffer , representativeFragmentTestEnable ,)")
}
pub(crate) fn impl_vkCmdEndVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pEndCodingInfo: *const VkVideoEndCodingInfoKHR,
) {
    unimplemented!("vkCmdEndVideoCodingKHR (commandBuffer , pEndCodingInfo ,)")
}
pub(crate) fn impl_vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkGetEventStatus (device , event ,)")
}
pub(crate) fn impl_vkCmdBindInvocationMaskHUAWEI(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    unimplemented!("vkCmdBindInvocationMaskHUAWEI (commandBuffer , imageView , imageLayout ,)")
}
pub(crate) fn impl_vkCreateShadersEXT(
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
pub(crate) fn impl_vkCmdSubpassShadingHUAWEI(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdSubpassShadingHUAWEI (commandBuffer ,)")
}
pub(crate) fn impl_vkDestroyDescriptorSetLayout(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDescriptorSetLayout (device , descriptorSetLayout , pAllocator ,)")
}
pub(crate) fn impl_vkCmdDrawIndexedIndirect(
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
pub(crate) fn impl_vkCreateDescriptorUpdateTemplate(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult {
    unimplemented!("vkCreateDescriptorUpdateTemplate (device , pCreateInfo , pAllocator , pDescriptorUpdateTemplate ,)")
}
pub(crate) fn impl_vkCmdCopyImageToBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2,
) {
    unimplemented!("vkCmdCopyImageToBuffer2 (commandBuffer , pCopyImageToBufferInfo ,)")
}
pub(crate) fn impl_vkGetImageMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!("vkGetImageMemoryRequirements2 (device , pInfo , pMemoryRequirements ,)")
}
pub(crate) fn impl_vkGetDeviceGroupSurfacePresentModesKHR(
    device: VkDevice,
    surface: VkSurfaceKHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupSurfacePresentModesKHR (device , surface , pModes ,)")
}
pub(crate) fn impl_vkCmdBeginConditionalRenderingEXT(
    commandBuffer: VkCommandBuffer,
    pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT,
) {
    unimplemented!(
        "vkCmdBeginConditionalRenderingEXT (commandBuffer , pConditionalRenderingBegin ,)"
    )
}
pub(crate) fn impl_vkCopyMicromapToMemoryEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
) -> VkResult {
    unimplemented!("vkCopyMicromapToMemoryEXT (device , deferredOperation , pInfo ,)")
}
pub(crate) fn impl_vkCreateMacOSSurfaceMVK(
    instance: VkInstance,
    pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateMacOSSurfaceMVK (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkCmdSetViewportWScalingEnableNV(
    commandBuffer: VkCommandBuffer,
    viewportWScalingEnable: VkBool32,
) {
    unimplemented!("vkCmdSetViewportWScalingEnableNV (commandBuffer , viewportWScalingEnable ,)")
}
pub(crate) fn impl_vkFreeMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkFreeMemory (device , memory , pAllocator ,)")
}
pub(crate) fn impl_vkDestroyDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDescriptorPool (device , descriptorPool , pAllocator ,)")
}
pub(crate) fn impl_vkCmdBlitImage2(
    commandBuffer: VkCommandBuffer,
    pBlitImageInfo: *const VkBlitImageInfo2,
) {
    unimplemented!("vkCmdBlitImage2 (commandBuffer , pBlitImageInfo ,)")
}
pub(crate) fn impl_vkFlushMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult {
    unimplemented!("vkFlushMappedMemoryRanges (device , memoryRangeCount , pMemoryRanges ,)")
}
pub(crate) fn impl_vkResetFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
) -> VkResult {
    unimplemented!("vkResetFences (device , fenceCount , pFences ,)")
}
pub(crate) fn impl_vkGetMicromapBuildSizesEXT(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkMicromapBuildInfoEXT,
    pSizeInfo: *mut VkMicromapBuildSizesInfoEXT,
) {
    unimplemented!("vkGetMicromapBuildSizesEXT (device , buildType , pBuildInfo , pSizeInfo ,)")
}
pub(crate) fn impl_vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdDebugMarkerEndEXT (commandBuffer ,)")
}
pub(crate) fn impl_vkCreateDisplayPlaneSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!(
        "vkCreateDisplayPlaneSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)"
    )
}
pub(crate) fn impl_vkCmdBeginRenderPass(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    contents: VkSubpassContents,
) {
    unimplemented!("vkCmdBeginRenderPass (commandBuffer , pRenderPassBegin , contents ,)")
}
pub(crate) fn impl_vkAcquireXlibDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireXlibDisplayEXT (physicalDevice , dpy , display ,)")
}
pub(crate) fn impl_vkCmdSetPerformanceMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceMarkerInfoINTEL,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceMarkerINTEL (commandBuffer , pMarkerInfo ,)")
}
pub(crate) fn impl_vkCmdSetPerformanceOverrideINTEL(
    commandBuffer: VkCommandBuffer,
    pOverrideInfo: *const VkPerformanceOverrideInfoINTEL,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceOverrideINTEL (commandBuffer , pOverrideInfo ,)")
}
pub(crate) fn impl_vkDestroySemaphore(
    device: VkDevice,
    semaphore: VkSemaphore,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySemaphore (device , semaphore , pAllocator ,)")
}
pub(crate) fn impl_vkCmdBeginTransformFeedbackEXT(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
) {
    unimplemented!("vkCmdBeginTransformFeedbackEXT (commandBuffer , firstCounterBuffer , counterBufferCount , pCounterBuffers , pCounterBufferOffsets ,)")
}
pub(crate) fn impl_vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents) {
    unimplemented!("vkCmdNextSubpass (commandBuffer , contents ,)")
}
pub(crate) fn impl_vkCmdEndConditionalRenderingEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndConditionalRenderingEXT (commandBuffer ,)")
}
pub(crate) fn impl_vkImportSemaphoreFdKHR(
    device: VkDevice,
    pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR,
) -> VkResult {
    unimplemented!("vkImportSemaphoreFdKHR (device , pImportSemaphoreFdInfo ,)")
}
pub(crate) fn impl_vkCreateAccelerationStructureKHR(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureKHR,
) -> VkResult {
    unimplemented!("vkCreateAccelerationStructureKHR (device , pCreateInfo , pAllocator , pAccelerationStructure ,)")
}
pub(crate) fn impl_vkCmdSetStencilReference(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    reference: u32,
) {
    unimplemented!("vkCmdSetStencilReference (commandBuffer , faceMask , reference ,)")
}
pub(crate) fn impl_vkGetShaderModuleCreateInfoIdentifierEXT(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
) {
    unimplemented!(
        "vkGetShaderModuleCreateInfoIdentifierEXT (device , pCreateInfo , pIdentifier ,)"
    )
}
pub(crate) fn impl_vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) {
    unimplemented!("vkUnmapMemory (device , memory ,)")
}
pub(crate) fn impl_vkGetDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: *mut VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkGetDrmDisplayEXT (physicalDevice , drmFd , connectorId , display ,)")
}
pub(crate) fn impl_vkGetAccelerationStructureDeviceAddressKHR(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR,
) -> VkDeviceAddress {
    unimplemented!("vkGetAccelerationStructureDeviceAddressKHR (device , pInfo ,)")
}
pub(crate) fn impl_vkGetQueueCheckpointDataNV(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointDataNV,
) {
    unimplemented!("vkGetQueueCheckpointDataNV (queue , pCheckpointDataCount , pCheckpointData ,)")
}
pub(crate) fn impl_vkGetMemoryZirconHandleFUCHSIA(
    device: VkDevice,
    pGetZirconHandleInfo: *const VkMemoryGetZirconHandleInfoFUCHSIA,
    pZirconHandle: *mut zx_handle_t,
) -> VkResult {
    unimplemented!(
        "vkGetMemoryZirconHandleFUCHSIA (device , pGetZirconHandleInfo , pZirconHandle ,)"
    )
}
pub(crate) fn impl_vkGetDeviceGroupPresentCapabilitiesKHR(
    device: VkDevice,
    pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR,
) -> VkResult {
    unimplemented!(
        "vkGetDeviceGroupPresentCapabilitiesKHR (device , pDeviceGroupPresentCapabilities ,)"
    )
}
pub(crate) fn impl_vkCmdSetCheckpointNV(
    commandBuffer: VkCommandBuffer,
    pCheckpointMarker: *const std::ffi::c_void,
) {
    unimplemented!("vkCmdSetCheckpointNV (commandBuffer , pCheckpointMarker ,)")
}
pub(crate) fn impl_vkCmdEncodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pEncodeInfo: *const VkVideoEncodeInfoKHR,
) {
    unimplemented!("vkCmdEncodeVideoKHR (commandBuffer , pEncodeInfo ,)")
}
pub(crate) fn impl_vkDestroyImageView(
    device: VkDevice,
    imageView: VkImageView,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyImageView (device , imageView , pAllocator ,)")
}
pub(crate) fn impl_vkCmdBuildMicromapsEXT(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
) {
    unimplemented!("vkCmdBuildMicromapsEXT (commandBuffer , infoCount , pInfos ,)")
}
pub(crate) fn impl_vkCmdCopyMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMicromapInfoEXT,
) {
    unimplemented!("vkCmdCopyMicromapEXT (commandBuffer , pInfo ,)")
}
pub(crate) fn impl_vkCmdSetDepthClipNegativeOneToOneEXT(
    commandBuffer: VkCommandBuffer,
    negativeOneToOne: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClipNegativeOneToOneEXT (commandBuffer , negativeOneToOne ,)")
}
pub(crate) fn impl_vkCmdSetStencilWriteMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    writeMask: u32,
) {
    unimplemented!("vkCmdSetStencilWriteMask (commandBuffer , faceMask , writeMask ,)")
}
pub(crate) fn impl_vkCmdDrawMeshTasksIndirectCountNV(
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
pub(crate) fn impl_vkGetPhysicalDeviceProperties(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties,
) {
    unimplemented!("vkGetPhysicalDeviceProperties (physicalDevice , pProperties ,)")
}
pub(crate) fn impl_vkCmdExecuteCommands(
    commandBuffer: VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
) {
    unimplemented!("vkCmdExecuteCommands (commandBuffer , commandBufferCount , pCommandBuffers ,)")
}
pub(crate) fn impl_vkCreateStreamDescriptorSurfaceGGP(
    instance: VkInstance,
    pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!(
        "vkCreateStreamDescriptorSurfaceGGP (instance , pCreateInfo , pAllocator , pSurface ,)"
    )
}
pub(crate) fn impl_vkGetVideoSessionMemoryRequirementsKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pMemoryRequirementsCount: *mut u32,
    pMemoryRequirements: *mut VkVideoSessionMemoryRequirementsKHR,
) -> VkResult {
    unimplemented!("vkGetVideoSessionMemoryRequirementsKHR (device , videoSession , pMemoryRequirementsCount , pMemoryRequirements ,)")
}
pub(crate) fn impl_vkCreateFence(
    device: VkDevice,
    pCreateInfo: *const VkFenceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult {
    unimplemented!("vkCreateFence (device , pCreateInfo , pAllocator , pFence ,)")
}
pub(crate) fn impl_vkImportFenceWin32HandleKHR(
    device: VkDevice,
    pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
) -> VkResult {
    unimplemented!("vkImportFenceWin32HandleKHR (device , pImportFenceWin32HandleInfo ,)")
}
pub(crate) fn impl_vkCmdBindDescriptorBufferEmbeddedSamplersEXT(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
) {
    unimplemented!("vkCmdBindDescriptorBufferEmbeddedSamplersEXT (commandBuffer , pipelineBindPoint , layout , set ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlanePropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceDisplayPlanePropertiesKHR (physicalDevice , pPropertyCount , pProperties ,)")
}
pub(crate) fn impl_vkGetMemorySciBufNV(
    device: VkDevice,
    pGetSciBufInfo: *const VkMemoryGetSciBufInfoNV,
    pHandle: *mut NvSciBufObj,
) -> VkResult {
    unimplemented!("vkGetMemorySciBufNV (device , pGetSciBufInfo , pHandle ,)")
}
pub(crate) fn impl_vkEnumeratePhysicalDeviceGroups(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: *mut u32,
    pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult {
    unimplemented!("vkEnumeratePhysicalDeviceGroups (instance , pPhysicalDeviceGroupCount , pPhysicalDeviceGroupProperties ,)")
}
pub(crate) fn impl_vkGetDescriptorEXT(
    device: VkDevice,
    pDescriptorInfo: *const VkDescriptorGetInfoEXT,
    dataSize: isize,
    pDescriptor: *mut std :: ffi :: c_void,
) {
    unimplemented!("vkGetDescriptorEXT (device , pDescriptorInfo , dataSize , pDescriptor ,)")
}
pub(crate) fn impl_vkGetSwapchainGrallocUsage2ANDROID(
    device: VkDevice,
    format: VkFormat,
    imageUsage: VkImageUsageFlags,
    swapchainImageUsage: VkSwapchainImageUsageFlagsANDROID,
    grallocConsumerUsage: *mut u64,
    grallocProducerUsage: *mut u64,
) -> VkResult {
    unimplemented!("vkGetSwapchainGrallocUsage2ANDROID (device , format , imageUsage , swapchainImageUsage , grallocConsumerUsage , grallocProducerUsage ,)")
}
pub(crate) fn impl_vkCmdSetColorBlendEnableEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEnables: *const VkBool32,
) {
    unimplemented!("vkCmdSetColorBlendEnableEXT (commandBuffer , firstAttachment , attachmentCount , pColorBlendEnables ,)")
}
pub(crate) fn impl_vkDestroyQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyQueryPool (device , queryPool , pAllocator ,)")
}
pub(crate) fn impl_vkCopyAccelerationStructureToMemoryKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) -> VkResult {
    unimplemented!("vkCopyAccelerationStructureToMemoryKHR (device , deferredOperation , pInfo ,)")
}
pub(crate) fn impl_vkCreateSampler(
    device: VkDevice,
    pCreateInfo: *const VkSamplerCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSampler: *mut VkSampler,
) -> VkResult {
    unimplemented!("vkCreateSampler (device , pCreateInfo , pAllocator , pSampler ,)")
}
pub(crate) fn impl_vkCmdSetDepthTestEnable(
    commandBuffer: VkCommandBuffer,
    depthTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthTestEnable (commandBuffer , depthTestEnable ,)")
}
pub(crate) fn impl_vkCmdCopyMemoryToMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
) {
    unimplemented!("vkCmdCopyMemoryToMicromapEXT (commandBuffer , pInfo ,)")
}
pub(crate) fn impl_vkCmdCopyImage2(
    commandBuffer: VkCommandBuffer,
    pCopyImageInfo: *const VkCopyImageInfo2,
) {
    unimplemented!("vkCmdCopyImage2 (commandBuffer , pCopyImageInfo ,)")
}
pub(crate) fn impl_vkGetShaderBinaryDataEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pDataSize: *mut isize,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetShaderBinaryDataEXT (device , shader , pDataSize , pData ,)")
}
pub(crate) fn impl_vkAcquireNextImage2KHR(
    device: VkDevice,
    pAcquireInfo: *const VkAcquireNextImageInfoKHR,
    pImageIndex: *mut u32,
) -> VkResult {
    unimplemented!("vkAcquireNextImage2KHR (device , pAcquireInfo , pImageIndex ,)")
}
pub(crate) fn impl_vkCmdSetDiscardRectangleEnableEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDiscardRectangleEnableEXT (commandBuffer , discardRectangleEnable ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceScreenPresentationSupportQNX(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    window: *mut _screen_window,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceScreenPresentationSupportQNX (physicalDevice , queueFamilyIndex , window ,)")
}
pub(crate) fn impl_vkCmdBeginVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkVideoBeginCodingInfoKHR,
) {
    unimplemented!("vkCmdBeginVideoCodingKHR (commandBuffer , pBeginInfo ,)")
}
pub(crate) fn impl_vkCmdSetFragmentShadingRateKHR(
    commandBuffer: VkCommandBuffer,
    pFragmentSize: *const VkExtent2D,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2 as usize],
) {
    unimplemented!("vkCmdSetFragmentShadingRateKHR (commandBuffer , pFragmentSize , combinerOps ,)")
}
pub(crate) fn impl_vkCmdWriteBufferMarkerAMD(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
) {
    unimplemented!("vkCmdWriteBufferMarkerAMD (commandBuffer , pipelineStage , dstBuffer , dstOffset , marker ,)")
}
pub(crate) fn impl_vkUpdateVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR,
) -> VkResult {
    unimplemented!(
        "vkUpdateVideoSessionParametersKHR (device , videoSessionParameters , pUpdateInfo ,)"
    )
}
pub(crate) fn impl_vkDestroyCuModuleNVX(
    device: VkDevice,
    module: VkCuModuleNVX,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyCuModuleNVX (device , module , pAllocator ,)")
}
pub(crate) fn impl_vkGetImageSparseMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
) {
    unimplemented!("vkGetImageSparseMemoryRequirements (device , image , pSparseMemoryRequirementCount , pSparseMemoryRequirements ,)")
}
pub(crate) fn impl_vkGetFenceWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult {
    unimplemented!("vkGetFenceWin32HandleKHR (device , pGetWin32HandleInfo , pHandle ,)")
}
pub(crate) fn impl_vkCmdNextSubpass2(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
    pSubpassEndInfo: *const VkSubpassEndInfo,
) {
    unimplemented!("vkCmdNextSubpass2 (commandBuffer , pSubpassBeginInfo , pSubpassEndInfo ,)")
}
pub(crate) fn impl_vkGetAccelerationStructureMemoryRequirementsNV(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2KHR,
) {
    unimplemented!(
        "vkGetAccelerationStructureMemoryRequirementsNV (device , pInfo , pMemoryRequirements ,)"
    )
}
pub(crate) fn impl_vkCmdSetLogicOpEnableEXT(
    commandBuffer: VkCommandBuffer,
    logicOpEnable: VkBool32,
) {
    unimplemented!("vkCmdSetLogicOpEnableEXT (commandBuffer , logicOpEnable ,)")
}
pub(crate) fn impl_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
    device: VkDevice,
    renderpass: VkRenderPass,
    pMaxWorkgroupSize: *mut VkExtent2D,
) -> VkResult {
    unimplemented!("vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI (device , renderpass , pMaxWorkgroupSize ,)")
}
pub(crate) fn impl_vkCmdBeginQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
) {
    unimplemented!("vkCmdBeginQuery (commandBuffer , queryPool , query , flags ,)")
}
pub(crate) fn impl_vkCreateAndroidSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateAndroidSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkCmdPipelineBarrier(
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
pub(crate) fn impl_vkGetPhysicalDeviceSurfacePresentModes2EXT(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfacePresentModes2EXT (physicalDevice , pSurfaceInfo , pPresentModeCount , pPresentModes ,)")
}
pub(crate) fn impl_vkCmdSetDepthCompareOp(
    commandBuffer: VkCommandBuffer,
    depthCompareOp: VkCompareOp,
) {
    unimplemented!("vkCmdSetDepthCompareOp (commandBuffer , depthCompareOp ,)")
}
pub(crate) fn impl_vkCmdDecompressMemoryIndirectCountNV(
    commandBuffer: VkCommandBuffer,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    stride: u32,
) {
    unimplemented!("vkCmdDecompressMemoryIndirectCountNV (commandBuffer , indirectCommandsAddress , indirectCommandsCountAddress , stride ,)")
}
pub(crate) fn impl_vkImportFenceFdKHR(
    device: VkDevice,
    pImportFenceFdInfo: *const VkImportFenceFdInfoKHR,
) -> VkResult {
    unimplemented!("vkImportFenceFdKHR (device , pImportFenceFdInfo ,)")
}
pub(crate) fn impl_vkCmdSetAlphaToCoverageEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToCoverageEnable: VkBool32,
) {
    unimplemented!("vkCmdSetAlphaToCoverageEnableEXT (commandBuffer , alphaToCoverageEnable ,)")
}
pub(crate) fn impl_vkCmdWriteTimestamp2(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdWriteTimestamp2 (commandBuffer , stage , queryPool , query ,)")
}
pub(crate) fn impl_vkCmdTraceRaysKHR(
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
pub(crate) fn impl_vkCreateQueryPool(
    device: VkDevice,
    pCreateInfo: *const VkQueryPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pQueryPool: *mut VkQueryPool,
) -> VkResult {
    unimplemented!("vkCreateQueryPool (device , pCreateInfo , pAllocator , pQueryPool ,)")
}
pub(crate) fn impl_vkCreateSharedSwapchainsKHR(
    device: VkDevice,
    swapchainCount: u32,
    pCreateInfos: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchains: *mut VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkCreateSharedSwapchainsKHR (device , swapchainCount , pCreateInfos , pAllocator , pSwapchains ,)")
}
pub(crate) fn impl_vkGetCalibratedTimestampsEXT(
    device: VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const VkCalibratedTimestampInfoEXT,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> VkResult {
    unimplemented!("vkGetCalibratedTimestampsEXT (device , timestampCount , pTimestampInfos , pTimestamps , pMaxDeviation ,)")
}
pub(crate) fn impl_vkQueueBeginDebugUtilsLabelEXT(
    queue: VkQueue,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    unimplemented!("vkQueueBeginDebugUtilsLabelEXT (queue , pLabelInfo ,)")
}
pub(crate) fn impl_vkUnmapMemory2KHR(
    device: VkDevice,
    pMemoryUnmapInfo: *const VkMemoryUnmapInfoKHR,
) -> VkResult {
    unimplemented!("vkUnmapMemory2KHR (device , pMemoryUnmapInfo ,)")
}
pub(crate) fn impl_vkCmdSetDiscardRectangleModeEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleMode: VkDiscardRectangleModeEXT,
) {
    unimplemented!("vkCmdSetDiscardRectangleModeEXT (commandBuffer , discardRectangleMode ,)")
}
pub(crate) fn impl_vkDestroyIndirectCommandsLayoutNV(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyIndirectCommandsLayoutNV (device , indirectCommandsLayout , pAllocator ,)"
    )
}
pub(crate) fn impl_vkInitializePerformanceApiINTEL(
    device: VkDevice,
    pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL,
) -> VkResult {
    unimplemented!("vkInitializePerformanceApiINTEL (device , pInitializeInfo ,)")
}
pub(crate) fn impl_vkCreateRenderPass(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult {
    unimplemented!("vkCreateRenderPass (device , pCreateInfo , pAllocator , pRenderPass ,)")
}
pub(crate) fn impl_vkCmdRefreshObjectsKHR(
    commandBuffer: VkCommandBuffer,
    pRefreshObjects: *const VkRefreshObjectListKHR,
) {
    unimplemented!("vkCmdRefreshObjectsKHR (commandBuffer , pRefreshObjects ,)")
}
pub(crate) fn impl_vkCreateDisplayModeKHR(
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
pub(crate) fn impl_vkCmdDecodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pDecodeInfo: *const VkVideoDecodeInfoKHR,
) {
    unimplemented!("vkCmdDecodeVideoKHR (commandBuffer , pDecodeInfo ,)")
}
pub(crate) fn impl_vkCmdSetConservativeRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
) {
    unimplemented!("vkCmdSetConservativeRasterizationModeEXT (commandBuffer , conservativeRasterizationMode ,)")
}
pub(crate) fn impl_vkGetSwapchainCounterEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    counter: VkSurfaceCounterFlagBitsEXT,
    pCounterValue: *mut u64,
) -> VkResult {
    unimplemented!("vkGetSwapchainCounterEXT (device , swapchain , counter , pCounterValue ,)")
}
pub(crate) fn impl_vkBuildAccelerationStructuresKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const VkAccelerationStructureBuildRangeInfoKHR,
) -> VkResult {
    unimplemented!("vkBuildAccelerationStructuresKHR (device , deferredOperation , infoCount , pInfos , ppBuildRangeInfos ,)")
}
pub(crate) fn impl_vkDestroyDescriptorUpdateTemplate(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyDescriptorUpdateTemplate (device , descriptorUpdateTemplate , pAllocator ,)"
    )
}
pub(crate) fn impl_vkCmdBuildAccelerationStructuresKHR(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const VkAccelerationStructureBuildRangeInfoKHR,
) {
    unimplemented!("vkCmdBuildAccelerationStructuresKHR (commandBuffer , infoCount , pInfos , ppBuildRangeInfos ,)")
}
pub(crate) fn impl_vkCmdUpdateBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    dataSize: VkDeviceSize,
    pData: *const std::ffi::c_void,
) {
    unimplemented!("vkCmdUpdateBuffer (commandBuffer , dstBuffer , dstOffset , dataSize , pData ,)")
}
pub(crate) fn impl_vkCmdBindDescriptorSets(
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
pub(crate) fn impl_vkGetMemoryAndroidHardwareBufferANDROID(
    device: VkDevice,
    pInfo: *const VkMemoryGetAndroidHardwareBufferInfoANDROID,
    pBuffer: *mut AHardwareBuffer,
) -> VkResult {
    unimplemented!("vkGetMemoryAndroidHardwareBufferANDROID (device , pInfo , pBuffer ,)")
}
pub(crate) fn impl_vkCreateIndirectCommandsLayoutNV(
    device: VkDevice,
    pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV,
) -> VkResult {
    unimplemented!("vkCreateIndirectCommandsLayoutNV (device , pCreateInfo , pAllocator , pIndirectCommandsLayout ,)")
}
pub(crate) fn impl_vkCreateImagePipeSurfaceFUCHSIA(
    instance: VkInstance,
    pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!(
        "vkCreateImagePipeSurfaceFUCHSIA (instance , pCreateInfo , pAllocator , pSurface ,)"
    )
}
pub(crate) fn impl_vkCopyAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
) -> VkResult {
    unimplemented!("vkCopyAccelerationStructureKHR (device , deferredOperation , pInfo ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pQualityLevelInfo: *const VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR,
    pQualityLevelProperties: *mut VkVideoEncodeQualityLevelPropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR (physicalDevice , pQualityLevelInfo , pQualityLevelProperties ,)")
}
pub(crate) fn impl_vkQueueEndDebugUtilsLabelEXT(queue: VkQueue) {
    unimplemented!("vkQueueEndDebugUtilsLabelEXT (queue ,)")
}
pub(crate) fn impl_vkCmdDispatch(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDispatch (commandBuffer , groupCountX , groupCountY , groupCountZ ,)")
}
pub(crate) fn impl_vkCmdSetLineRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    lineRasterizationMode: VkLineRasterizationModeEXT,
) {
    unimplemented!("vkCmdSetLineRasterizationModeEXT (commandBuffer , lineRasterizationMode ,)")
}
pub(crate) fn impl_vkEnumerateDeviceExtensionProperties(
    physicalDevice: VkPhysicalDevice,
    pLayerName: *const std::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult {
    unimplemented!("vkEnumerateDeviceExtensionProperties (physicalDevice , pLayerName , pPropertyCount , pProperties ,)")
}
pub(crate) fn impl_vkCreateRenderPass2(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult {
    unimplemented!("vkCreateRenderPass2 (device , pCreateInfo , pAllocator , pRenderPass ,)")
}
pub(crate) fn impl_vkCreateOpticalFlowSessionNV(
    device: VkDevice,
    pCreateInfo: *const VkOpticalFlowSessionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pSession: *mut VkOpticalFlowSessionNV,
) -> VkResult {
    unimplemented!("vkCreateOpticalFlowSessionNV (device , pCreateInfo , pAllocator , pSession ,)")
}
pub(crate) fn impl_vkQueueSetPerformanceConfigurationINTEL(
    queue: VkQueue,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!("vkQueueSetPerformanceConfigurationINTEL (queue , configuration ,)")
}
pub(crate) fn impl_vkDebugMarkerSetObjectTagEXT(
    device: VkDevice,
    pTagInfo: *const VkDebugMarkerObjectTagInfoEXT,
) -> VkResult {
    unimplemented!("vkDebugMarkerSetObjectTagEXT (device , pTagInfo ,)")
}
pub(crate) fn impl_vkDeferredOperationJoinKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkDeferredOperationJoinKHR (device , operation ,)")
}
pub(crate) fn impl_vkCmdEndQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdEndQuery (commandBuffer , queryPool , query ,)")
}
pub(crate) fn impl_vkGetMemoryHostPointerPropertiesEXT(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHostPointer: *const std::ffi::c_void,
    pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult {
    unimplemented!("vkGetMemoryHostPointerPropertiesEXT (device , handleType , pHostPointer , pMemoryHostPointerProperties ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceDisplayProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayProperties2KHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceDisplayProperties2KHR (physicalDevice , pPropertyCount , pProperties ,)")
}
pub(crate) fn impl_vkCmdPipelineBarrier2(
    commandBuffer: VkCommandBuffer,
    pDependencyInfo: *const VkDependencyInfo,
) {
    unimplemented!("vkCmdPipelineBarrier2 (commandBuffer , pDependencyInfo ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceCapabilities2EXT (physicalDevice , surface , pSurfaceCapabilities ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSurfaceFormats2KHR(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormat2KHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceFormats2KHR (physicalDevice , pSurfaceInfo , pSurfaceFormatCount , pSurfaceFormats ,)")
}
pub(crate) fn impl_vkCreateDeferredOperationKHR(
    device: VkDevice,
    pAllocator: *const VkAllocationCallbacks,
    pDeferredOperation: *mut VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkCreateDeferredOperationKHR (device , pAllocator , pDeferredOperation ,)")
}
pub(crate) fn impl_vkCmdSetDescriptorBufferOffsetsEXT(
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
pub(crate) fn impl_vkGetPhysicalDeviceSurfaceFormatsKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormatKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceFormatsKHR (physicalDevice , surface , pSurfaceFormatCount , pSurfaceFormats ,)")
}
pub(crate) fn impl_vkGetDeferredOperationMaxConcurrencyKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> u32 {
    unimplemented!("vkGetDeferredOperationMaxConcurrencyKHR (device , operation ,)")
}
pub(crate) fn impl_vkCmdSetTessellationDomainOriginEXT(
    commandBuffer: VkCommandBuffer,
    domainOrigin: VkTessellationDomainOrigin,
) {
    unimplemented!("vkCmdSetTessellationDomainOriginEXT (commandBuffer , domainOrigin ,)")
}
pub(crate) fn impl_vkCreateSwapchainKHR(
    device: VkDevice,
    pCreateInfo: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchain: *mut VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkCreateSwapchainKHR (device , pCreateInfo , pAllocator , pSwapchain ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceWin32PresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceWin32PresentationSupportKHR (physicalDevice , queueFamilyIndex ,)"
    )
}
pub(crate) fn impl_vkQueueInsertDebugUtilsLabelEXT(
    queue: VkQueue,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    unimplemented!("vkQueueInsertDebugUtilsLabelEXT (queue , pLabelInfo ,)")
}
pub(crate) fn impl_vkSetHdrMetadataEXT(
    device: VkDevice,
    swapchainCount: u32,
    pSwapchains: *const VkSwapchainKHR,
    pMetadata: *const VkHdrMetadataEXT,
) {
    unimplemented!("vkSetHdrMetadataEXT (device , swapchainCount , pSwapchains , pMetadata ,)")
}
pub(crate) fn impl_vkCompileDeferredNV(
    device: VkDevice,
    pipeline: VkPipeline,
    shader: u32,
) -> VkResult {
    unimplemented!("vkCompileDeferredNV (device , pipeline , shader ,)")
}
pub(crate) fn impl_vkCmdBindVertexBuffers(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
) {
    unimplemented!("vkCmdBindVertexBuffers (commandBuffer , firstBinding , bindingCount , pBuffers , pOffsets ,)")
}
pub(crate) fn impl_vkCreateVideoSessionKHR(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSession: *mut VkVideoSessionKHR,
) -> VkResult {
    unimplemented!("vkCreateVideoSessionKHR (device , pCreateInfo , pAllocator , pVideoSession ,)")
}
pub(crate) fn impl_vkCopyMemoryToMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
) -> VkResult {
    unimplemented!("vkCopyMemoryToMicromapEXT (device , deferredOperation , pInfo ,)")
}
pub(crate) fn impl_vkGetDeviceQueue(
    device: VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: *mut VkQueue,
) {
    unimplemented!("vkGetDeviceQueue (device , queueFamilyIndex , queueIndex , pQueue ,)")
}
pub(crate) fn impl_vkCmdSetCoverageToColorLocationNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorLocation: u32,
) {
    unimplemented!("vkCmdSetCoverageToColorLocationNV (commandBuffer , coverageToColorLocation ,)")
}
pub(crate) fn impl_vkCmdDrawMeshTasksIndirectEXT(
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
pub(crate) fn impl_vkGetImageViewHandleNVX(
    device: VkDevice,
    pInfo: *const VkImageViewHandleInfoNVX,
) -> u32 {
    unimplemented!("vkGetImageViewHandleNVX (device , pInfo ,)")
}
pub(crate) fn impl_vkDestroyPrivateDataSlot(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyPrivateDataSlot (device , privateDataSlot , pAllocator ,)")
}
pub(crate) fn impl_vkGetDescriptorSetLayoutSupport(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pSupport: *mut VkDescriptorSetLayoutSupport,
) {
    unimplemented!("vkGetDescriptorSetLayoutSupport (device , pCreateInfo , pSupport ,)")
}
pub(crate) fn impl_vkCmdBeginDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    unimplemented!("vkCmdBeginDebugUtilsLabelEXT (commandBuffer , pLabelInfo ,)")
}
pub(crate) fn impl_vkWriteAccelerationStructuresPropertiesKHR(
    device: VkDevice,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    dataSize: isize,
    pData: *mut std :: ffi :: c_void,
    stride: isize,
) -> VkResult {
    unimplemented!("vkWriteAccelerationStructuresPropertiesKHR (device , accelerationStructureCount , pAccelerationStructures , queryType , dataSize , pData , stride ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceCooperativeMatrixPropertiesNV (physicalDevice , pPropertyCount , pProperties ,)")
}
pub(crate) fn impl_vkTrimCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
) {
    unimplemented!("vkTrimCommandPool (device , commandPool , flags ,)")
}
pub(crate) fn impl_vkEnumerateInstanceLayerProperties(
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult {
    unimplemented!("vkEnumerateInstanceLayerProperties (pPropertyCount , pProperties ,)")
}
pub(crate) fn impl_vkCmdSetSampleMaskEXT(
    commandBuffer: VkCommandBuffer,
    samples: VkSampleCountFlagBits,
    pSampleMask: *const VkSampleMask,
) {
    unimplemented!("vkCmdSetSampleMaskEXT (commandBuffer , samples , pSampleMask ,)")
}
pub(crate) fn impl_vkDestroyEvent(
    device: VkDevice,
    event: VkEvent,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyEvent (device , event , pAllocator ,)")
}
pub(crate) fn impl_vkGetValidationCacheDataEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pDataSize: *mut isize,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetValidationCacheDataEXT (device , validationCache , pDataSize , pData ,)")
}
pub(crate) fn impl_vkCreateImageView(
    device: VkDevice,
    pCreateInfo: *const VkImageViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkImageView,
) -> VkResult {
    unimplemented!("vkCreateImageView (device , pCreateInfo , pAllocator , pView ,)")
}
pub(crate) fn impl_vkCmdSetViewportWithCount(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const VkViewport,
) {
    unimplemented!("vkCmdSetViewportWithCount (commandBuffer , viewportCount , pViewports ,)")
}
pub(crate) fn impl_vkCmdSetExclusiveScissorNV(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissors: *const VkRect2D,
) {
    unimplemented!("vkCmdSetExclusiveScissorNV (commandBuffer , firstExclusiveScissor , exclusiveScissorCount , pExclusiveScissors ,)")
}
pub(crate) fn impl_vkDestroyRenderPass(
    device: VkDevice,
    renderPass: VkRenderPass,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyRenderPass (device , renderPass , pAllocator ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlaneProperties2KHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceDisplayPlaneProperties2KHR (physicalDevice , pPropertyCount , pProperties ,)")
}
pub(crate) fn impl_vkCreateInstance(
    pCreateInfo: *const VkInstanceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pInstance: *mut VkInstance,
) -> VkResult {
    println!("Hello from vkCreateInstance()!");
    unsafe {
        *pInstance = std::ptr::null_mut();
    }
    // TODO: Create and register internal VkInstance.
    VkResult::VK_SUCCESS
}
pub(crate) fn impl_vkGetPhysicalDeviceToolProperties(
    physicalDevice: VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolProperties,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceToolProperties (physicalDevice , pToolCount , pToolProperties ,)"
    )
}
pub(crate) fn impl_vkCmdBindPipeline(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
) {
    unimplemented!("vkCmdBindPipeline (commandBuffer , pipelineBindPoint , pipeline ,)")
}
pub(crate) fn impl_vkCmdBindTransformFeedbackBuffersEXT(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
) {
    unimplemented!("vkCmdBindTransformFeedbackBuffersEXT (commandBuffer , firstBinding , bindingCount , pBuffers , pOffsets , pSizes ,)")
}
pub(crate) fn impl_vkGetBufferDeviceAddress(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress {
    unimplemented!("vkGetBufferDeviceAddress (device , pInfo ,)")
}
pub(crate) fn impl_vkDestroyDeferredOperationKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDeferredOperationKHR (device , operation , pAllocator ,)")
}
pub(crate) fn impl_vkGetDescriptorSetLayoutSizeEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    pLayoutSizeInBytes: *mut VkDeviceSize,
) {
    unimplemented!("vkGetDescriptorSetLayoutSizeEXT (device , layout , pLayoutSizeInBytes ,)")
}
pub(crate) fn impl_vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> VkResult {
    unimplemented!("vkEnumerateInstanceVersion (pApiVersion ,)")
}
pub(crate) fn impl_vkCmdSetPerformanceStreamMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceStreamMarkerINTEL (commandBuffer , pMarkerInfo ,)")
}
pub(crate) fn impl_vkDestroySwapchainKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySwapchainKHR (device , swapchain , pAllocator ,)")
}
pub(crate) fn impl_vkCopyMemoryToAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) -> VkResult {
    unimplemented!("vkCopyMemoryToAccelerationStructureKHR (device , deferredOperation , pInfo ,)")
}
pub(crate) fn impl_vkGetQueryPoolResults(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dataSize: isize,
    pData: *mut std :: ffi :: c_void,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) -> VkResult {
    unimplemented!("vkGetQueryPoolResults (device , queryPool , firstQuery , queryCount , dataSize , pData , stride , flags ,)")
}
pub(crate) fn impl_vkMergeValidationCachesEXT(
    device: VkDevice,
    dstCache: VkValidationCacheEXT,
    srcCacheCount: u32,
    pSrcCaches: *const VkValidationCacheEXT,
) -> VkResult {
    unimplemented!("vkMergeValidationCachesEXT (device , dstCache , srcCacheCount , pSrcCaches ,)")
}
pub(crate) fn impl_vkGetDisplayPlaneCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    mode: VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayPlaneCapabilitiesKHR (physicalDevice , mode , planeIndex , pCapabilities ,)"
    )
}
pub(crate) fn impl_vkCmdDrawMeshTasksNV(
    commandBuffer: VkCommandBuffer,
    taskCount: u32,
    firstTask: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksNV (commandBuffer , taskCount , firstTask ,)")
}
pub(crate) fn impl_vkGetPerformanceParameterINTEL(
    device: VkDevice,
    parameter: VkPerformanceParameterTypeINTEL,
    pValue: *mut VkPerformanceValueINTEL,
) -> VkResult {
    unimplemented!("vkGetPerformanceParameterINTEL (device , parameter , pValue ,)")
}
pub(crate) fn impl_vkCmdDecompressMemoryNV(
    commandBuffer: VkCommandBuffer,
    decompressRegionCount: u32,
    pDecompressMemoryRegions: *const VkDecompressMemoryRegionNV,
) {
    unimplemented!("vkCmdDecompressMemoryNV (commandBuffer , decompressRegionCount , pDecompressMemoryRegions ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    dfb: *mut IDirectFB,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceDirectFBPresentationSupportEXT (physicalDevice , queueFamilyIndex , dfb ,)")
}
pub(crate) fn impl_vkGetFenceSciSyncFenceNV(
    device: VkDevice,
    pGetSciSyncHandleInfo: *const VkFenceGetSciSyncInfoNV,
    pHandle: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetFenceSciSyncFenceNV (device , pGetSciSyncHandleInfo , pHandle ,)")
}
pub(crate) fn impl_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut VkPerformanceCounterKHR,
    pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR,
) -> VkResult {
    unimplemented!("vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR (physicalDevice , queueFamilyIndex , pCounterCount , pCounters , pCounterDescriptions ,)")
}
pub(crate) fn impl_vkCreateBuffer(
    device: VkDevice,
    pCreateInfo: *const VkBufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pBuffer: *mut VkBuffer,
) -> VkResult {
    unimplemented!("vkCreateBuffer (device , pCreateInfo , pAllocator , pBuffer ,)")
}
pub(crate) fn impl_vkGetQueueCheckpointData2NV(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointData2NV,
) {
    unimplemented!("vkGetQueueCheckpointData2NV (queue , pCheckpointDataCount , pCheckpointData ,)")
}
pub(crate) fn impl_vkGetSwapchainImagesKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut VkImage,
) -> VkResult {
    unimplemented!(
        "vkGetSwapchainImagesKHR (device , swapchain , pSwapchainImageCount , pSwapchainImages ,)"
    )
}
pub(crate) fn impl_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
    physicalDevice: VkPhysicalDevice,
    pCombinationCount: *mut u32,
    pCombinations: *mut VkFramebufferMixedSamplesCombinationNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV (physicalDevice , pCombinationCount , pCombinations ,)")
}
pub(crate) fn impl_vkCreateShaderModule(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pShaderModule: *mut VkShaderModule,
) -> VkResult {
    unimplemented!("vkCreateShaderModule (device , pCreateInfo , pAllocator , pShaderModule ,)")
}
pub(crate) fn impl_vkGetMemoryWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandleKHR (device , pGetWin32HandleInfo , pHandle ,)")
}
pub(crate) fn impl_vkGetDeviceMicromapCompatibilityEXT(
    device: VkDevice,
    pVersionInfo: *const VkMicromapVersionInfoEXT,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
) {
    unimplemented!("vkGetDeviceMicromapCompatibilityEXT (device , pVersionInfo , pCompatibility ,)")
}
pub(crate) fn impl_vkCreateImage(
    device: VkDevice,
    pCreateInfo: *const VkImageCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pImage: *mut VkImage,
) -> VkResult {
    unimplemented!("vkCreateImage (device , pCreateInfo , pAllocator , pImage ,)")
}
pub(crate) fn impl_vkReleaseDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkReleaseDisplayEXT (physicalDevice , display ,)")
}
pub(crate) fn impl_vkCmdDrawMeshTasksEXT(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!(
        "vkCmdDrawMeshTasksEXT (commandBuffer , groupCountX , groupCountY , groupCountZ ,)"
    )
}
pub(crate) fn impl_vkImportFenceSciSyncFenceNV(
    device: VkDevice,
    pImportFenceSciSyncInfo: *const VkImportFenceSciSyncInfoNV,
) -> VkResult {
    unimplemented!("vkImportFenceSciSyncFenceNV (device , pImportFenceSciSyncInfo ,)")
}
pub(crate) fn impl_vkGetBufferOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> u64 {
    unimplemented!("vkGetBufferOpaqueCaptureAddress (device , pInfo ,)")
}
pub(crate) fn impl_vkCmdSetStencilCompareMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    compareMask: u32,
) {
    unimplemented!("vkCmdSetStencilCompareMask (commandBuffer , faceMask , compareMask ,)")
}
pub(crate) fn impl_vkGetDeviceGroupPeerMemoryFeatures(
    device: VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags,
) {
    unimplemented!("vkGetDeviceGroupPeerMemoryFeatures (device , heapIndex , localDeviceIndex , remoteDeviceIndex , pPeerMemoryFeatures ,)")
}
pub(crate) fn impl_vkCmdResolveImage2(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: *const VkResolveImageInfo2,
) {
    unimplemented!("vkCmdResolveImage2 (commandBuffer , pResolveImageInfo ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceOpticalFlowImageFormatsNV(
    physicalDevice: VkPhysicalDevice,
    pOpticalFlowImageFormatInfo: *const VkOpticalFlowImageFormatInfoNV,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut VkOpticalFlowImageFormatPropertiesNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceOpticalFlowImageFormatsNV (physicalDevice , pOpticalFlowImageFormatInfo , pFormatCount , pImageFormatProperties ,)")
}
pub(crate) fn impl_vkDisplayPowerControlEXT(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayPowerInfo: *const VkDisplayPowerInfoEXT,
) -> VkResult {
    unimplemented!("vkDisplayPowerControlEXT (device , display , pDisplayPowerInfo ,)")
}
pub(crate) fn impl_vkCmdSetCoverageModulationTableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableCount: u32,
    pCoverageModulationTable: *const f32,
) {
    unimplemented!("vkCmdSetCoverageModulationTableNV (commandBuffer , coverageModulationTableCount , pCoverageModulationTable ,)")
}
pub(crate) fn impl_vkCmdSetPatchControlPointsEXT(
    commandBuffer: VkCommandBuffer,
    patchControlPoints: u32,
) {
    unimplemented!("vkCmdSetPatchControlPointsEXT (commandBuffer , patchControlPoints ,)")
}
pub(crate) fn impl_vkDestroyVideoSessionKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyVideoSessionKHR (device , videoSession , pAllocator ,)")
}
pub(crate) fn impl_vkDestroyMicromapEXT(
    device: VkDevice,
    micromap: VkMicromapEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyMicromapEXT (device , micromap , pAllocator ,)")
}
pub(crate) fn impl_vkQueueWaitIdle(queue: VkQueue) -> VkResult {
    unimplemented!("vkQueueWaitIdle (queue ,)")
}
pub(crate) fn impl_vkCmdTraceRaysNV(
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
pub(crate) fn impl_vkCmdSetFragmentShadingRateEnumNV(
    commandBuffer: VkCommandBuffer,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2 as usize],
) {
    unimplemented!(
        "vkCmdSetFragmentShadingRateEnumNV (commandBuffer , shadingRate , combinerOps ,)"
    )
}
pub(crate) fn impl_vkCmdSetViewport(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: *const VkViewport,
) {
    unimplemented!(
        "vkCmdSetViewport (commandBuffer , firstViewport , viewportCount , pViewports ,)"
    )
}
pub(crate) fn impl_vkBindBufferMemory(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    unimplemented!("vkBindBufferMemory (device , buffer , memory , memoryOffset ,)")
}
pub(crate) fn impl_vkRegisterDeviceEventEXT(
    device: VkDevice,
    pDeviceEventInfo: *const VkDeviceEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult {
    unimplemented!("vkRegisterDeviceEventEXT (device , pDeviceEventInfo , pAllocator , pFence ,)")
}
pub(crate) fn impl_vkGetImageMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: *mut VkMemoryRequirements,
) {
    unimplemented!("vkGetImageMemoryRequirements (device , image , pMemoryRequirements ,)")
}
pub(crate) fn impl_vkDestroyValidationCacheEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyValidationCacheEXT (device , validationCache , pAllocator ,)")
}
pub(crate) fn impl_vkCmdDrawIndirectCount(
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
pub(crate) fn impl_vkDestroyShaderEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyShaderEXT (device , shader , pAllocator ,)")
}
pub(crate) fn impl_vkCmdDrawMultiIndexedEXT(
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
pub(crate) fn impl_vkWaitForFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
    waitAll: VkBool32,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitForFences (device , fenceCount , pFences , waitAll , timeout ,)")
}
pub(crate) fn impl_vkGetDeferredOperationResultKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkGetDeferredOperationResultKHR (device , operation ,)")
}
pub(crate) fn impl_vkCmdSetDepthClipEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthClipEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClipEnableEXT (commandBuffer , depthClipEnable ,)")
}
pub(crate) fn impl_vkCreateSamplerYcbcrConversion(
    device: VkDevice,
    pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult {
    unimplemented!(
        "vkCreateSamplerYcbcrConversion (device , pCreateInfo , pAllocator , pYcbcrConversion ,)"
    )
}
pub(crate) fn impl_vkCmdSetPrimitiveTopology(
    commandBuffer: VkCommandBuffer,
    primitiveTopology: VkPrimitiveTopology,
) {
    unimplemented!("vkCmdSetPrimitiveTopology (commandBuffer , primitiveTopology ,)")
}
pub(crate) fn impl_vkGetFenceFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkFenceGetFdInfoKHR,
    pFd: *mut int,
) -> VkResult {
    unimplemented!("vkGetFenceFdKHR (device , pGetFdInfo , pFd ,)")
}
pub(crate) fn impl_vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndRenderPass (commandBuffer ,)")
}
pub(crate) fn impl_vkCmdExecuteGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
) {
    unimplemented!("vkCmdExecuteGeneratedCommandsNV (commandBuffer , isPreprocessed , pGeneratedCommandsInfo ,)")
}
pub(crate) fn impl_vkGetMemoryWin32HandlePropertiesKHR(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandlePropertiesKHR (device , handleType , handle , pMemoryWin32HandleProperties ,)")
}
pub(crate) fn impl_vkGetBufferMemoryRequirements(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: *mut VkMemoryRequirements,
) {
    unimplemented!("vkGetBufferMemoryRequirements (device , buffer , pMemoryRequirements ,)")
}
pub(crate) fn impl_vkCmdDrawMeshTasksIndirectNV(
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
pub(crate) fn impl_vkGetImageViewOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkImageViewCaptureDescriptorDataInfoEXT,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetImageViewOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)")
}
pub(crate) fn impl_vkCmdResetQueryPool(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    unimplemented!("vkCmdResetQueryPool (commandBuffer , queryPool , firstQuery , queryCount ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceProperties2(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties2,
) {
    unimplemented!("vkGetPhysicalDeviceProperties2 (physicalDevice , pProperties ,)")
}
pub(crate) fn impl_vkCmdBeginRenderPass2(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
) {
    unimplemented!("vkCmdBeginRenderPass2 (commandBuffer , pRenderPassBegin , pSubpassBeginInfo ,)")
}
pub(crate) fn impl_vkCmdSetSampleLocationsEnableEXT(
    commandBuffer: VkCommandBuffer,
    sampleLocationsEnable: VkBool32,
) {
    unimplemented!("vkCmdSetSampleLocationsEnableEXT (commandBuffer , sampleLocationsEnable ,)")
}
pub(crate) fn impl_vkGetPrivateData(
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
pub(crate) fn impl_vkCreateCuModuleNVX(
    device: VkDevice,
    pCreateInfo: *const VkCuModuleCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pModule: *mut VkCuModuleNVX,
) -> VkResult {
    unimplemented!("vkCreateCuModuleNVX (device , pCreateInfo , pAllocator , pModule ,)")
}
pub(crate) fn impl_vkGetBufferOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkBufferCaptureDescriptorDataInfoEXT,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetBufferOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)")
}
pub(crate) fn impl_vkGetDeviceFaultInfoEXT(
    device: VkDevice,
    pFaultCounts: *mut VkDeviceFaultCountsEXT,
    pFaultInfo: *mut VkDeviceFaultInfoEXT,
) -> VkResult {
    unimplemented!("vkGetDeviceFaultInfoEXT (device , pFaultCounts , pFaultInfo ,)")
}
pub(crate) fn impl_vkCmdSetSampleLocationsEXT(
    commandBuffer: VkCommandBuffer,
    pSampleLocationsInfo: *const VkSampleLocationsInfoEXT,
) {
    unimplemented!("vkCmdSetSampleLocationsEXT (commandBuffer , pSampleLocationsInfo ,)")
}
pub(crate) fn impl_vkCmdWriteTimestamp(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdWriteTimestamp (commandBuffer , pipelineStage , queryPool , query ,)")
}
pub(crate) fn impl_vkGetMemoryRemoteAddressNV(
    device: VkDevice,
    pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV,
    pAddress: *mut VkRemoteAddressNV,
) -> VkResult {
    unimplemented!("vkGetMemoryRemoteAddressNV (device , pMemoryGetRemoteAddressInfo , pAddress ,)")
}
pub(crate) fn impl_vkGetDeviceQueue2(
    device: VkDevice,
    pQueueInfo: *const VkDeviceQueueInfo2,
    pQueue: *mut VkQueue,
) {
    unimplemented!("vkGetDeviceQueue2 (device , pQueueInfo , pQueue ,)")
}
pub(crate) fn impl_vkCmdSetStencilOp(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
) {
    unimplemented!("vkCmdSetStencilOp (commandBuffer , faceMask , failOp , passOp , depthFailOp , compareOp ,)")
}
pub(crate) fn impl_vkCmdSetCullMode(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags) {
    unimplemented!("vkCmdSetCullMode (commandBuffer , cullMode ,)")
}
pub(crate) fn impl_vkCmdSetDepthBias(
    commandBuffer: VkCommandBuffer,
    depthBiasConstantFactor: f32,
    depthBiasClamp: f32,
    depthBiasSlopeFactor: f32,
) {
    unimplemented!("vkCmdSetDepthBias (commandBuffer , depthBiasConstantFactor , depthBiasClamp , depthBiasSlopeFactor ,)")
}
pub(crate) fn impl_vkFreeCommandBuffers(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
) {
    unimplemented!(
        "vkFreeCommandBuffers (device , commandPool , commandBufferCount , pCommandBuffers ,)"
    )
}
pub(crate) fn impl_vkGetBufferCollectionPropertiesFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pProperties: *mut VkBufferCollectionPropertiesFUCHSIA,
) -> VkResult {
    unimplemented!("vkGetBufferCollectionPropertiesFUCHSIA (device , collection , pProperties ,)")
}
pub(crate) fn impl_vkCmdSetStencilTestEnable(
    commandBuffer: VkCommandBuffer,
    stencilTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetStencilTestEnable (commandBuffer , stencilTestEnable ,)")
}
pub(crate) fn impl_vkCmdDispatchIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    unimplemented!("vkCmdDispatchIndirect (commandBuffer , buffer , offset ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    physicalDevice: VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut VkTimeDomainEXT,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceCalibrateableTimeDomainsEXT (physicalDevice , pTimeDomainCount , pTimeDomains ,)")
}
pub(crate) fn impl_vkCmdBindPipelineShaderGroupNV(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    groupIndex: u32,
) {
    unimplemented!("vkCmdBindPipelineShaderGroupNV (commandBuffer , pipelineBindPoint , pipeline , groupIndex ,)")
}
pub(crate) fn impl_vkCmdSetViewportSwizzleNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportSwizzles: *const VkViewportSwizzleNV,
) {
    unimplemented!("vkCmdSetViewportSwizzleNV (commandBuffer , firstViewport , viewportCount , pViewportSwizzles ,)")
}
pub(crate) fn impl_vkCmdEndRenderPass2(
    commandBuffer: VkCommandBuffer,
    pSubpassEndInfo: *const VkSubpassEndInfo,
) {
    unimplemented!("vkCmdEndRenderPass2 (commandBuffer , pSubpassEndInfo ,)")
}
pub(crate) fn impl_vkDestroySamplerYcbcrConversion(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySamplerYcbcrConversion (device , ycbcrConversion , pAllocator ,)")
}
pub(crate) fn impl_vkCmdSetLineStippleEXT(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    unimplemented!(
        "vkCmdSetLineStippleEXT (commandBuffer , lineStippleFactor , lineStipplePattern ,)"
    )
}
pub(crate) fn impl_vkGetDisplayModePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModePropertiesKHR,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayModePropertiesKHR (physicalDevice , display , pPropertyCount , pProperties ,)"
    )
}
pub(crate) fn impl_vkCreateMetalSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: *const VkMetalSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateMetalSurfaceEXT (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkGetDeviceBufferMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!("vkGetDeviceBufferMemoryRequirements (device , pInfo , pMemoryRequirements ,)")
}
pub(crate) fn impl_vkCreateWin32SurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateWin32SurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkGetSemaphoreWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult {
    unimplemented!("vkGetSemaphoreWin32HandleKHR (device , pGetWin32HandleInfo , pHandle ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSciSyncAttributesNV(
    physicalDevice: VkPhysicalDevice,
    pSciSyncAttributesInfo: *const VkSciSyncAttributesInfoNV,
    pAttributes: NvSciSyncAttrList,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSciSyncAttributesNV (physicalDevice , pSciSyncAttributesInfo , pAttributes ,)")
}
pub(crate) fn impl_vkGetImageSubresourceLayout(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource,
    pLayout: *mut VkSubresourceLayout,
) {
    unimplemented!("vkGetImageSubresourceLayout (device , image , pSubresource , pLayout ,)")
}
pub(crate) fn impl_vkCmdBindDescriptorBuffersEXT(
    commandBuffer: VkCommandBuffer,
    bufferCount: u32,
    pBindingInfos: *const VkDescriptorBufferBindingInfoEXT,
) {
    unimplemented!("vkCmdBindDescriptorBuffersEXT (commandBuffer , bufferCount , pBindingInfos ,)")
}
pub(crate) fn impl_vkGetFenceSciSyncObjNV(
    device: VkDevice,
    pGetSciSyncHandleInfo: *const VkFenceGetSciSyncInfoNV,
    pHandle: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetFenceSciSyncObjNV (device , pGetSciSyncHandleInfo , pHandle ,)")
}
pub(crate) fn impl_vkGetImageSparseMemoryRequirements2(
    device: VkDevice,
    pInfo: *const VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
) {
    unimplemented!("vkGetImageSparseMemoryRequirements2 (device , pInfo , pSparseMemoryRequirementCount , pSparseMemoryRequirements ,)")
}
pub(crate) fn impl_vkCmdBindShadingRateImageNV(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    unimplemented!("vkCmdBindShadingRateImageNV (commandBuffer , imageView , imageLayout ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
    physicalDevice: VkPhysicalDevice,
    pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR,
    pNumPasses: *mut u32,
) {
    unimplemented!("vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR (physicalDevice , pPerformanceQueryCreateInfo , pNumPasses ,)")
}
pub(crate) fn impl_vkDestroyImage(
    device: VkDevice,
    image: VkImage,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyImage (device , image , pAllocator ,)")
}
pub(crate) fn impl_vkResetCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolResetFlags,
) -> VkResult {
    unimplemented!("vkResetCommandPool (device , commandPool , flags ,)")
}
pub(crate) fn impl_vkGetSwapchainStatusKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkGetSwapchainStatusKHR (device , swapchain ,)")
}
pub(crate) fn impl_vkCmdResetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
) {
    unimplemented!("vkCmdResetEvent2 (commandBuffer , event , stageMask ,)")
}
pub(crate) fn impl_vkDestroyDebugReportCallbackEXT(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDebugReportCallbackEXT (instance , callback , pAllocator ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceQueueFamilyProperties(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties,
) {
    unimplemented!("vkGetPhysicalDeviceQueueFamilyProperties (physicalDevice , pQueueFamilyPropertyCount , pQueueFamilyProperties ,)")
}
pub(crate) fn impl_vkCmdCopyBuffer(
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
pub(crate) fn impl_vkCmdSetColorBlendAdvancedEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendAdvanced: *const VkColorBlendAdvancedEXT,
) {
    unimplemented!("vkCmdSetColorBlendAdvancedEXT (commandBuffer , firstAttachment , attachmentCount , pColorBlendAdvanced ,)")
}
pub(crate) fn impl_vkDestroyOpticalFlowSessionNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyOpticalFlowSessionNV (device , session , pAllocator ,)")
}
pub(crate) fn impl_vkCreateComputePipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkComputePipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    unimplemented!("vkCreateComputePipelines (device , pipelineCache , createInfoCount , pCreateInfos , pAllocator , pPipelines ,)")
}
pub(crate) fn impl_vkCreateGraphicsPipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkGraphicsPipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult {
    unimplemented!("vkCreateGraphicsPipelines (device , pipelineCache , createInfoCount , pCreateInfos , pAllocator , pPipelines ,)")
}
pub(crate) fn impl_vkCmdWaitEvents(
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
pub(crate) fn impl_vkGetPhysicalDeviceExternalSemaphoreProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
    pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
) {
    unimplemented!("vkGetPhysicalDeviceExternalSemaphoreProperties (physicalDevice , pExternalSemaphoreInfo , pExternalSemaphoreProperties ,)")
}
pub(crate) fn impl_vkImportFenceSciSyncObjNV(
    device: VkDevice,
    pImportFenceSciSyncInfo: *const VkImportFenceSciSyncInfoNV,
) -> VkResult {
    unimplemented!("vkImportFenceSciSyncObjNV (device , pImportFenceSciSyncInfo ,)")
}
pub(crate) fn impl_vkGetRefreshCycleDurationGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE,
) -> VkResult {
    unimplemented!(
        "vkGetRefreshCycleDurationGOOGLE (device , swapchain , pDisplayTimingProperties ,)"
    )
}
pub(crate) fn impl_vkCmdSetFrontFace(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace) {
    unimplemented!("vkCmdSetFrontFace (commandBuffer , frontFace ,)")
}
pub(crate) fn impl_vkBuildMicromapsEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
) -> VkResult {
    unimplemented!("vkBuildMicromapsEXT (device , deferredOperation , infoCount , pInfos ,)")
}
pub(crate) fn impl_vkGetDescriptorSetLayoutBindingOffsetEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    binding: u32,
    pOffset: *mut VkDeviceSize,
) {
    unimplemented!(
        "vkGetDescriptorSetLayoutBindingOffsetEXT (device , layout , binding , pOffset ,)"
    )
}
pub(crate) fn impl_vkGetFramebufferTilePropertiesQCOM(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pPropertiesCount: *mut u32,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult {
    unimplemented!("vkGetFramebufferTilePropertiesQCOM (device , framebuffer , pPropertiesCount , pProperties ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSparseImageFormatProperties(
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
pub(crate) fn impl_vkCmdCopyMemoryIndirectNV(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdCopyMemoryIndirectNV (commandBuffer , copyBufferAddress , copyCount , stride ,)"
    )
}
pub(crate) fn impl_vkEnumerateInstanceExtensionProperties(
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
pub(crate) fn impl_vkSubmitDebugUtilsMessageEXT(
    instance: VkInstance,
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
) {
    unimplemented!("vkSubmitDebugUtilsMessageEXT (instance , messageSeverity , messageTypes , pCallbackData ,)")
}
pub(crate) fn impl_vkCreateVideoSessionParametersKHR(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionParametersCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSessionParameters: *mut VkVideoSessionParametersKHR,
) -> VkResult {
    unimplemented!("vkCreateVideoSessionParametersKHR (device , pCreateInfo , pAllocator , pVideoSessionParameters ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceFragmentShadingRatesKHR(
    physicalDevice: VkPhysicalDevice,
    pFragmentShadingRateCount: *mut u32,
    pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceFragmentShadingRatesKHR (physicalDevice , pFragmentShadingRateCount , pFragmentShadingRates ,)")
}
pub(crate) fn impl_vkGetGeneratedCommandsMemoryRequirementsNV(
    device: VkDevice,
    pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!(
        "vkGetGeneratedCommandsMemoryRequirementsNV (device , pInfo , pMemoryRequirements ,)"
    )
}
pub(crate) fn impl_vkCmdSetCoverageToColorEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorEnable: VkBool32,
) {
    unimplemented!("vkCmdSetCoverageToColorEnableNV (commandBuffer , coverageToColorEnable ,)")
}
pub(crate) fn impl_vkQueuePresentKHR(
    queue: VkQueue,
    pPresentInfo: *const VkPresentInfoKHR,
) -> VkResult {
    unimplemented!("vkQueuePresentKHR (queue , pPresentInfo ,)")
}
pub(crate) fn impl_vkAcquireDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireDrmDisplayEXT (physicalDevice , drmFd , display ,)")
}
pub(crate) fn impl_vkCreatePipelineLayout(
    device: VkDevice,
    pCreateInfo: *const VkPipelineLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineLayout: *mut VkPipelineLayout,
) -> VkResult {
    unimplemented!("vkCreatePipelineLayout (device , pCreateInfo , pAllocator , pPipelineLayout ,)")
}
pub(crate) fn impl_vkGetDeviceProcAddr(
    device: VkDevice,
    pName: *const std::ffi::c_char,
) -> PFN_vkVoidFunction {
    unimplemented!("vkGetDeviceProcAddr (device , pName ,)")
}
pub(crate) fn impl_vkCreateXlibSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateXlibSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkDebugMarkerSetObjectNameEXT(
    device: VkDevice,
    pNameInfo: *const VkDebugMarkerObjectNameInfoEXT,
) -> VkResult {
    unimplemented!("vkDebugMarkerSetObjectNameEXT (device , pNameInfo ,)")
}
pub(crate) fn impl_vkSetDebugUtilsObjectNameEXT(
    device: VkDevice,
    pNameInfo: *const VkDebugUtilsObjectNameInfoEXT,
) -> VkResult {
    unimplemented!("vkSetDebugUtilsObjectNameEXT (device , pNameInfo ,)")
}
pub(crate) fn impl_vkGetDeviceMemoryCommitment(
    device: VkDevice,
    memory: VkDeviceMemory,
    pCommittedMemoryInBytes: *mut VkDeviceSize,
) {
    unimplemented!("vkGetDeviceMemoryCommitment (device , memory , pCommittedMemoryInBytes ,)")
}
pub(crate) fn impl_vkCmdEndQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    index: u32,
) {
    unimplemented!("vkCmdEndQueryIndexedEXT (commandBuffer , queryPool , query , index ,)")
}
pub(crate) fn impl_vkCmdSetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: *const VkDependencyInfo,
) {
    unimplemented!("vkCmdSetEvent2 (commandBuffer , event , pDependencyInfo ,)")
}
pub(crate) fn impl_vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkResetEvent (device , event ,)")
}
pub(crate) fn impl_vkCmdCopyAccelerationStructureToMemoryKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) {
    unimplemented!("vkCmdCopyAccelerationStructureToMemoryKHR (commandBuffer , pInfo ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: NvSciBufObj,
    pMemorySciBufProperties: *mut VkMemorySciBufPropertiesNV,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV (physicalDevice , handleType , handle , pMemorySciBufProperties ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceRefreshableObjectTypesKHR(
    physicalDevice: VkPhysicalDevice,
    pRefreshableObjectTypeCount: *mut u32,
    pRefreshableObjectTypes: *mut VkObjectType,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceRefreshableObjectTypesKHR (physicalDevice , pRefreshableObjectTypeCount , pRefreshableObjectTypes ,)")
}
pub(crate) fn impl_vkCreateCuFunctionNVX(
    device: VkDevice,
    pCreateInfo: *const VkCuFunctionCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pFunction: *mut VkCuFunctionNVX,
) -> VkResult {
    unimplemented!("vkCreateCuFunctionNVX (device , pCreateInfo , pAllocator , pFunction ,)")
}
pub(crate) fn impl_vkGetWinrtDisplayNV(
    physicalDevice: VkPhysicalDevice,
    deviceRelativeId: u32,
    pDisplay: *mut VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkGetWinrtDisplayNV (physicalDevice , deviceRelativeId , pDisplay ,)")
}
pub(crate) fn impl_vkCmdSetShadingRateImageEnableNV(
    commandBuffer: VkCommandBuffer,
    shadingRateImageEnable: VkBool32,
) {
    unimplemented!("vkCmdSetShadingRateImageEnableNV (commandBuffer , shadingRateImageEnable ,)")
}
pub(crate) fn impl_vkDestroyBuffer(
    device: VkDevice,
    buffer: VkBuffer,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyBuffer (device , buffer , pAllocator ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceMemoryProperties2(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
) {
    unimplemented!("vkGetPhysicalDeviceMemoryProperties2 (physicalDevice , pMemoryProperties ,)")
}
pub(crate) fn impl_vkCmdSetExclusiveScissorEnableNV(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissorEnables: *const VkBool32,
) {
    unimplemented!("vkCmdSetExclusiveScissorEnableNV (commandBuffer , firstExclusiveScissor , exclusiveScissorCount , pExclusiveScissorEnables ,)")
}
pub(crate) fn impl_vkImportSemaphoreSciSyncObjNV(
    device: VkDevice,
    pImportSemaphoreSciSyncInfo: *const VkImportSemaphoreSciSyncInfoNV,
) -> VkResult {
    unimplemented!("vkImportSemaphoreSciSyncObjNV (device , pImportSemaphoreSciSyncInfo ,)")
}
pub(crate) fn impl_vkDestroySurfaceKHR(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySurfaceKHR (instance , surface , pAllocator ,)")
}
pub(crate) fn impl_vkCreateDirectFBSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateDirectFBSurfaceEXT (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkBindImageMemory(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    unimplemented!("vkBindImageMemory (device , image , memory , memoryOffset ,)")
}
pub(crate) fn impl_vkCmdControlVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pCodingControlInfo: *const VkVideoCodingControlInfoKHR,
) {
    unimplemented!("vkCmdControlVideoCodingKHR (commandBuffer , pCodingControlInfo ,)")
}
pub(crate) fn impl_vkCmdBeginRendering(
    commandBuffer: VkCommandBuffer,
    pRenderingInfo: *const VkRenderingInfo,
) {
    unimplemented!("vkCmdBeginRendering (commandBuffer , pRenderingInfo ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties2,
) {
    unimplemented!(
        "vkGetPhysicalDeviceFormatProperties2 (physicalDevice , format , pFormatProperties ,)"
    )
}
pub(crate) fn impl_vkCmdInsertDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
) {
    unimplemented!("vkCmdInsertDebugUtilsLabelEXT (commandBuffer , pLabelInfo ,)")
}
pub(crate) fn impl_vkCmdEndTransformFeedbackEXT(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
) {
    unimplemented!("vkCmdEndTransformFeedbackEXT (commandBuffer , firstCounterBuffer , counterBufferCount , pCounterBuffers , pCounterBufferOffsets ,)")
}
pub(crate) fn impl_vkCmdPreprocessGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
) {
    unimplemented!("vkCmdPreprocessGeneratedCommandsNV (commandBuffer , pGeneratedCommandsInfo ,)")
}
pub(crate) fn impl_vkCmdSetDepthBiasEnable(
    commandBuffer: VkCommandBuffer,
    depthBiasEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthBiasEnable (commandBuffer , depthBiasEnable ,)")
}
pub(crate) fn impl_vkQueueSubmit(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueSubmit (queue , submitCount , pSubmits , fence ,)")
}
pub(crate) fn impl_vkCmdResolveImage(
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
pub(crate) fn impl_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: isize,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetRayTracingCaptureReplayShaderGroupHandlesKHR (device , pipeline , firstGroup , groupCount , dataSize , pData ,)")
}
pub(crate) fn impl_vkBindOpticalFlowSessionImageNV(
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
pub(crate) fn impl_vkAcquireWinrtDisplayNV(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireWinrtDisplayNV (physicalDevice , display ,)")
}
pub(crate) fn impl_vkGetDeviceGroupSurfacePresentModes2EXT(
    device: VkDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupSurfacePresentModes2EXT (device , pSurfaceInfo , pModes ,)")
}
pub(crate) fn impl_vkGetPhysicalDevicePresentRectanglesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pRectCount: *mut u32,
    pRects: *mut VkRect2D,
) -> VkResult {
    unimplemented!("vkGetPhysicalDevicePresentRectanglesKHR (physicalDevice , surface , pRectCount , pRects ,)")
}
pub(crate) fn impl_vkCreateHeadlessSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateHeadlessSurfaceEXT (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkGetDisplayModeProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModeProperties2KHR,
) -> VkResult {
    unimplemented!("vkGetDisplayModeProperties2KHR (physicalDevice , display , pPropertyCount , pProperties ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceImageFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
    pImageFormatProperties: *mut VkImageFormatProperties2,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceImageFormatProperties2 (physicalDevice , pImageFormatInfo , pImageFormatProperties ,)")
}
pub(crate) fn impl_vkCmdCuLaunchKernelNVX(
    commandBuffer: VkCommandBuffer,
    pLaunchInfo: *const VkCuLaunchInfoNVX,
) {
    unimplemented!("vkCmdCuLaunchKernelNVX (commandBuffer , pLaunchInfo ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceQueueFamilyProperties2(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2,
) {
    unimplemented!("vkGetPhysicalDeviceQueueFamilyProperties2 (physicalDevice , pQueueFamilyPropertyCount , pQueueFamilyProperties ,)")
}
pub(crate) fn impl_vkCmdBindVertexBuffers2(
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
pub(crate) fn impl_vkBindBufferMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindBufferMemoryInfo,
) -> VkResult {
    unimplemented!("vkBindBufferMemory2 (device , bindInfoCount , pBindInfos ,)")
}
pub(crate) fn impl_vkCreateIOSSurfaceMVK(
    instance: VkInstance,
    pCreateInfo: *const VkIOSSurfaceCreateInfoMVK,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateIOSSurfaceMVK (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkDestroyPipeline(
    device: VkDevice,
    pipeline: VkPipeline,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyPipeline (device , pipeline , pAllocator ,)")
}
pub(crate) fn impl_vkCmdSetExtraPrimitiveOverestimationSizeEXT(
    commandBuffer: VkCommandBuffer,
    extraPrimitiveOverestimationSize: f32,
) {
    unimplemented!("vkCmdSetExtraPrimitiveOverestimationSizeEXT (commandBuffer , extraPrimitiveOverestimationSize ,)")
}
pub(crate) fn impl_vkCmdCopyAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
) {
    unimplemented!("vkCmdCopyAccelerationStructureKHR (commandBuffer , pInfo ,)")
}
pub(crate) fn impl_vkGetImageViewAddressNVX(
    device: VkDevice,
    imageView: VkImageView,
    pProperties: *mut VkImageViewAddressPropertiesNVX,
) -> VkResult {
    unimplemented!("vkGetImageViewAddressNVX (device , imageView , pProperties ,)")
}
pub(crate) fn impl_vkDestroyCuFunctionNVX(
    device: VkDevice,
    function: VkCuFunctionNVX,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyCuFunctionNVX (device , function , pAllocator ,)")
}
pub(crate) fn impl_vkCmdClearAttachments(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pAttachments: *const VkClearAttachment,
    rectCount: u32,
    pRects: *const VkClearRect,
) {
    unimplemented!("vkCmdClearAttachments (commandBuffer , attachmentCount , pAttachments , rectCount , pRects ,)")
}
pub(crate) fn impl_vkInvalidateMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult {
    unimplemented!("vkInvalidateMappedMemoryRanges (device , memoryRangeCount , pMemoryRanges ,)")
}
pub(crate) fn impl_vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkSetEvent (device , event ,)")
}
pub(crate) fn impl_vkCmdSetDepthBounds(
    commandBuffer: VkCommandBuffer,
    minDepthBounds: f32,
    maxDepthBounds: f32,
) {
    unimplemented!("vkCmdSetDepthBounds (commandBuffer , minDepthBounds , maxDepthBounds ,)")
}
pub(crate) fn impl_vkDestroyBufferView(
    device: VkDevice,
    bufferView: VkBufferView,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyBufferView (device , bufferView , pAllocator ,)")
}
pub(crate) fn impl_vkCmdResetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    unimplemented!("vkCmdResetEvent (commandBuffer , event , stageMask ,)")
}
pub(crate) fn impl_vkGetMemoryWin32HandleNV(
    device: VkDevice,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    pHandle: *mut HANDLE,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandleNV (device , memory , handleType , pHandle ,)")
}
pub(crate) fn impl_vkSetBufferCollectionImageConstraintsFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pImageConstraintsInfo: *const VkImageConstraintsInfoFUCHSIA,
) -> VkResult {
    unimplemented!("vkSetBufferCollectionImageConstraintsFUCHSIA (device , collection , pImageConstraintsInfo ,)")
}
pub(crate) fn impl_vkCreateAccelerationStructureNV(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureNV,
) -> VkResult {
    unimplemented!("vkCreateAccelerationStructureNV (device , pCreateInfo , pAllocator , pAccelerationStructure ,)")
}
pub(crate) fn impl_vkCmdCopyImage(
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
pub(crate) fn impl_vkCmdSetCoverageModulationModeNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationMode: VkCoverageModulationModeNV,
) {
    unimplemented!("vkCmdSetCoverageModulationModeNV (commandBuffer , coverageModulationMode ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceExternalFenceProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
    pExternalFenceProperties: *mut VkExternalFenceProperties,
) {
    unimplemented!("vkGetPhysicalDeviceExternalFenceProperties (physicalDevice , pExternalFenceInfo , pExternalFenceProperties ,)")
}
pub(crate) fn impl_vkAllocateCommandBuffers(
    device: VkDevice,
    pAllocateInfo: *const VkCommandBufferAllocateInfo,
    pCommandBuffers: *mut VkCommandBuffer,
) -> VkResult {
    unimplemented!("vkAllocateCommandBuffers (device , pAllocateInfo , pCommandBuffers ,)")
}
pub(crate) fn impl_vkGetShaderInfoAMD(
    device: VkDevice,
    pipeline: VkPipeline,
    shaderStage: VkShaderStageFlagBits,
    infoType: VkShaderInfoTypeAMD,
    pInfoSize: *mut isize,
    pInfo: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!(
        "vkGetShaderInfoAMD (device , pipeline , shaderStage , infoType , pInfoSize , pInfo ,)"
    )
}
pub(crate) fn impl_vkBindAccelerationStructureMemoryNV(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV,
) -> VkResult {
    unimplemented!("vkBindAccelerationStructureMemoryNV (device , bindInfoCount , pBindInfos ,)")
}
pub(crate) fn impl_vkDestroyDebugUtilsMessengerEXT(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyDebugUtilsMessengerEXT (instance , messenger , pAllocator ,)")
}
pub(crate) fn impl_vkGetDeviceMemoryOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo,
) -> u64 {
    unimplemented!("vkGetDeviceMemoryOpaqueCaptureAddress (device , pInfo ,)")
}
pub(crate) fn impl_vkGetInstanceProcAddr(
    instance: VkInstance,
    pName: *const std::ffi::c_char,
) -> PFN_vkVoidFunction {
    unimplemented!("vkGetInstanceProcAddr (instance , pName ,)")
}
pub(crate) fn impl_vkUninitializePerformanceApiINTEL(device: VkDevice) {
    unimplemented!("vkUninitializePerformanceApiINTEL (device ,)")
}
pub(crate) fn impl_vkCreateSemaphoreSciSyncPoolNV(
    device: VkDevice,
    pCreateInfo: *const VkSemaphoreSciSyncPoolCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pSemaphorePool: *mut VkSemaphoreSciSyncPoolNV,
) -> VkResult {
    unimplemented!(
        "vkCreateSemaphoreSciSyncPoolNV (device , pCreateInfo , pAllocator , pSemaphorePool ,)"
    )
}
pub(crate) fn impl_vkDestroyCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyCommandPool (device , commandPool , pAllocator ,)")
}
pub(crate) fn impl_vkCmdTraceRaysIndirectKHR(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    indirectDeviceAddress: VkDeviceAddress,
) {
    unimplemented!("vkCmdTraceRaysIndirectKHR (commandBuffer , pRaygenShaderBindingTable , pMissShaderBindingTable , pHitShaderBindingTable , pCallableShaderBindingTable , indirectDeviceAddress ,)")
}
pub(crate) fn impl_vkCmdDrawMeshTasksIndirectCountEXT(
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
pub(crate) fn impl_vkDestroyPipelineCache(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyPipelineCache (device , pipelineCache , pAllocator ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceVideoCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoProfile: *const VkVideoProfileInfoKHR,
    pCapabilities: *mut VkVideoCapabilitiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceVideoCapabilitiesKHR (physicalDevice , pVideoProfile , pCapabilities ,)")
}
pub(crate) fn impl_vkCmdSetViewportWScalingNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportWScalings: *const VkViewportWScalingNV,
) {
    unimplemented!("vkCmdSetViewportWScalingNV (commandBuffer , firstViewport , viewportCount , pViewportWScalings ,)")
}
pub(crate) fn impl_vkGetDescriptorSetHostMappingVALVE(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    ppData: *mut std :: ffi :: c_void,
) {
    unimplemented!("vkGetDescriptorSetHostMappingVALVE (device , descriptorSet , ppData ,)")
}
pub(crate) fn impl_vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32) {
    unimplemented!("vkCmdSetDeviceMask (commandBuffer , deviceMask ,)")
}
pub(crate) fn impl_vkCmdDrawClusterHUAWEI(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!(
        "vkCmdDrawClusterHUAWEI (commandBuffer , groupCountX , groupCountY , groupCountZ ,)"
    )
}
pub(crate) fn impl_vkCmdSetColorWriteEnableEXT(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pColorWriteEnables: *const VkBool32,
) {
    unimplemented!(
        "vkCmdSetColorWriteEnableEXT (commandBuffer , attachmentCount , pColorWriteEnables ,)"
    )
}
pub(crate) fn impl_vkCmdSetLineStippleEnableEXT(
    commandBuffer: VkCommandBuffer,
    stippledLineEnable: VkBool32,
) {
    unimplemented!("vkCmdSetLineStippleEnableEXT (commandBuffer , stippledLineEnable ,)")
}
pub(crate) fn impl_vkCmdSetRayTracingPipelineStackSizeKHR(
    commandBuffer: VkCommandBuffer,
    pipelineStackSize: u32,
) {
    unimplemented!("vkCmdSetRayTracingPipelineStackSizeKHR (commandBuffer , pipelineStackSize ,)")
}
pub(crate) fn impl_vkCmdBeginQueryIndexedEXT(
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
pub(crate) fn impl_vkCreatePrivateDataSlot(
    device: VkDevice,
    pCreateInfo: *const VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPrivateDataSlot: *mut VkPrivateDataSlot,
) -> VkResult {
    unimplemented!(
        "vkCreatePrivateDataSlot (device , pCreateInfo , pAllocator , pPrivateDataSlot ,)"
    )
}
pub(crate) fn impl_vkQueueSubmit2(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo2,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueSubmit2 (queue , submitCount , pSubmits , fence ,)")
}
pub(crate) fn impl_vkCreateFramebuffer(
    device: VkDevice,
    pCreateInfo: *const VkFramebufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFramebuffer: *mut VkFramebuffer,
) -> VkResult {
    unimplemented!("vkCreateFramebuffer (device , pCreateInfo , pAllocator , pFramebuffer ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceDisplayPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPropertiesKHR,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayPropertiesKHR (physicalDevice , pPropertyCount , pProperties ,)"
    )
}
pub(crate) fn impl_vkDestroyAccelerationStructureNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyAccelerationStructureNV (device , accelerationStructure , pAllocator ,)"
    )
}
pub(crate) fn impl_vkCmdSetDepthWriteEnable(
    commandBuffer: VkCommandBuffer,
    depthWriteEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthWriteEnable (commandBuffer , depthWriteEnable ,)")
}
pub(crate) fn impl_vkMapMemory2KHR(
    device: VkDevice,
    pMemoryMapInfo: *const VkMemoryMapInfoKHR,
    ppData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkMapMemory2KHR (device , pMemoryMapInfo , ppData ,)")
}
pub(crate) fn impl_vkCreatePipelineCache(
    device: VkDevice,
    pCreateInfo: *const VkPipelineCacheCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineCache: *mut VkPipelineCache,
) -> VkResult {
    unimplemented!("vkCreatePipelineCache (device , pCreateInfo , pAllocator , pPipelineCache ,)")
}
pub(crate) fn impl_vkGetCommandPoolMemoryConsumption(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBuffer: VkCommandBuffer,
    pConsumption: *mut VkCommandPoolMemoryConsumption,
) {
    unimplemented!(
        "vkGetCommandPoolMemoryConsumption (device , commandPool , commandBuffer , pConsumption ,)"
    )
}
pub(crate) fn impl_vkReleasePerformanceConfigurationINTEL(
    device: VkDevice,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!("vkReleasePerformanceConfigurationINTEL (device , configuration ,)")
}
pub(crate) fn impl_vkCmdSetPolygonModeEXT(
    commandBuffer: VkCommandBuffer,
    polygonMode: VkPolygonMode,
) {
    unimplemented!("vkCmdSetPolygonModeEXT (commandBuffer , polygonMode ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceExternalBufferProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
    pExternalBufferProperties: *mut VkExternalBufferProperties,
) {
    unimplemented!("vkGetPhysicalDeviceExternalBufferProperties (physicalDevice , pExternalBufferInfo , pExternalBufferProperties ,)")
}
pub(crate) fn impl_vkReleaseProfilingLockKHR(device: VkDevice) {
    unimplemented!("vkReleaseProfilingLockKHR (device ,)")
}
pub(crate) fn impl_vkCmdWriteMicromapsPropertiesEXT(
    commandBuffer: VkCommandBuffer,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!("vkCmdWriteMicromapsPropertiesEXT (commandBuffer , micromapCount , pMicromaps , queryType , queryPool , firstQuery ,)")
}
pub(crate) fn impl_vkCmdDrawMultiEXT(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pVertexInfo: *const VkMultiDrawInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawMultiEXT (commandBuffer , drawCount , pVertexInfo , instanceCount , firstInstance , stride ,)")
}
pub(crate) fn impl_vkGetAccelerationStructureBuildSizesKHR(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pMaxPrimitiveCounts: *const u32,
    pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
) {
    unimplemented!("vkGetAccelerationStructureBuildSizesKHR (device , buildType , pBuildInfo , pMaxPrimitiveCounts , pSizeInfo ,)")
}
pub(crate) fn impl_vkGetFaultData(
    device: VkDevice,
    faultQueryBehavior: VkFaultQueryBehavior,
    pUnrecordedFaults: *mut VkBool32,
    pFaultCount: *mut u32,
    pFaults: *mut VkFaultData,
) -> VkResult {
    unimplemented!("vkGetFaultData (device , faultQueryBehavior , pUnrecordedFaults , pFaultCount , pFaults ,)")
}
pub(crate) fn impl_vkCmdCopyMicromapToMemoryEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
) {
    unimplemented!("vkCmdCopyMicromapToMemoryEXT (commandBuffer , pInfo ,)")
}
pub(crate) fn impl_vkCmdDebugMarkerInsertEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
) {
    unimplemented!("vkCmdDebugMarkerInsertEXT (commandBuffer , pMarkerInfo ,)")
}
pub(crate) fn impl_vkGetPipelineExecutableStatisticsKHR(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pStatisticCount: *mut u32,
    pStatistics: *mut VkPipelineExecutableStatisticKHR,
) -> VkResult {
    unimplemented!("vkGetPipelineExecutableStatisticsKHR (device , pExecutableInfo , pStatisticCount , pStatistics ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceMultisamplePropertiesEXT(
    physicalDevice: VkPhysicalDevice,
    samples: VkSampleCountFlagBits,
    pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
) {
    unimplemented!("vkGetPhysicalDeviceMultisamplePropertiesEXT (physicalDevice , samples , pMultisampleProperties ,)")
}
pub(crate) fn impl_vkBindVideoSessionMemoryKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    bindSessionMemoryInfoCount: u32,
    pBindSessionMemoryInfos: *const VkBindVideoSessionMemoryInfoKHR,
) -> VkResult {
    unimplemented!("vkBindVideoSessionMemoryKHR (device , videoSession , bindSessionMemoryInfoCount , pBindSessionMemoryInfos ,)")
}
pub(crate) fn impl_vkBindImageMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindImageMemoryInfo,
) -> VkResult {
    unimplemented!("vkBindImageMemory2 (device , bindInfoCount , pBindInfos ,)")
}
pub(crate) fn impl_vkResetQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    unimplemented!("vkResetQueryPool (device , queryPool , firstQuery , queryCount ,)")
}
pub(crate) fn impl_vkMergePipelineCaches(
    device: VkDevice,
    dstCache: VkPipelineCache,
    srcCacheCount: u32,
    pSrcCaches: *const VkPipelineCache,
) -> VkResult {
    unimplemented!("vkMergePipelineCaches (device , dstCache , srcCacheCount , pSrcCaches ,)")
}
pub(crate) fn impl_vkCmdSetDiscardRectangleEXT(
    commandBuffer: VkCommandBuffer,
    firstDiscardRectangle: u32,
    discardRectangleCount: u32,
    pDiscardRectangles: *const VkRect2D,
) {
    unimplemented!("vkCmdSetDiscardRectangleEXT (commandBuffer , firstDiscardRectangle , discardRectangleCount , pDiscardRectangles ,)")
}
pub(crate) fn impl_vkCmdCopyBufferToImage2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2,
) {
    unimplemented!("vkCmdCopyBufferToImage2 (commandBuffer , pCopyBufferToImageInfo ,)")
}
pub(crate) fn impl_vkCmdSetScissor(
    commandBuffer: VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: *const VkRect2D,
) {
    unimplemented!("vkCmdSetScissor (commandBuffer , firstScissor , scissorCount , pScissors ,)")
}
pub(crate) fn impl_vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32) {
    unimplemented!("vkCmdSetLineWidth (commandBuffer , lineWidth ,)")
}
pub(crate) fn impl_vkCmdDebugMarkerBeginEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
) {
    unimplemented!("vkCmdDebugMarkerBeginEXT (commandBuffer , pMarkerInfo ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoFormatInfo: *const VkPhysicalDeviceVideoFormatInfoKHR,
    pVideoFormatPropertyCount: *mut u32,
    pVideoFormatProperties: *mut VkVideoFormatPropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceVideoFormatPropertiesKHR (physicalDevice , pVideoFormatInfo , pVideoFormatPropertyCount , pVideoFormatProperties ,)")
}
pub(crate) fn impl_vkGetRandROutputDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    rrOutput: RROutput,
    pDisplay: *mut VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkGetRandROutputDisplayEXT (physicalDevice , dpy , rrOutput , pDisplay ,)")
}
pub(crate) fn impl_vkGetSemaphoreFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkSemaphoreGetFdInfoKHR,
    pFd: *mut int,
) -> VkResult {
    unimplemented!("vkGetSemaphoreFdKHR (device , pGetFdInfo , pFd ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceCapabilities2KHR (physicalDevice , pSurfaceInfo , pSurfaceCapabilities ,)")
}
pub(crate) fn impl_vkCmdCopyAccelerationStructureNV(
    commandBuffer: VkCommandBuffer,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    mode: VkCopyAccelerationStructureModeKHR,
) {
    unimplemented!("vkCmdCopyAccelerationStructureNV (commandBuffer , dst , src , mode ,)")
}
pub(crate) fn impl_vkAcquireFullScreenExclusiveModeEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkAcquireFullScreenExclusiveModeEXT (device , swapchain ,)")
}
pub(crate) fn impl_vkCreateEvent(
    device: VkDevice,
    pCreateInfo: *const VkEventCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pEvent: *mut VkEvent,
) -> VkResult {
    unimplemented!("vkCreateEvent (device , pCreateInfo , pAllocator , pEvent ,)")
}
pub(crate) fn impl_vkDestroyPipelineLayout(
    device: VkDevice,
    pipelineLayout: VkPipelineLayout,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyPipelineLayout (device , pipelineLayout , pAllocator ,)")
}
pub(crate) fn impl_vkEnumerateDeviceLayerProperties(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult {
    unimplemented!(
        "vkEnumerateDeviceLayerProperties (physicalDevice , pPropertyCount , pProperties ,)"
    )
}
pub(crate) fn impl_vkCreateWaylandSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateWaylandSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkCmdSetProvokingVertexModeEXT(
    commandBuffer: VkCommandBuffer,
    provokingVertexMode: VkProvokingVertexModeEXT,
) {
    unimplemented!("vkCmdSetProvokingVertexModeEXT (commandBuffer , provokingVertexMode ,)")
}
pub(crate) fn impl_vkCmdSetDepthClampEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthClampEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClampEnableEXT (commandBuffer , depthClampEnable ,)")
}
pub(crate) fn impl_vkWriteMicromapsPropertiesEXT(
    device: VkDevice,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    dataSize: isize,
    pData: *mut std :: ffi :: c_void,
    stride: isize,
) -> VkResult {
    unimplemented!("vkWriteMicromapsPropertiesEXT (device , micromapCount , pMicromaps , queryType , dataSize , pData , stride ,)")
}
pub(crate) fn impl_vkAllocateMemory(
    device: VkDevice,
    pAllocateInfo: *const VkMemoryAllocateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pMemory: *mut VkDeviceMemory,
) -> VkResult {
    unimplemented!("vkAllocateMemory (device , pAllocateInfo , pAllocator , pMemory ,)")
}
pub(crate) fn impl_vkDestroyBufferCollectionFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyBufferCollectionFUCHSIA (device , collection , pAllocator ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSurfacePresentModesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfacePresentModesKHR (physicalDevice , surface , pPresentModeCount , pPresentModes ,)")
}
pub(crate) fn impl_vkCopyMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapInfoEXT,
) -> VkResult {
    unimplemented!("vkCopyMicromapEXT (device , deferredOperation , pInfo ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceXlibPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    dpy: *mut Display,
    visualID: VisualID,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceXlibPresentationSupportKHR (physicalDevice , queueFamilyIndex , dpy , visualID ,)")
}
pub(crate) fn impl_vkCmdSetCoarseSampleOrderNV(
    commandBuffer: VkCommandBuffer,
    sampleOrderType: VkCoarseSampleOrderTypeNV,
    customSampleOrderCount: u32,
    pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
) {
    unimplemented!("vkCmdSetCoarseSampleOrderNV (commandBuffer , sampleOrderType , customSampleOrderCount , pCustomSampleOrders ,)")
}
pub(crate) fn impl_vkCreateBufferView(
    device: VkDevice,
    pCreateInfo: *const VkBufferViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkBufferView,
) -> VkResult {
    unimplemented!("vkCreateBufferView (device , pCreateInfo , pAllocator , pView ,)")
}
pub(crate) fn impl_vkDestroyShaderModule(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyShaderModule (device , shaderModule , pAllocator ,)")
}
pub(crate) fn impl_vkCreateDebugUtilsMessengerEXT(
    instance: VkInstance,
    pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMessenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult {
    unimplemented!(
        "vkCreateDebugUtilsMessengerEXT (instance , pCreateInfo , pAllocator , pMessenger ,)"
    )
}
pub(crate) fn impl_vkReleaseFullScreenExclusiveModeEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkReleaseFullScreenExclusiveModeEXT (device , swapchain ,)")
}
pub(crate) fn impl_vkCmdPushDescriptorSetWithTemplateKHR(
    commandBuffer: VkCommandBuffer,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const std::ffi::c_void,
) {
    unimplemented!("vkCmdPushDescriptorSetWithTemplateKHR (commandBuffer , descriptorUpdateTemplate , layout , set , pData ,)")
}
pub(crate) fn impl_vkCmdCopyBufferToImage(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
) {
    unimplemented!("vkCmdCopyBufferToImage (commandBuffer , srcBuffer , dstImage , dstImageLayout , regionCount , pRegions ,)")
}
pub(crate) fn impl_vkCmdDrawIndexed(
    commandBuffer: VkCommandBuffer,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
) {
    unimplemented!("vkCmdDrawIndexed (commandBuffer , indexCount , instanceCount , firstIndex , vertexOffset , firstInstance ,)")
}
pub(crate) fn impl_vkCmdSetRasterizationStreamEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationStream: u32,
) {
    unimplemented!("vkCmdSetRasterizationStreamEXT (commandBuffer , rasterizationStream ,)")
}
pub(crate) fn impl_vkGetRayTracingShaderGroupStackSizeKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    group: u32,
    groupShader: VkShaderGroupShaderKHR,
) -> VkDeviceSize {
    unimplemented!(
        "vkGetRayTracingShaderGroupStackSizeKHR (device , pipeline , group , groupShader ,)"
    )
}
pub(crate) fn impl_vkGetImageSubresourceLayout2EXT(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource2EXT,
    pLayout: *mut VkSubresourceLayout2EXT,
) {
    unimplemented!("vkGetImageSubresourceLayout2EXT (device , image , pSubresource , pLayout ,)")
}
pub(crate) fn impl_vkCmdDispatchBase(
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
pub(crate) fn impl_vkGetAndroidHardwareBufferPropertiesANDROID(
    device: VkDevice,
    buffer: *const AHardwareBuffer,
    pProperties: *mut VkAndroidHardwareBufferPropertiesANDROID,
) -> VkResult {
    unimplemented!("vkGetAndroidHardwareBufferPropertiesANDROID (device , buffer , pProperties ,)")
}
pub(crate) fn impl_vkCreateDescriptorSetLayout(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSetLayout: *mut VkDescriptorSetLayout,
) -> VkResult {
    unimplemented!("vkCreateDescriptorSetLayout (device , pCreateInfo , pAllocator , pSetLayout ,)")
}
pub(crate) fn impl_vkCreateScreenSurfaceQNX(
    instance: VkInstance,
    pCreateInfo: *const VkScreenSurfaceCreateInfoQNX,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateScreenSurfaceQNX (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkCmdSetAttachmentFeedbackLoopEnableEXT(
    commandBuffer: VkCommandBuffer,
    aspectMask: VkImageAspectFlags,
) {
    unimplemented!("vkCmdSetAttachmentFeedbackLoopEnableEXT (commandBuffer , aspectMask ,)")
}
pub(crate) fn impl_vkWaitSemaphores(
    device: VkDevice,
    pWaitInfo: *const VkSemaphoreWaitInfo,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitSemaphores (device , pWaitInfo , timeout ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceImageFormatProperties(
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
pub(crate) fn impl_vkCmdBuildAccelerationStructureNV(
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
pub(crate) fn impl_vkGetDeviceAccelerationStructureCompatibilityKHR(
    device: VkDevice,
    pVersionInfo: *const VkAccelerationStructureVersionInfoKHR,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
) {
    unimplemented!("vkGetDeviceAccelerationStructureCompatibilityKHR (device , pVersionInfo , pCompatibility ,)")
}
pub(crate) fn impl_vkCmdSetRasterizationSamplesEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationSamples: VkSampleCountFlagBits,
) {
    unimplemented!("vkCmdSetRasterizationSamplesEXT (commandBuffer , rasterizationSamples ,)")
}
pub(crate) fn impl_vkAllocateDescriptorSets(
    device: VkDevice,
    pAllocateInfo: *const VkDescriptorSetAllocateInfo,
    pDescriptorSets: *mut VkDescriptorSet,
) -> VkResult {
    unimplemented!("vkAllocateDescriptorSets (device , pAllocateInfo , pDescriptorSets ,)")
}
pub(crate) fn impl_vkCmdSetViewportShadingRatePaletteNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pShadingRatePalettes: *const VkShadingRatePaletteNV,
) {
    unimplemented!("vkCmdSetViewportShadingRatePaletteNV (commandBuffer , firstViewport , viewportCount , pShadingRatePalettes ,)")
}
pub(crate) fn impl_vkGetAccelerationStructureHandleNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    dataSize: isize,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!(
        "vkGetAccelerationStructureHandleNV (device , accelerationStructure , dataSize , pData ,)"
    )
}
pub(crate) fn impl_vkCmdSetScissorWithCount(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const VkRect2D,
) {
    unimplemented!("vkCmdSetScissorWithCount (commandBuffer , scissorCount , pScissors ,)")
}
pub(crate) fn impl_vkCreateDevice(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: *const VkDeviceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDevice: *mut VkDevice,
) -> VkResult {
    unimplemented!("vkCreateDevice (physicalDevice , pCreateInfo , pAllocator , pDevice ,)")
}
pub(crate) fn impl_vkCmdBlitImage(
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
pub(crate) fn impl_vkGetEncodedVideoSessionParametersKHR(
    device: VkDevice,
    pVideoSessionParametersInfo: *const VkVideoEncodeSessionParametersGetInfoKHR,
    pFeedbackInfo: *mut VkVideoEncodeSessionParametersFeedbackInfoKHR,
    pDataSize: *mut isize,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetEncodedVideoSessionParametersKHR (device , pVideoSessionParametersInfo , pFeedbackInfo , pDataSize , pData ,)")
}
pub(crate) fn impl_vkDestroyVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!(
        "vkDestroyVideoSessionParametersKHR (device , videoSessionParameters , pAllocator ,)"
    )
}
pub(crate) fn impl_vkGetImageOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkImageCaptureDescriptorDataInfoEXT,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetImageOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)")
}
pub(crate) fn impl_vkSetBufferCollectionBufferConstraintsFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pBufferConstraintsInfo: *const VkBufferConstraintsInfoFUCHSIA,
) -> VkResult {
    unimplemented!("vkSetBufferCollectionBufferConstraintsFUCHSIA (device , collection , pBufferConstraintsInfo ,)")
}
pub(crate) fn impl_vkCmdSetColorWriteMaskEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorWriteMasks: *const VkColorComponentFlags,
) {
    unimplemented!("vkCmdSetColorWriteMaskEXT (commandBuffer , firstAttachment , attachmentCount , pColorWriteMasks ,)")
}
pub(crate) fn impl_vkGetPipelineCacheData(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pDataSize: *mut isize,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetPipelineCacheData (device , pipelineCache , pDataSize , pData ,)")
}
pub(crate) fn impl_vkGetSemaphoreSciSyncObjNV(
    device: VkDevice,
    pGetSciSyncInfo: *const VkSemaphoreGetSciSyncInfoNV,
    pHandle: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetSemaphoreSciSyncObjNV (device , pGetSciSyncInfo , pHandle ,)")
}
pub(crate) fn impl_vkCmdDrawIndirectByteCountEXT(
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
pub(crate) fn impl_vkCmdWriteAccelerationStructuresPropertiesKHR(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!("vkCmdWriteAccelerationStructuresPropertiesKHR (commandBuffer , accelerationStructureCount , pAccelerationStructures , queryType , queryPool , firstQuery ,)")
}
pub(crate) fn impl_vkCmdDraw(
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
pub(crate) fn impl_vkCmdSetColorBlendEquationEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEquations: *const VkColorBlendEquationEXT,
) {
    unimplemented!("vkCmdSetColorBlendEquationEXT (commandBuffer , firstAttachment , attachmentCount , pColorBlendEquations ,)")
}
pub(crate) fn impl_vkCmdSetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    unimplemented!("vkCmdSetEvent (commandBuffer , event , stageMask ,)")
}
pub(crate) fn impl_vkGetDeviceImageMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
) {
    unimplemented!("vkGetDeviceImageMemoryRequirements (device , pInfo , pMemoryRequirements ,)")
}
pub(crate) fn impl_vkCreateValidationCacheEXT(
    device: VkDevice,
    pCreateInfo: *const VkValidationCacheCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pValidationCache: *mut VkValidationCacheEXT,
) -> VkResult {
    unimplemented!(
        "vkCreateValidationCacheEXT (device , pCreateInfo , pAllocator , pValidationCache ,)"
    )
}
pub(crate) fn impl_vkCmdTraceRaysIndirect2KHR(
    commandBuffer: VkCommandBuffer,
    indirectDeviceAddress: VkDeviceAddress,
) {
    unimplemented!("vkCmdTraceRaysIndirect2KHR (commandBuffer , indirectDeviceAddress ,)")
}
pub(crate) fn impl_vkCmdSetPrimitiveRestartEnable(
    commandBuffer: VkCommandBuffer,
    primitiveRestartEnable: VkBool32,
) {
    unimplemented!("vkCmdSetPrimitiveRestartEnable (commandBuffer , primitiveRestartEnable ,)")
}
pub(crate) fn impl_vkCmdSetBlendConstants(
    commandBuffer: VkCommandBuffer,
    blendConstants: [f32; 4 as usize],
) {
    unimplemented!("vkCmdSetBlendConstants (commandBuffer , blendConstants ,)")
}
pub(crate) fn impl_vkCmdSetAlphaToOneEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToOneEnable: VkBool32,
) {
    unimplemented!("vkCmdSetAlphaToOneEnableEXT (commandBuffer , alphaToOneEnable ,)")
}
pub(crate) fn impl_vkGetSemaphoreCounterValue(
    device: VkDevice,
    semaphore: VkSemaphore,
    pValue: *mut u64,
) -> VkResult {
    unimplemented!("vkGetSemaphoreCounterValue (device , semaphore , pValue ,)")
}
pub(crate) fn impl_vkGetDisplayPlaneCapabilities2KHR(
    physicalDevice: VkPhysicalDevice,
    pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR,
    pCapabilities: *mut VkDisplayPlaneCapabilities2KHR,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayPlaneCapabilities2KHR (physicalDevice , pDisplayPlaneInfo , pCapabilities ,)"
    )
}
pub(crate) fn impl_vkGetPhysicalDeviceFeatures(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures,
) {
    unimplemented!("vkGetPhysicalDeviceFeatures (physicalDevice , pFeatures ,)")
}
pub(crate) fn impl_vkUpdateDescriptorSets(
    device: VkDevice,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
    descriptorCopyCount: u32,
    pDescriptorCopies: *const VkCopyDescriptorSet,
) {
    unimplemented!("vkUpdateDescriptorSets (device , descriptorWriteCount , pDescriptorWrites , descriptorCopyCount , pDescriptorCopies ,)")
}
pub(crate) fn impl_vkGetDisplayPlaneSupportedDisplaysKHR(
    physicalDevice: VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: *mut u32,
    pDisplays: *mut VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkGetDisplayPlaneSupportedDisplaysKHR (physicalDevice , planeIndex , pDisplayCount , pDisplays ,)")
}
pub(crate) fn impl_vkGetPipelineExecutablePropertiesKHR(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoKHR,
    pExecutableCount: *mut u32,
    pProperties: *mut VkPipelineExecutablePropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetPipelineExecutablePropertiesKHR (device , pPipelineInfo , pExecutableCount , pProperties ,)")
}
pub(crate) fn impl_vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult {
    unimplemented!("vkGetFenceStatus (device , fence ,)")
}
pub(crate) fn impl_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureCaptureDescriptorDataInfoEXT,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!(
        "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT (device , pInfo , pData ,)"
    )
}
pub(crate) fn impl_vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    display: *mut wl_display,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceWaylandPresentationSupportKHR (physicalDevice , queueFamilyIndex , display ,)")
}
pub(crate) fn impl_vkCreateMicromapEXT(
    device: VkDevice,
    pCreateInfo: *const VkMicromapCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMicromap: *mut VkMicromapEXT,
) -> VkResult {
    unimplemented!("vkCreateMicromapEXT (device , pCreateInfo , pAllocator , pMicromap ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceMemoryProperties(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
) {
    unimplemented!("vkGetPhysicalDeviceMemoryProperties (physicalDevice , pMemoryProperties ,)")
}
pub(crate) fn impl_vkDestroySampler(
    device: VkDevice,
    sampler: VkSampler,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySampler (device , sampler , pAllocator ,)")
}
pub(crate) fn impl_vkCmdSetRasterizerDiscardEnable(
    commandBuffer: VkCommandBuffer,
    rasterizerDiscardEnable: VkBool32,
) {
    unimplemented!("vkCmdSetRasterizerDiscardEnable (commandBuffer , rasterizerDiscardEnable ,)")
}
pub(crate) fn impl_vkCreateXcbSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateXcbSurfaceKHR (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkAcquireImageANDROID(
    device: VkDevice,
    image: VkImage,
    nativeFenceFd: int,
    semaphore: VkSemaphore,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkAcquireImageANDROID (device , image , nativeFenceFd , semaphore , fence ,)")
}
pub(crate) fn impl_vkCmdCopyMemoryToImageIndirectNV(
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
pub(crate) fn impl_vkSetDebugUtilsObjectTagEXT(
    device: VkDevice,
    pTagInfo: *const VkDebugUtilsObjectTagInfoEXT,
) -> VkResult {
    unimplemented!("vkSetDebugUtilsObjectTagEXT (device , pTagInfo ,)")
}
pub(crate) fn impl_vkGetShaderModuleIdentifierEXT(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
) {
    unimplemented!("vkGetShaderModuleIdentifierEXT (device , shaderModule , pIdentifier ,)")
}
pub(crate) fn impl_vkGetMemoryFdKHR(
    device: VkDevice,
    pGetFdInfo: *const VkMemoryGetFdInfoKHR,
    pFd: *mut int,
) -> VkResult {
    unimplemented!("vkGetMemoryFdKHR (device , pGetFdInfo , pFd ,)")
}
pub(crate) fn impl_vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndDebugUtilsLabelEXT (commandBuffer ,)")
}
pub(crate) fn impl_vkCmdSetCoverageReductionModeNV(
    commandBuffer: VkCommandBuffer,
    coverageReductionMode: VkCoverageReductionModeNV,
) {
    unimplemented!("vkCmdSetCoverageReductionModeNV (commandBuffer , coverageReductionMode ,)")
}
pub(crate) fn impl_vkCmdEndRendering(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndRendering (commandBuffer ,)")
}
pub(crate) fn impl_vkDestroyFramebuffer(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyFramebuffer (device , framebuffer , pAllocator ,)")
}
pub(crate) fn impl_vkCmdBuildAccelerationStructuresIndirectKHR(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pIndirectDeviceAddresses: *const VkDeviceAddress,
    pIndirectStrides: *const u32,
    ppMaxPrimitiveCounts: *const u32,
) {
    unimplemented!("vkCmdBuildAccelerationStructuresIndirectKHR (commandBuffer , infoCount , pInfos , pIndirectDeviceAddresses , pIndirectStrides , ppMaxPrimitiveCounts ,)")
}
pub(crate) fn impl_vkMapMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    ppData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkMapMemory (device , memory , offset , size , flags , ppData ,)")
}
pub(crate) fn impl_vkDestroySemaphoreSciSyncPoolNV(
    device: VkDevice,
    semaphorePool: VkSemaphoreSciSyncPoolNV,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroySemaphoreSciSyncPoolNV (device , semaphorePool , pAllocator ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties,
) {
    unimplemented!(
        "vkGetPhysicalDeviceFormatProperties (physicalDevice , format , pFormatProperties ,)"
    )
}
pub(crate) fn impl_vkCmdCopyImageToBuffer(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
) {
    unimplemented!("vkCmdCopyImageToBuffer (commandBuffer , srcImage , srcImageLayout , dstBuffer , regionCount , pRegions ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceXcbPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceXcbPresentationSupportKHR (physicalDevice , queueFamilyIndex , connection , visual_id ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceFeatures2(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures2,
) {
    unimplemented!("vkGetPhysicalDeviceFeatures2 (physicalDevice , pFeatures ,)")
}
pub(crate) fn impl_vkCreateRayTracingPipelinesKHR(
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
pub(crate) fn impl_vkCmdWriteBufferMarker2AMD(
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
pub(crate) fn impl_vkCreateSemaphore(
    device: VkDevice,
    pCreateInfo: *const VkSemaphoreCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSemaphore: *mut VkSemaphore,
) -> VkResult {
    unimplemented!("vkCreateSemaphore (device , pCreateInfo , pAllocator , pSemaphore ,)")
}
pub(crate) fn impl_vkGetSwapchainGrallocUsageANDROID(
    device: VkDevice,
    format: VkFormat,
    imageUsage: VkImageUsageFlags,
    grallocUsage: *mut int,
) -> VkResult {
    unimplemented!(
        "vkGetSwapchainGrallocUsageANDROID (device , format , imageUsage , grallocUsage ,)"
    )
}
pub(crate) fn impl_vkCreateBufferCollectionFUCHSIA(
    device: VkDevice,
    pCreateInfo: *const VkBufferCollectionCreateInfoFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
    pCollection: *mut VkBufferCollectionFUCHSIA,
) -> VkResult {
    unimplemented!(
        "vkCreateBufferCollectionFUCHSIA (device , pCreateInfo , pAllocator , pCollection ,)"
    )
}
pub(crate) fn impl_vkCmdDrawClusterIndirectHUAWEI(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    unimplemented!("vkCmdDrawClusterIndirectHUAWEI (commandBuffer , buffer , offset ,)")
}
pub(crate) fn impl_vkAcquirePerformanceConfigurationINTEL(
    device: VkDevice,
    pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL,
    pConfiguration: *mut VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!(
        "vkAcquirePerformanceConfigurationINTEL (device , pAcquireInfo , pConfiguration ,)"
    )
}
pub(crate) fn impl_vkDebugReportMessageEXT(
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
pub(crate) fn impl_vkGetPipelineExecutableInternalRepresentationsKHR(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pInternalRepresentationCount: *mut u32,
    pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
) -> VkResult {
    unimplemented!("vkGetPipelineExecutableInternalRepresentationsKHR (device , pExecutableInfo , pInternalRepresentationCount , pInternalRepresentations ,)")
}
pub(crate) fn impl_vkSetPrivateData(
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
pub(crate) fn impl_vkGetDescriptorSetLayoutHostMappingInfoVALVE(
    device: VkDevice,
    pBindingReference: *const VkDescriptorSetBindingReferenceVALVE,
    pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE,
) {
    unimplemented!("vkGetDescriptorSetLayoutHostMappingInfoVALVE (device , pBindingReference , pHostMapping ,)")
}
pub(crate) fn impl_vkCreateDebugReportCallbackEXT(
    instance: VkInstance,
    pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pCallback: *mut VkDebugReportCallbackEXT,
) -> VkResult {
    unimplemented!(
        "vkCreateDebugReportCallbackEXT (instance , pCreateInfo , pAllocator , pCallback ,)"
    )
}
pub(crate) fn impl_vkSetLocalDimmingAMD(
    device: VkDevice,
    swapChain: VkSwapchainKHR,
    localDimmingEnable: VkBool32,
) {
    unimplemented!("vkSetLocalDimmingAMD (device , swapChain , localDimmingEnable ,)")
}
pub(crate) fn impl_vkGetDynamicRenderingTilePropertiesQCOM(
    device: VkDevice,
    pRenderingInfo: *const VkRenderingInfo,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult {
    unimplemented!(
        "vkGetDynamicRenderingTilePropertiesQCOM (device , pRenderingInfo , pProperties ,)"
    )
}
pub(crate) fn impl_vkCmdWaitEvents2(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    pDependencyInfos: *const VkDependencyInfo,
) {
    unimplemented!("vkCmdWaitEvents2 (commandBuffer , eventCount , pEvents , pDependencyInfos ,)")
}
pub(crate) fn impl_vkGetSemaphoreZirconHandleFUCHSIA(
    device: VkDevice,
    pGetZirconHandleInfo: *const VkSemaphoreGetZirconHandleInfoFUCHSIA,
    pZirconHandle: *mut zx_handle_t,
) -> VkResult {
    unimplemented!(
        "vkGetSemaphoreZirconHandleFUCHSIA (device , pGetZirconHandleInfo , pZirconHandle ,)"
    )
}
pub(crate) fn impl_vkEnumeratePhysicalDevices(
    instance: VkInstance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut VkPhysicalDevice,
) -> VkResult {
    unimplemented!(
        "vkEnumeratePhysicalDevices (instance , pPhysicalDeviceCount , pPhysicalDevices ,)"
    )
}
pub(crate) fn impl_vkCmdBindIndexBuffer(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    indexType: VkIndexType,
) {
    unimplemented!("vkCmdBindIndexBuffer (commandBuffer , buffer , offset , indexType ,)")
}
pub(crate) fn impl_vkCmdClearColorImage(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pColor: *const VkClearColorValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
) {
    unimplemented!("vkCmdClearColorImage (commandBuffer , image , imageLayout , pColor , rangeCount , pRanges ,)")
}
pub(crate) fn impl_vkGetRenderAreaGranularity(
    device: VkDevice,
    renderPass: VkRenderPass,
    pGranularity: *mut VkExtent2D,
) {
    unimplemented!("vkGetRenderAreaGranularity (device , renderPass , pGranularity ,)")
}
pub(crate) fn impl_vkCmdSetVertexInputEXT(
    commandBuffer: VkCommandBuffer,
    vertexBindingDescriptionCount: u32,
    pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT,
    vertexAttributeDescriptionCount: u32,
    pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT,
) {
    unimplemented!("vkCmdSetVertexInputEXT (commandBuffer , vertexBindingDescriptionCount , pVertexBindingDescriptions , vertexAttributeDescriptionCount , pVertexAttributeDescriptions ,)")
}
pub(crate) fn impl_vkExportMetalObjectsEXT(
    device: VkDevice,
    pMetalObjectsInfo: *mut VkExportMetalObjectsInfoEXT,
) {
    unimplemented!("vkExportMetalObjectsEXT (device , pMetalObjectsInfo ,)")
}
pub(crate) fn impl_vkCreateCommandPool(
    device: VkDevice,
    pCreateInfo: *const VkCommandPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pCommandPool: *mut VkCommandPool,
) -> VkResult {
    unimplemented!("vkCreateCommandPool (device , pCreateInfo , pAllocator , pCommandPool ,)")
}
pub(crate) fn impl_vkGetMemoryFdPropertiesKHR(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: int,
    pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
) -> VkResult {
    unimplemented!("vkGetMemoryFdPropertiesKHR (device , handleType , fd , pMemoryFdProperties ,)")
}
pub(crate) fn impl_vkGetImageDrmFormatModifierPropertiesEXT(
    device: VkDevice,
    image: VkImage,
    pProperties: *mut VkImageDrmFormatModifierPropertiesEXT,
) -> VkResult {
    unimplemented!("vkGetImageDrmFormatModifierPropertiesEXT (device , image , pProperties ,)")
}
pub(crate) fn impl_vkGetDeviceImageSparseMemoryRequirements(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
) {
    unimplemented!("vkGetDeviceImageSparseMemoryRequirements (device , pInfo , pSparseMemoryRequirementCount , pSparseMemoryRequirements ,)")
}
pub(crate) fn impl_vkCmdOpticalFlowExecuteNV(
    commandBuffer: VkCommandBuffer,
    session: VkOpticalFlowSessionNV,
    pExecuteInfo: *const VkOpticalFlowExecuteInfoNV,
) {
    unimplemented!("vkCmdOpticalFlowExecuteNV (commandBuffer , session , pExecuteInfo ,)")
}
pub(crate) fn impl_vkAcquireNextImageKHR(
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
pub(crate) fn impl_vkCreateViSurfaceNN(
    instance: VkInstance,
    pCreateInfo: *const VkViSurfaceCreateInfoNN,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult {
    unimplemented!("vkCreateViSurfaceNN (instance , pCreateInfo , pAllocator , pSurface ,)")
}
pub(crate) fn impl_vkGetPhysicalDeviceSciBufAttributesNV(
    physicalDevice: VkPhysicalDevice,
    pAttributes: NvSciBufAttrList,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSciBufAttributesNV (physicalDevice , pAttributes ,)")
}
pub(crate) fn impl_vkGetRayTracingShaderGroupHandlesKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: isize,
    pData: *mut std :: ffi :: c_void,
) -> VkResult {
    unimplemented!("vkGetRayTracingShaderGroupHandlesKHR (device , pipeline , firstGroup , groupCount , dataSize , pData ,)")
}
pub(crate) fn impl_vkGetPastPresentationTimingGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pPresentationTimingCount: *mut u32,
    pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
) -> VkResult {
    unimplemented!("vkGetPastPresentationTimingGOOGLE (device , swapchain , pPresentationTimingCount , pPresentationTimings ,)")
}
pub(crate) fn impl_vkCmdFillBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
) {
    unimplemented!("vkCmdFillBuffer (commandBuffer , dstBuffer , dstOffset , size , data ,)")
}
pub(crate) fn impl_vkCmdWriteAccelerationStructuresPropertiesNV(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureNV,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!("vkCmdWriteAccelerationStructuresPropertiesNV (commandBuffer , accelerationStructureCount , pAccelerationStructures , queryType , queryPool , firstQuery ,)")
}
pub(crate) fn impl_vkCmdCopyMemoryToAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) {
    unimplemented!("vkCmdCopyMemoryToAccelerationStructureKHR (commandBuffer , pInfo ,)")
}
pub(crate) fn impl_vkDestroyFence(
    device: VkDevice,
    fence: VkFence,
    pAllocator: *const VkAllocationCallbacks,
) {
    unimplemented!("vkDestroyFence (device , fence , pAllocator ,)")
}
pub(crate) fn impl_vkSignalSemaphore(
    device: VkDevice,
    pSignalInfo: *const VkSemaphoreSignalInfo,
) -> VkResult {
    unimplemented!("vkSignalSemaphore (device , pSignalInfo ,)")
}
pub(crate) fn impl_vkCmdSetCoverageModulationTableEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableEnable: VkBool32,
) {
    unimplemented!(
        "vkCmdSetCoverageModulationTableEnableNV (commandBuffer , coverageModulationTableEnable ,)"
    )
}
pub(crate) fn impl_vkReleaseSwapchainImagesEXT(
    device: VkDevice,
    pReleaseInfo: *const VkReleaseSwapchainImagesInfoEXT,
) -> VkResult {
    unimplemented!("vkReleaseSwapchainImagesEXT (device , pReleaseInfo ,)")
}
pub(crate) fn impl_vkQueueSignalReleaseImageANDROID(
    queue: VkQueue,
    waitSemaphoreCount: u32,
    pWaitSemaphores: *const VkSemaphore,
    image: VkImage,
    pNativeFenceFd: *mut int,
) -> VkResult {
    unimplemented!("vkQueueSignalReleaseImageANDROID (queue , waitSemaphoreCount , pWaitSemaphores , image , pNativeFenceFd ,)")
}
