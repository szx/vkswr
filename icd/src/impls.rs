/* Runtime crate function wrappers. */
#![allow(non_snake_case)]
#![allow(unused)]

use headers::include_commands;
use headers::vk_decls::*;
use headers::vk_defs::*;
use runtime::*;

#[no_mangle]
pub unsafe extern "C" fn vkCreateInstance(
    pCreateInfo: Option<NonNull<VkInstanceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pInstance: Option<NonNull<VkInstance>>,
) -> VkResult {
    println!("Hello from vkCreateInstance()!");

    // VUID-vkCreateInstance-pCreateInfo-parameter
    let Some(pCreateInfo) = pCreateInfo else { unreachable!() };
    let create_info = pCreateInfo.as_ref();
    // TODO: Automate valid VkInstanceCreateInfo structure asserts.
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO
    );

    // VUID-vkCreateInstance-pAllocator-parameter
    if let Some(pAllocator) = pAllocator {
        let pAllocator = pAllocator.as_ptr();
        // TODO: Use callbacks for memory allocation.
    }

    // VUID-vkCreateInstance-pInstance-parameter
    let Some(mut pInstance) = pInstance else { unreachable!() };

    runtime::create_instance(create_info, pInstance)
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumeratePhysicalDevices(
    instance: VkInstance,
    pPhysicalDeviceCount: Option<NonNull<u32>>,
    pPhysicalDevices: Option<NonNull<VkPhysicalDevice>>,
) -> VkResult {
    // VUID-vkEnumeratePhysicalDevices-instance-parameter
    let Some(instance) = get_dispatchable_handle_ref::<Instance>(instance) else { unreachable!() };

    // VUID-vkEnumeratePhysicalDevices-pPhysicalDeviceCount-parameter
    let Some(pPhysicalDeviceCount) = pPhysicalDeviceCount else { unreachable!() };

    // VUID-vkEnumeratePhysicalDevices-pPhysicalDevices-parameter
    pPhysicalDevices.map_or_else(
        || {
            *pPhysicalDeviceCount.as_ptr() = runtime::PhysicalDevice::count(instance);
            VkResult::VK_SUCCESS
        },
        |pPhysicalDevices| {
            set_dispatchable_handle(pPhysicalDevices, instance.physical_device());
            VkResult::VK_SUCCESS
        },
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateDeviceExtensionProperties(
    physicalDevice: VkPhysicalDevice,
    pLayerName: Option<NonNull<std::ffi::c_char>>,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult {
    // VUID-vkEnumerateDeviceExtensionProperties-physicalDevice-parameter
    let Some(physicalDevice) = get_dispatchable_handle_ref::<PhysicalDevice>(physicalDevice) else { unreachable!() };

    // VUID-vkEnumerateDeviceExtensionProperties-pPropertyCount-parameter
    let Some(pPropertyCount) = pPropertyCount else { unreachable!() };

    // VUID-vkEnumerateDeviceExtensionProperties-pLayerName-parameter
    if pLayerName.is_none() {
        // TODO: SPEC: "only extensions provided by the Vulkan implementation or by implicitly enabled layers are returned"
        *pPropertyCount.as_ptr() = 0;
    } else {
        let Some(pLayerName) = pLayerName else { unreachable!() };
        let Ok(layerName) = std::ffi::CStr::from_ptr(pLayerName.as_ptr()).to_str() else { unreachable!() };
        todo!("the device extensions provided by {layerName} layer are returned");
    }
    VkResult::VK_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceProperties(
    physicalDevice: VkPhysicalDevice,
    pProperties: Option<NonNull<VkPhysicalDeviceProperties>>,
) {
    // VUID-vkGetPhysicalDeviceProperties-physicalDevice-parameter
    let Some(physicalDevice) = get_dispatchable_handle_ref::<PhysicalDevice>(physicalDevice) else { unreachable!() };

    // VUID-vkGetPhysicalDeviceProperties-pProperties-parameter
    let Some(pProperties) = pProperties else { unreachable!() };

    // SPEC: "Returns properties of a physical device"
    *pProperties.as_ptr() = physicalDevice.properties();
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceMemoryProperties(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: Option<NonNull<VkPhysicalDeviceMemoryProperties>>,
) {
    // VUID-vkGetPhysicalDeviceMemoryProperties-physicalDevice-parameter
    let Some(physicalDevice) = get_dispatchable_handle_ref::<PhysicalDevice>(physicalDevice) else { unreachable!() };

    // VUID-vkGetPhysicalDeviceMemoryProperties-pMemoryProperties-parameter
    let Some(pMemoryProperties) = pMemoryProperties else { unreachable!() };

    // SPEC: "Reports memory information for the specified physical device"
    *pMemoryProperties.as_ptr() = physicalDevice.memory_properties();
}


/* unimplemented */

#[no_mangle]
pub unsafe extern "C" fn vkCmdBuildAccelerationStructureNV(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkAccelerationStructureInfoNV>>,
    instanceData: VkBuffer,
    instanceOffset: VkDeviceSize,
    update: VkBool32,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    scratch: VkBuffer,
    scratchOffset: VkDeviceSize,
) {
    unimplemented!(
        "vkCmdBuildAccelerationStructureNV(
        commandBuffer,
        pInfo,
        instanceData,
        instanceOffset,
        update,
        dst,
        src,
        scratch,
        scratchOffset,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndTransformFeedbackEXT(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: Option<NonNull<VkBuffer>>,
    pCounterBufferOffsets: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!(
        "vkCmdEndTransformFeedbackEXT(
        commandBuffer,
        firstCounterBuffer,
        counterBufferCount,
        pCounterBuffers,
        pCounterBufferOffsets,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
    device: VkDevice,
    renderpass: VkRenderPass,
    pMaxWorkgroupSize: Option<NonNull<VkExtent2D>>,
) -> VkResult {
    unimplemented!(
        "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(device, renderpass, pMaxWorkgroupSize"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDescriptorPool(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorPoolCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDescriptorPool: Option<NonNull<VkDescriptorPool>>,
) -> VkResult {
    unimplemented!("vkCreateDescriptorPool(device, pCreateInfo, pAllocator, pDescriptorPool")
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueSignalReleaseImageANDROID(
    queue: VkQueue,
    waitSemaphoreCount: u32,
    pWaitSemaphores: Option<NonNull<VkSemaphore>>,
    image: VkImage,
    pNativeFenceFd: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!(
        "vkQueueSignalReleaseImageANDROID(
        queue,
        waitSemaphoreCount,
        pWaitSemaphores,
        image,
        pNativeFenceFd,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineExecutableInternalRepresentationsKHR(
    device: VkDevice,
    pExecutableInfo: Option<NonNull<VkPipelineExecutableInfoKHR>>,
    pInternalRepresentationCount: Option<NonNull<u32>>,
    pInternalRepresentations: Option<NonNull<VkPipelineExecutableInternalRepresentationKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPipelineExecutableInternalRepresentationsKHR(
        device,
        pExecutableInfo,
        pInternalRepresentationCount,
        pInternalRepresentations,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkBindVideoSessionMemoryKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    bindSessionMemoryInfoCount: u32,
    pBindSessionMemoryInfos: Option<NonNull<VkBindVideoSessionMemoryInfoKHR>>,
) -> VkResult {
    unimplemented!(
        "vkBindVideoSessionMemoryKHR(
        device,
        videoSession,
        bindSessionMemoryInfoCount,
        pBindSessionMemoryInfos,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResolveImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: Option<NonNull<VkImageResolve>>,
) {
    unimplemented!(
        "vkCmdResolveImage(
        commandBuffer,
        srcImage,
        srcImageLayout,
        dstImage,
        dstImageLayout,
        regionCount,
        pRegions,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetProvokingVertexModeEXT(
    commandBuffer: VkCommandBuffer,
    provokingVertexMode: VkProvokingVertexModeEXT,
) {
    unimplemented!("vkCmdSetProvokingVertexModeEXT(commandBuffer, provokingVertexMode")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreatePrivateDataSlot(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkPrivateDataSlotCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPrivateDataSlot: Option<NonNull<VkPrivateDataSlot>>,
) -> VkResult {
    unimplemented!("vkCreatePrivateDataSlot(device, pCreateInfo, pAllocator, pPrivateDataSlot")
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyMicromapInfoEXT>>,
) -> VkResult {
    unimplemented!("vkCopyMicromapEXT(device, deferredOperation, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateScreenSurfaceQNX(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkScreenSurfaceCreateInfoQNX>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateScreenSurfaceQNX(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyIndirectCommandsLayoutNV(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyIndirectCommandsLayoutNV(device, indirectCommandsLayout, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageModulationTableEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableEnable: VkBool32,
) {
    unimplemented!(
        "vkCmdSetCoverageModulationTableEnableNV(commandBuffer, coverageModulationTableEnable"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateComputePipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: Option<NonNull<VkComputePipelineCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelines: Option<NonNull<VkPipeline>>,
) -> VkResult {
    unimplemented!(
        "vkCreateComputePipelines(
        device,
        pipelineCache,
        createInfoCount,
        pCreateInfos,
        pAllocator,
        pPipelines,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdRefreshObjectsKHR(
    commandBuffer: VkCommandBuffer,
    pRefreshObjects: Option<NonNull<VkRefreshObjectListKHR>>,
) {
    unimplemented!("vkCmdRefreshObjectsKHR(commandBuffer, pRefreshObjects")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPipelineBarrier2(
    commandBuffer: VkCommandBuffer,
    pDependencyInfo: Option<NonNull<VkDependencyInfo>>,
) {
    unimplemented!("vkCmdPipelineBarrier2(commandBuffer, pDependencyInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceWin32PresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice, queueFamilyIndex"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureBuildSizesKHR(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: Option<NonNull<VkAccelerationStructureBuildGeometryInfoKHR>>,
    pMaxPrimitiveCounts: Option<NonNull<u32>>,
    pSizeInfo: Option<NonNull<VkAccelerationStructureBuildSizesInfoKHR>>,
) {
    unimplemented!(
        "vkGetAccelerationStructureBuildSizesKHR(
        device,
        buildType,
        pBuildInfo,
        pMaxPrimitiveCounts,
        pSizeInfo,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateFence(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkFenceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFence: Option<NonNull<VkFence>>,
) -> VkResult {
    unimplemented!("vkCreateFence(device, pCreateInfo, pAllocator, pFence")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthClampEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthClampEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClampEnableEXT(commandBuffer, depthClampEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferDeviceAddress(
    device: VkDevice,
    pInfo: Option<NonNull<VkBufferDeviceAddressInfo>>,
) -> VkDeviceAddress {
    unimplemented!("vkGetBufferDeviceAddress(device, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkBindAccelerationStructureMemoryNV(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: Option<NonNull<VkBindAccelerationStructureMemoryInfoNV>>,
) -> VkResult {
    unimplemented!("vkBindAccelerationStructureMemoryNV(device, bindInfoCount, pBindInfos")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateShaderModule(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkShaderModuleCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pShaderModule: Option<NonNull<VkShaderModule>>,
) -> VkResult {
    unimplemented!("vkCreateShaderModule(device, pCreateInfo, pAllocator, pShaderModule")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferInfo: Option<NonNull<VkCopyBufferInfo2>>,
) {
    unimplemented!("vkCmdCopyBuffer2(commandBuffer, pCopyBufferInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: Option<NonNull<VkBufferDeviceAddressInfo>>,
) -> u64 {
    unimplemented!("vkGetBufferOpaqueCaptureAddress(device, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeferredOperationMaxConcurrencyKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> u32 {
    unimplemented!("vkGetDeferredOperationMaxConcurrencyKHR(device, operation")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRayTracingShaderGroupHandlesKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: isize,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!(
        "vkGetRayTracingShaderGroupHandlesKHR(
        device, pipeline, firstGroup, groupCount, dataSize, pData,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetImageOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySampler(
    device: VkDevice,
    sampler: VkSampler,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySampler(device, sampler, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceExtensionProperties(
    pLayerName: Option<NonNull<std::ffi::c_char>>,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult {
    println!("Hello from vkEnumerateInstanceExtensionProperties()!");
    assert_eq!(pLayerName, None);
    unsafe {
        println!("*pPropertyCount = {:?}", pPropertyCount);
        println!("*pProperties = {:?}", pProperties);
        if pProperties.is_none() {
            if let Some(pPropertyCount) = pPropertyCount {
                *pPropertyCount.as_ptr() = 0;
            }
        }
    }
    VkResult::VK_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn vkGetQueueCheckpointDataNV(
    queue: VkQueue,
    pCheckpointDataCount: Option<NonNull<u32>>,
    pCheckpointData: Option<NonNull<VkCheckpointDataNV>>,
) {
    unimplemented!("vkGetQueueCheckpointDataNV(queue, pCheckpointDataCount, pCheckpointData")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: NvSciBufObj,
    pMemorySciBufProperties: Option<NonNull<VkMemorySciBufPropertiesNV>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV(
        physicalDevice,
        handleType,
        handle,
        pMemorySciBufProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkResetCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolResetFlags,
) -> VkResult {
    unimplemented!("vkResetCommandPool(device, commandPool, flags")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoFormatInfo: Option<NonNull<VkPhysicalDeviceVideoFormatInfoKHR>>,
    pVideoFormatPropertyCount: Option<NonNull<u32>>,
    pVideoFormatProperties: Option<NonNull<VkVideoFormatPropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceVideoFormatPropertiesKHR(
        physicalDevice,
        pVideoFormatInfo,
        pVideoFormatPropertyCount,
        pVideoFormatProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateGraphicsPipelines(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: Option<NonNull<VkGraphicsPipelineCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelines: Option<NonNull<VkPipeline>>,
) -> VkResult {
    unimplemented!(
        "vkCreateGraphicsPipelines(
        device,
        pipelineCache,
        createInfoCount,
        pCreateInfos,
        pAllocator,
        pPipelines,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayPlanePropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice, pPropertyCount, pProperties"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetEncodedVideoSessionParametersKHR(
    device: VkDevice,
    pVideoSessionParametersInfo: Option<NonNull<VkVideoEncodeSessionParametersGetInfoKHR>>,
    pFeedbackInfo: Option<NonNull<VkVideoEncodeSessionParametersFeedbackInfoKHR>>,
    pDataSize: Option<NonNull<isize>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!(
        "vkGetEncodedVideoSessionParametersKHR(
        device,
        pVideoSessionParametersInfo,
        pFeedbackInfo,
        pDataSize,
        pData,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthWriteEnable(
    commandBuffer: VkCommandBuffer,
    depthWriteEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthWriteEnable(commandBuffer, depthWriteEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportWScalingNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportWScalings: Option<NonNull<VkViewportWScalingNV>>,
) {
    unimplemented!(
        "vkCmdSetViewportWScalingNV(
        commandBuffer,
        firstViewport,
        viewportCount,
        pViewportWScalings,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferMemoryRequirements2(
    device: VkDevice,
    pInfo: Option<NonNull<VkBufferMemoryRequirementsInfo2>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetBufferMemoryRequirements2(device, pInfo, pMemoryRequirements")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateOpticalFlowSessionNV(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkOpticalFlowSessionCreateInfoNV>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSession: Option<NonNull<VkOpticalFlowSessionNV>>,
) -> VkResult {
    unimplemented!("vkCreateOpticalFlowSessionNV(device, pCreateInfo, pAllocator, pSession")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndirectCount(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDrawIndirectCount(
        commandBuffer,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDispatchBase(
    commandBuffer: VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!(
        "vkCmdDispatchBase(
        commandBuffer,
        baseGroupX,
        baseGroupY,
        baseGroupZ,
        groupCountX,
        groupCountY,
        groupCountZ,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalSemaphoreProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalSemaphoreInfo: Option<NonNull<VkPhysicalDeviceExternalSemaphoreInfo>>,
    pExternalSemaphoreProperties: Option<NonNull<VkExternalSemaphoreProperties>>,
) {
    unimplemented!(
        "vkGetPhysicalDeviceExternalSemaphoreProperties(
        physicalDevice,
        pExternalSemaphoreInfo,
        pExternalSemaphoreProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueEndDebugUtilsLabelEXT(queue: VkQueue) {
    unimplemented!("vkQueueEndDebugUtilsLabelEXT(queue")
}

#[no_mangle]
pub unsafe extern "C" fn vkDebugReportMessageEXT(
    instance: VkInstance,
    flags: VkDebugReportFlagsEXT,
    objectType: VkDebugReportObjectTypeEXT,
    object: u64,
    location: isize,
    messageCode: i32,
    pLayerPrefix: Option<NonNull<std::ffi::c_char>>,
    pMessage: Option<NonNull<std::ffi::c_char>>,
) {
    unimplemented!(
        "vkDebugReportMessageEXT(
        instance,
        flags,
        objectType,
        object,
        location,
        messageCode,
        pLayerPrefix,
        pMessage,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawClusterHUAWEI(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDrawClusterHUAWEI(commandBuffer, groupCountX, groupCountY, groupCountZ")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRayTracingPipelineStackSizeKHR(
    commandBuffer: VkCommandBuffer,
    pipelineStackSize: u32,
) {
    unimplemented!("vkCmdSetRayTracingPipelineStackSizeKHR(commandBuffer, pipelineStackSize")
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMemoryToAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyMemoryToAccelerationStructureInfoKHR>>,
) -> VkResult {
    unimplemented!("vkCopyMemoryToAccelerationStructureKHR(device, deferredOperation, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyEvent(
    device: VkDevice,
    event: VkEvent,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyEvent(device, event, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPipelineCache(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyPipelineCache(device, pipelineCache, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDescriptorUpdateTemplate(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDescriptorUpdateTemplate(device, descriptorUpdateTemplate, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: Option<NonNull<u32>>,
    pCounters: Option<NonNull<VkPerformanceCounterKHR>>,
    pCounterDescriptions: Option<NonNull<VkPerformanceCounterDescriptionKHR>>,
) -> VkResult {
    unimplemented!(
        "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
        physicalDevice,
        queueFamilyIndex,
        pCounterCount,
        pCounters,
        pCounterDescriptions,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndRenderPass(commandBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPrivateData(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: Option<NonNull<u64>>,
) {
    unimplemented!("vkGetPrivateData(device, objectType, objectHandle, privateDataSlot, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginRenderPass2(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: Option<NonNull<VkRenderPassBeginInfo>>,
    pSubpassBeginInfo: Option<NonNull<VkSubpassBeginInfo>>,
) {
    unimplemented!("vkCmdBeginRenderPass2(commandBuffer, pRenderPassBegin, pSubpassBeginInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorBufferEmbeddedSamplersEXT(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
) {
    unimplemented!("vkCmdBindDescriptorBufferEmbeddedSamplersEXT(commandBuffer, pipelineBindPoint, layout, set")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBufferToImage(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: Option<NonNull<VkBufferImageCopy>>,
) {
    unimplemented!(
        "vkCmdCopyBufferToImage(
        commandBuffer,
        srcBuffer,
        dstImage,
        dstImageLayout,
        regionCount,
        pRegions,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: Option<NonNull<VkSurfaceCapabilities2EXT>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSurfaceCapabilities2EXT(physicalDevice, surface, pSurfaceCapabilities"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdClearColorImage(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pColor: Option<NonNull<VkClearColorValue>>,
    rangeCount: u32,
    pRanges: Option<NonNull<VkImageSubresourceRange>>,
) {
    unimplemented!(
        "vkCmdClearColorImage(
        commandBuffer,
        image,
        imageLayout,
        pColor,
        rangeCount,
        pRanges,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksEXT(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksEXT(commandBuffer, groupCountX, groupCountY, groupCountZ")
}

#[no_mangle]
pub unsafe extern "C" fn vkSignalSemaphore(
    device: VkDevice,
    pSignalInfo: Option<NonNull<VkSemaphoreSignalInfo>>,
) -> VkResult {
    unimplemented!("vkSignalSemaphore(device, pSignalInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndexedIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawIndexedIndirect(commandBuffer, buffer, offset, drawCount, stride")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRasterizerDiscardEnable(
    commandBuffer: VkCommandBuffer,
    rasterizerDiscardEnable: VkBool32,
) {
    unimplemented!("vkCmdSetRasterizerDiscardEnable(commandBuffer, rasterizerDiscardEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLogicOpEXT(commandBuffer: VkCommandBuffer, logicOp: VkLogicOp) {
    unimplemented!("vkCmdSetLogicOpEXT(commandBuffer, logicOp")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    index: u32,
) {
    unimplemented!("vkCmdEndQueryIndexedEXT(commandBuffer, queryPool, query, index")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    lineRasterizationMode: VkLineRasterizationModeEXT,
) {
    unimplemented!("vkCmdSetLineRasterizationModeEXT(commandBuffer, lineRasterizationMode")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImage2(
    commandBuffer: VkCommandBuffer,
    pCopyImageInfo: Option<NonNull<VkCopyImageInfo2>>,
) {
    unimplemented!("vkCmdCopyImage2(commandBuffer, pCopyImageInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyOpticalFlowSessionNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyOpticalFlowSessionNV(device, session, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageModulationTableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableCount: u32,
    pCoverageModulationTable: Option<NonNull<f32>>,
) {
    unimplemented!(
        "vkCmdSetCoverageModulationTableNV(
        commandBuffer,
        coverageModulationTableCount,
        pCoverageModulationTable,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDiscardRectangleEnableEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDiscardRectangleEnableEXT(commandBuffer, discardRectangleEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkBindImageMemory(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    unimplemented!("vkBindImageMemory(device, image, memory, memoryOffset")
}

#[no_mangle]
pub unsafe extern "C" fn vkFreeCommandBuffers(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: Option<NonNull<VkCommandBuffer>>,
) {
    unimplemented!("vkFreeCommandBuffers(device, commandPool, commandBufferCount, pCommandBuffers")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetQueueCheckpointData2NV(
    queue: VkQueue,
    pCheckpointDataCount: Option<NonNull<u32>>,
    pCheckpointData: Option<NonNull<VkCheckpointData2NV>>,
) {
    unimplemented!("vkGetQueueCheckpointData2NV(queue, pCheckpointDataCount, pCheckpointData")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateMacOSSurfaceMVK(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkMacOSSurfaceCreateInfoMVK>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateMacOSSurfaceMVK(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkSetEvent(device, event")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilReference(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    reference: u32,
) {
    unimplemented!("vkCmdSetStencilReference(commandBuffer, faceMask, reference")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPipeline(
    device: VkDevice,
    pipeline: VkPipeline,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyPipeline(device, pipeline, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBuffer(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: Option<NonNull<VkBufferCopy>>,
) {
    unimplemented!("vkCmdCopyBuffer(commandBuffer, srcBuffer, dstBuffer, regionCount, pRegions")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceMemoryProperties2(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: Option<NonNull<VkPhysicalDeviceMemoryProperties2>>,
) {
    unimplemented!("vkGetPhysicalDeviceMemoryProperties2(physicalDevice, pMemoryProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSparseImageFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    pFormatInfo: Option<NonNull<VkPhysicalDeviceSparseImageFormatInfo2>>,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkSparseImageFormatProperties2>>,
) {
    unimplemented!(
        "vkGetPhysicalDeviceSparseImageFormatProperties2(
        physicalDevice,
        pFormatInfo,
        pPropertyCount,
        pProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayPlaneCapabilities2KHR(
    physicalDevice: VkPhysicalDevice,
    pDisplayPlaneInfo: Option<NonNull<VkDisplayPlaneInfo2KHR>>,
    pCapabilities: Option<NonNull<VkDisplayPlaneCapabilities2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayPlaneCapabilities2KHR(physicalDevice, pDisplayPlaneInfo, pCapabilities"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCheckpointNV(
    commandBuffer: VkCommandBuffer,
    pCheckpointMarker: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!("vkCmdSetCheckpointNV(commandBuffer, pCheckpointMarker")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateStreamDescriptorSurfaceGGP(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkStreamDescriptorSurfaceCreateInfoGGP>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateStreamDescriptorSurfaceGGP(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdExecuteGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: Option<NonNull<VkGeneratedCommandsInfoNV>>,
) {
    unimplemented!(
        "vkCmdExecuteGeneratedCommandsNV(commandBuffer, isPreprocessed, pGeneratedCommandsInfo"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSemaphoreSciSyncPoolNV(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSemaphoreSciSyncPoolCreateInfoNV>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSemaphorePool: Option<NonNull<VkSemaphoreSciSyncPoolNV>>,
) -> VkResult {
    unimplemented!("vkCreateSemaphoreSciSyncPoolNV(device, pCreateInfo, pAllocator, pSemaphorePool")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBias(
    commandBuffer: VkCommandBuffer,
    depthBiasConstantFactor: f32,
    depthBiasClamp: f32,
    depthBiasSlopeFactor: f32,
) {
    unimplemented!(
        "vkCmdSetDepthBias(
        commandBuffer,
        depthBiasConstantFactor,
        depthBiasClamp,
        depthBiasSlopeFactor,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawClusterIndirectHUAWEI(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    unimplemented!("vkCmdDrawClusterIndirectHUAWEI(commandBuffer, buffer, offset")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySemaphoreSciSyncPoolNV(
    device: VkDevice,
    semaphorePool: VkSemaphoreSciSyncPoolNV,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySemaphoreSciSyncPoolNV(device, semaphorePool, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueSubmit(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: Option<NonNull<VkSubmitInfo>>,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueSubmit(queue, submitCount, pSubmits, fence")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetHdrMetadataEXT(
    device: VkDevice,
    swapchainCount: u32,
    pSwapchains: Option<NonNull<VkSwapchainKHR>>,
    pMetadata: Option<NonNull<VkHdrMetadataEXT>>,
) {
    unimplemented!("vkSetHdrMetadataEXT(device, swapchainCount, pSwapchains, pMetadata")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkCooperativeMatrixPropertiesNV>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(
        physicalDevice,
        pPropertyCount,
        pProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndirectByteCountEXT(
    commandBuffer: VkCommandBuffer,
    instanceCount: u32,
    firstInstance: u32,
    counterBuffer: VkBuffer,
    counterBufferOffset: VkDeviceSize,
    counterOffset: u32,
    vertexStride: u32,
) {
    unimplemented!(
        "vkCmdDrawIndirectByteCountEXT(
        commandBuffer,
        instanceCount,
        firstInstance,
        counterBuffer,
        counterBufferOffset,
        counterOffset,
        vertexStride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    connection: Option<NonNull<xcb_connection_t>>,
    visual_id: xcb_visualid_t,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceXcbPresentationSupportKHR(
        physicalDevice,
        queueFamilyIndex,
        connection,
        visual_id,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdTraceRaysIndirect2KHR(
    commandBuffer: VkCommandBuffer,
    indirectDeviceAddress: VkDeviceAddress,
) {
    unimplemented!("vkCmdTraceRaysIndirect2KHR(commandBuffer, indirectDeviceAddress")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryWin32HandlePropertiesKHR(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    pMemoryWin32HandleProperties: Option<NonNull<VkMemoryWin32HandlePropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetMemoryWin32HandlePropertiesKHR(
        device,
        handleType,
        handle,
        pMemoryWin32HandleProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSparseImageFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    samples: VkSampleCountFlagBits,
    usage: VkImageUsageFlags,
    tiling: VkImageTiling,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkSparseImageFormatProperties>>,
) {
    unimplemented!(
        "vkGetPhysicalDeviceSparseImageFormatProperties(
        physicalDevice,
        format,
        type_,
        samples,
        usage,
        tiling,
        pPropertyCount,
        pProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkInitializePerformanceApiINTEL(
    device: VkDevice,
    pInitializeInfo: Option<NonNull<VkInitializePerformanceApiInfoINTEL>>,
) -> VkResult {
    unimplemented!("vkInitializePerformanceApiINTEL(device, pInitializeInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateIndirectCommandsLayoutNV(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkIndirectCommandsLayoutCreateInfoNV>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pIndirectCommandsLayout: Option<NonNull<VkIndirectCommandsLayoutNV>>,
) -> VkResult {
    unimplemented!(
        "vkCreateIndirectCommandsLayoutNV(device, pCreateInfo, pAllocator, pIndirectCommandsLayout"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBlitImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: Option<NonNull<VkImageBlit>>,
    filter: VkFilter,
) {
    unimplemented!(
        "vkCmdBlitImage(
        commandBuffer,
        srcImage,
        srcImageLayout,
        dstImage,
        dstImageLayout,
        regionCount,
        pRegions,
        filter,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRasterizationStreamEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationStream: u32,
) {
    unimplemented!("vkCmdSetRasterizationStreamEXT(commandBuffer, rasterizationStream")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageViewOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageViewCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetImageViewOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateCuFunctionNVX(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkCuFunctionCreateInfoNVX>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFunction: Option<NonNull<VkCuFunctionNVX>>,
) -> VkResult {
    unimplemented!("vkCreateCuFunctionNVX(device, pCreateInfo, pAllocator, pFunction")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyMicromapEXT(
    device: VkDevice,
    micromap: VkMicromapEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyMicromapEXT(device, micromap, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateDeviceLayerProperties(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkLayerProperties>>,
) -> VkResult {
    unimplemented!("vkEnumerateDeviceLayerProperties(physicalDevice, pPropertyCount, pProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkReleaseSwapchainImagesEXT(
    device: VkDevice,
    pReleaseInfo: Option<NonNull<VkReleaseSwapchainImagesInfoEXT>>,
) -> VkResult {
    unimplemented!("vkReleaseSwapchainImagesEXT(device, pReleaseInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdExecuteCommands(
    commandBuffer: VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers: Option<NonNull<VkCommandBuffer>>,
) {
    unimplemented!("vkCmdExecuteCommands(commandBuffer, commandBufferCount, pCommandBuffers")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutHostMappingInfoVALVE(
    device: VkDevice,
    pBindingReference: Option<NonNull<VkDescriptorSetBindingReferenceVALVE>>,
    pHostMapping: Option<NonNull<VkDescriptorSetLayoutHostMappingInfoVALVE>>,
) {
    unimplemented!(
        "vkGetDescriptorSetLayoutHostMappingInfoVALVE(device, pBindingReference, pHostMapping"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireFullScreenExclusiveModeEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkAcquireFullScreenExclusiveModeEXT(device, swapchain")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyAccelerationStructureNV(
    commandBuffer: VkCommandBuffer,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    mode: VkCopyAccelerationStructureModeKHR,
) {
    unimplemented!("vkCmdCopyAccelerationStructureNV(commandBuffer, dst, src, mode")
}

#[no_mangle]
pub unsafe extern "C" fn vkQueuePresentKHR(
    queue: VkQueue,
    pPresentInfo: Option<NonNull<VkPresentInfoKHR>>,
) -> VkResult {
    unimplemented!("vkQueuePresentKHR(queue, pPresentInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyBufferToImage2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: Option<NonNull<VkCopyBufferToImageInfo2>>,
) {
    unimplemented!("vkCmdCopyBufferToImage2(commandBuffer, pCopyBufferToImageInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateIOSSurfaceMVK(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkIOSSurfaceCreateInfoMVK>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateIOSSurfaceMVK(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pUpdateInfo: Option<NonNull<VkVideoSessionParametersUpdateInfoKHR>>,
) -> VkResult {
    unimplemented!("vkUpdateVideoSessionParametersKHR(device, videoSessionParameters, pUpdateInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyShaderEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyShaderEXT(device, shader, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteBufferMarker2AMD(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
) {
    unimplemented!("vkCmdWriteBufferMarker2AMD(commandBuffer, stage, dstBuffer, dstOffset, marker")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSwapchainKHR(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSwapchainCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSwapchain: Option<NonNull<VkSwapchainKHR>>,
) -> VkResult {
    unimplemented!("vkCreateSwapchainKHR(device, pCreateInfo, pAllocator, pSwapchain")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: Option<NonNull<VkFormatProperties>>,
) {
    unimplemented!("vkGetPhysicalDeviceFormatProperties(physicalDevice, format, pFormatProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceVersion(pApiVersion: Option<NonNull<u32>>) -> VkResult {
    unimplemented!("vkEnumerateInstanceVersion(pApiVersion")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSharedSwapchainsKHR(
    device: VkDevice,
    swapchainCount: u32,
    pCreateInfos: Option<NonNull<VkSwapchainCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSwapchains: Option<NonNull<VkSwapchainKHR>>,
) -> VkResult {
    unimplemented!(
        "vkCreateSharedSwapchainsKHR(
        device,
        swapchainCount,
        pCreateInfos,
        pAllocator,
        pSwapchains,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetValidationCacheDataEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pDataSize: Option<NonNull<isize>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetValidationCacheDataEXT(device, validationCache, pDataSize, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquirePerformanceConfigurationINTEL(
    device: VkDevice,
    pAcquireInfo: Option<NonNull<VkPerformanceConfigurationAcquireInfoINTEL>>,
    pConfiguration: Option<NonNull<VkPerformanceConfigurationINTEL>>,
) -> VkResult {
    unimplemented!("vkAcquirePerformanceConfigurationINTEL(device, pAcquireInfo, pConfiguration")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyRenderPass(
    device: VkDevice,
    renderPass: VkRenderPass,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyRenderPass(device, renderPass, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateMicromapEXT(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkMicromapCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMicromap: Option<NonNull<VkMicromapEXT>>,
) -> VkResult {
    unimplemented!("vkCreateMicromapEXT(device, pCreateInfo, pAllocator, pMicromap")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateQueryPool(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkQueryPoolCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pQueryPool: Option<NonNull<VkQueryPool>>,
) -> VkResult {
    unimplemented!("vkCreateQueryPool(device, pCreateInfo, pAllocator, pQueryPool")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutSupport(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorSetLayoutCreateInfo>>,
    pSupport: Option<NonNull<VkDescriptorSetLayoutSupport>>,
) {
    unimplemented!("vkGetDescriptorSetLayoutSupport(device, pCreateInfo, pSupport")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdControlVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pCodingControlInfo: Option<NonNull<VkVideoCodingControlInfoKHR>>,
) {
    unimplemented!("vkCmdControlVideoCodingKHR(commandBuffer, pCodingControlInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetDebugUtilsObjectNameEXT(
    device: VkDevice,
    pNameInfo: Option<NonNull<VkDebugUtilsObjectNameInfoEXT>>,
) -> VkResult {
    unimplemented!("vkSetDebugUtilsObjectNameEXT(device, pNameInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateXlibSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkXlibSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateXlibSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkRegisterDisplayEventEXT(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayEventInfo: Option<NonNull<VkDisplayEventInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFence: Option<NonNull<VkFence>>,
) -> VkResult {
    unimplemented!(
        "vkRegisterDisplayEventEXT(device, display, pDisplayEventInfo, pAllocator, pFence"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteAccelerationStructuresPropertiesNV(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: Option<NonNull<VkAccelerationStructureNV>>,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!(
        "vkCmdWriteAccelerationStructuresPropertiesNV(
        commandBuffer,
        accelerationStructureCount,
        pAccelerationStructures,
        queryType,
        queryPool,
        firstQuery,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pQualityLevelInfo: Option<NonNull<VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR>>,
    pQualityLevelProperties: Option<NonNull<VkVideoEncodeQualityLevelPropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
        physicalDevice,
        pQualityLevelInfo,
        pQualityLevelProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCuLaunchKernelNVX(
    commandBuffer: VkCommandBuffer,
    pLaunchInfo: Option<NonNull<VkCuLaunchInfoNVX>>,
) {
    unimplemented!("vkCmdCuLaunchKernelNVX(commandBuffer, pLaunchInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyQueryPool(device, queryPool, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueBeginDebugUtilsLabelEXT(
    queue: VkQueue,
    pLabelInfo: Option<NonNull<VkDebugUtilsLabelEXT>>,
) {
    unimplemented!("vkQueueBeginDebugUtilsLabelEXT(queue, pLabelInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyAccelerationStructureToMemoryKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyAccelerationStructureToMemoryInfoKHR>>,
) {
    unimplemented!("vkCmdCopyAccelerationStructureToMemoryKHR(commandBuffer, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkGetEventStatus(device, event")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: Option<NonNull<VkPhysicalDeviceSurfaceInfo2KHR>>,
    pSurfaceCapabilities: Option<NonNull<VkSurfaceCapabilities2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSurfaceCapabilities2KHR(
        physicalDevice,
        pSurfaceInfo,
        pSurfaceCapabilities,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceImageMemoryRequirements(
    device: VkDevice,
    pInfo: Option<NonNull<VkDeviceImageMemoryRequirements>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetDeviceImageMemoryRequirements(device, pInfo, pMemoryRequirements")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFeatures2(
    physicalDevice: VkPhysicalDevice,
    pFeatures: Option<NonNull<VkPhysicalDeviceFeatures2>>,
) {
    unimplemented!("vkGetPhysicalDeviceFeatures2(physicalDevice, pFeatures")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyVideoSessionKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyVideoSessionKHR(device, videoSession, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateHeadlessSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkHeadlessSurfaceCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateHeadlessSurfaceEXT(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemorySciBufNV(
    device: VkDevice,
    pGetSciBufInfo: Option<NonNull<VkMemoryGetSciBufInfoNV>>,
    pHandle: Option<NonNull<NvSciBufObj>>,
) -> VkResult {
    unimplemented!("vkGetMemorySciBufNV(device, pGetSciBufInfo, pHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: Option<NonNull<VkMemoryGetWin32HandleInfoKHR>>,
    pHandle: Option<NonNull<HANDLE>>,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandleKHR(device, pGetWin32HandleInfo, pHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkImportFenceWin32HandleKHR(
    device: VkDevice,
    pImportFenceWin32HandleInfo: Option<NonNull<VkImportFenceWin32HandleInfoKHR>>,
) -> VkResult {
    unimplemented!("vkImportFenceWin32HandleKHR(device, pImportFenceWin32HandleInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWaitEvents(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: Option<NonNull<VkEvent>>,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: Option<NonNull<VkMemoryBarrier>>,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: Option<NonNull<VkBufferMemoryBarrier>>,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: Option<NonNull<VkImageMemoryBarrier>>,
) {
    unimplemented!(
        "vkCmdWaitEvents(
        commandBuffer,
        eventCount,
        pEvents,
        srcStageMask,
        dstStageMask,
        memoryBarrierCount,
        pMemoryBarriers,
        bufferMemoryBarrierCount,
        pBufferMemoryBarriers,
        imageMemoryBarrierCount,
        pImageMemoryBarriers,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkResetFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: Option<NonNull<VkFence>>,
) -> VkResult {
    unimplemented!("vkResetFences(device, fenceCount, pFences")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSparseMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pSparseMemoryRequirementCount: Option<NonNull<u32>>,
    pSparseMemoryRequirements: Option<NonNull<VkSparseImageMemoryRequirements>>,
) {
    unimplemented!(
        "vkGetImageSparseMemoryRequirements(
        device,
        image,
        pSparseMemoryRequirementCount,
        pSparseMemoryRequirements,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRandROutputDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    dpy: Option<NonNull<Display>>,
    rrOutput: RROutput,
    pDisplay: Option<NonNull<VkDisplayKHR>>,
) -> VkResult {
    unimplemented!("vkGetRandROutputDisplayEXT(physicalDevice, dpy, rrOutput, pDisplay")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRefreshCycleDurationGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pDisplayTimingProperties: Option<NonNull<VkRefreshCycleDurationGOOGLE>>,
) -> VkResult {
    unimplemented!("vkGetRefreshCycleDurationGOOGLE(device, swapchain, pDisplayTimingProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindPipelineShaderGroupNV(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    groupIndex: u32,
) {
    unimplemented!(
        "vkCmdBindPipelineShaderGroupNV(commandBuffer, pipelineBindPoint, pipeline, groupIndex"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyBufferView(
    device: VkDevice,
    bufferView: VkBufferView,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyBufferView(device, bufferView, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPipelineLayout(
    device: VkDevice,
    pipelineLayout: VkPipelineLayout,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyPipelineLayout(device, pipelineLayout, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageViewHandleNVX(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageViewHandleInfoNVX>>,
) -> u32 {
    unimplemented!("vkGetImageViewHandleNVX(device, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateWin32SurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkWin32SurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateWin32SurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetDebugUtilsObjectTagEXT(
    device: VkDevice,
    pTagInfo: Option<NonNull<VkDebugUtilsObjectTagInfoEXT>>,
) -> VkResult {
    unimplemented!("vkSetDebugUtilsObjectTagEXT(device, pTagInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineStippleEXT(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    unimplemented!("vkCmdSetLineStippleEXT(commandBuffer, lineStippleFactor, lineStipplePattern")
}

#[no_mangle]
pub unsafe extern "C" fn vkMergePipelineCaches(
    device: VkDevice,
    dstCache: VkPipelineCache,
    srcCacheCount: u32,
    pSrcCaches: Option<NonNull<VkPipelineCache>>,
) -> VkResult {
    unimplemented!("vkMergePipelineCaches(device, dstCache, srcCacheCount, pSrcCaches")
}

#[no_mangle]
pub unsafe extern "C" fn vkImportSemaphoreSciSyncObjNV(
    device: VkDevice,
    pImportSemaphoreSciSyncInfo: Option<NonNull<VkImportSemaphoreSciSyncInfoNV>>,
) -> VkResult {
    unimplemented!("vkImportSemaphoreSciSyncObjNV(device, pImportSemaphoreSciSyncInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMemoryToImageIndirectNV(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pImageSubresources: Option<NonNull<VkImageSubresourceLayers>>,
) {
    unimplemented!(
        "vkCmdCopyMemoryToImageIndirectNV(
        commandBuffer,
        copyBufferAddress,
        copyCount,
        stride,
        dstImage,
        dstImageLayout,
        pImageSubresources,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDescriptorPool(device, descriptorPool, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkMergeValidationCachesEXT(
    device: VkDevice,
    dstCache: VkValidationCacheEXT,
    srcCacheCount: u32,
    pSrcCaches: Option<NonNull<VkValidationCacheEXT>>,
) -> VkResult {
    unimplemented!("vkMergeValidationCachesEXT(device, dstCache, srcCacheCount, pSrcCaches")
}

#[no_mangle]
pub unsafe extern "C" fn vkImportFenceFdKHR(
    device: VkDevice,
    pImportFenceFdInfo: Option<NonNull<VkImportFenceFdInfoKHR>>,
) -> VkResult {
    unimplemented!("vkImportFenceFdKHR(device, pImportFenceFdInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndexedIndirectCount(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDrawIndexedIndirectCount(
        commandBuffer,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthClipNegativeOneToOneEXT(
    commandBuffer: VkCommandBuffer,
    negativeOneToOne: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClipNegativeOneToOneEXT(commandBuffer, negativeOneToOne")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutBindingOffsetEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    binding: u32,
    pOffset: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!("vkGetDescriptorSetLayoutBindingOffsetEXT(device, layout, binding, pOffset")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayModePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayModePropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayModePropertiesKHR(physicalDevice, display, pPropertyCount, pProperties"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkAllocateMemory(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkMemoryAllocateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMemory: Option<NonNull<VkDeviceMemory>>,
) -> VkResult {
    unimplemented!("vkAllocateMemory(device, pAllocateInfo, pAllocator, pMemory")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFenceWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: Option<NonNull<VkFenceGetWin32HandleInfoKHR>>,
    pHandle: Option<NonNull<HANDLE>>,
) -> VkResult {
    unimplemented!("vkGetFenceWin32HandleKHR(device, pGetWin32HandleInfo, pHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkImportFenceSciSyncObjNV(
    device: VkDevice,
    pImportFenceSciSyncInfo: Option<NonNull<VkImportFenceSciSyncInfoNV>>,
) -> VkResult {
    unimplemented!("vkImportFenceSciSyncObjNV(device, pImportFenceSciSyncInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetWinrtDisplayNV(
    physicalDevice: VkPhysicalDevice,
    deviceRelativeId: u32,
    pDisplay: Option<NonNull<VkDisplayKHR>>,
) -> VkResult {
    unimplemented!("vkGetWinrtDisplayNV(physicalDevice, deviceRelativeId, pDisplay")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSubpassShadingHUAWEI(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdSubpassShadingHUAWEI(commandBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult {
    unimplemented!("vkGetFenceStatus(device, fence")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainStatusKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkGetSwapchainStatusKHR(device, swapchain")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreatePipelineCache(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkPipelineCacheCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelineCache: Option<NonNull<VkPipelineCache>>,
) -> VkResult {
    unimplemented!("vkCreatePipelineCache(device, pCreateInfo, pAllocator, pPipelineCache")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceGroupPresentCapabilitiesKHR(
    device: VkDevice,
    pDeviceGroupPresentCapabilities: Option<NonNull<VkDeviceGroupPresentCapabilitiesKHR>>,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupPresentCapabilitiesKHR(device, pDeviceGroupPresentCapabilities")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBoundsTestEnable(
    commandBuffer: VkCommandBuffer,
    depthBoundsTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthBoundsTestEnable(commandBuffer, depthBoundsTestEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkResetEvent(device, event")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportWScalingEnableNV(
    commandBuffer: VkCommandBuffer,
    viewportWScalingEnable: VkBool32,
) {
    unimplemented!("vkCmdSetViewportWScalingEnableNV(commandBuffer, viewportWScalingEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBlitImage2(
    commandBuffer: VkCommandBuffer,
    pBlitImageInfo: Option<NonNull<VkBlitImageInfo2>>,
) {
    unimplemented!("vkCmdBlitImage2(commandBuffer, pBlitImageInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdOpticalFlowExecuteNV(
    commandBuffer: VkCommandBuffer,
    session: VkOpticalFlowSessionNV,
    pExecuteInfo: Option<NonNull<VkOpticalFlowExecuteInfoNV>>,
) {
    unimplemented!("vkCmdOpticalFlowExecuteNV(commandBuffer, session, pExecuteInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthCompareOp(
    commandBuffer: VkCommandBuffer,
    depthCompareOp: VkCompareOp,
) {
    unimplemented!("vkCmdSetDepthCompareOp(commandBuffer, depthCompareOp")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelinePropertiesEXT(
    device: VkDevice,
    pPipelineInfo: Option<NonNull<VkPipelineInfoEXT>>,
    pPipelineProperties: Option<NonNull<VkBaseOutStructure>>,
) -> VkResult {
    unimplemented!("vkGetPipelinePropertiesEXT(device, pPipelineInfo, pPipelineProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkExportMetalObjectsEXT(
    device: VkDevice,
    pMetalObjectsInfo: Option<NonNull<VkExportMetalObjectsInfoEXT>>,
) {
    unimplemented!("vkExportMetalObjectsEXT(device, pMetalObjectsInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkRegisterDeviceEventEXT(
    device: VkDevice,
    pDeviceEventInfo: Option<NonNull<VkDeviceEventInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFence: Option<NonNull<VkFence>>,
) -> VkResult {
    unimplemented!("vkRegisterDeviceEventEXT(device, pDeviceEventInfo, pAllocator, pFence")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorBuffersEXT(
    commandBuffer: VkCommandBuffer,
    bufferCount: u32,
    pBindingInfos: Option<NonNull<VkDescriptorBufferBindingInfoEXT>>,
) {
    unimplemented!("vkCmdBindDescriptorBuffersEXT(commandBuffer, bufferCount, pBindingInfos")
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMicromapToMemoryEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyMicromapToMemoryInfoEXT>>,
) -> VkResult {
    unimplemented!("vkCopyMicromapToMemoryEXT(device, deferredOperation, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkReleaseProfilingLockKHR(device: VkDevice) {
    unimplemented!("vkReleaseProfilingLockKHR(device")
}

#[no_mangle]
pub unsafe extern "C" fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult {
    unimplemented!("vkEndCommandBuffer(commandBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageToColorLocationNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorLocation: u32,
) {
    unimplemented!("vkCmdSetCoverageToColorLocationNV(commandBuffer, coverageToColorLocation")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMemoryToMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyMemoryToMicromapInfoEXT>>,
) {
    unimplemented!("vkCmdCopyMemoryToMicromapEXT(commandBuffer, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceAccelerationStructureCompatibilityKHR(
    device: VkDevice,
    pVersionInfo: Option<NonNull<VkAccelerationStructureVersionInfoKHR>>,
    pCompatibility: Option<NonNull<VkAccelerationStructureCompatibilityKHR>>,
) {
    unimplemented!(
        "vkGetDeviceAccelerationStructureCompatibilityKHR(device, pVersionInfo, pCompatibility"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindPipeline(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
) {
    unimplemented!("vkCmdBindPipeline(commandBuffer, pipelineBindPoint, pipeline")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageDrmFormatModifierPropertiesEXT(
    device: VkDevice,
    image: VkImage,
    pProperties: Option<NonNull<VkImageDrmFormatModifierPropertiesEXT>>,
) -> VkResult {
    unimplemented!("vkGetImageDrmFormatModifierPropertiesEXT(device, image, pProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResolveImage2(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: Option<NonNull<VkResolveImageInfo2>>,
) {
    unimplemented!("vkCmdResolveImage2(commandBuffer, pResolveImageInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyMicromapInfoEXT>>,
) {
    unimplemented!("vkCmdCopyMicromapEXT(commandBuffer, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkFreeDescriptorSets(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: Option<NonNull<VkDescriptorSet>>,
) -> VkResult {
    unimplemented!(
        "vkFreeDescriptorSets(device, descriptorPool, descriptorSetCount, pDescriptorSets"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueWaitIdle(queue: VkQueue) -> VkResult {
    unimplemented!("vkQueueWaitIdle(queue")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRepresentativeFragmentTestEnableNV(
    commandBuffer: VkCommandBuffer,
    representativeFragmentTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetRepresentativeFragmentTestEnableNV(commandBuffer, representativeFragmentTestEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFramebufferTilePropertiesQCOM(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pPropertiesCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkTilePropertiesQCOM>>,
) -> VkResult {
    unimplemented!(
        "vkGetFramebufferTilePropertiesQCOM(device, framebuffer, pPropertiesCount, pProperties"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDevice(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: Option<NonNull<VkDeviceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDevice: Option<NonNull<VkDevice>>,
) -> VkResult {
    unimplemented!("vkCreateDevice(physicalDevice, pCreateInfo, pAllocator, pDevice")
}

#[no_mangle]
pub unsafe extern "C" fn vkUninitializePerformanceApiINTEL(device: VkDevice) {
    unimplemented!("vkUninitializePerformanceApiINTEL(device")
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireImageANDROID(
    device: VkDevice,
    image: VkImage,
    nativeFenceFd: int,
    semaphore: VkSemaphore,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkAcquireImageANDROID(device, image, nativeFenceFd, semaphore, fence")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPerformanceOverrideINTEL(
    commandBuffer: VkCommandBuffer,
    pOverrideInfo: Option<NonNull<VkPerformanceOverrideInfoINTEL>>,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceOverrideINTEL(commandBuffer, pOverrideInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreatePipelineLayout(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkPipelineLayoutCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelineLayout: Option<NonNull<VkPipelineLayout>>,
) -> VkResult {
    unimplemented!("vkCreatePipelineLayout(device, pCreateInfo, pAllocator, pPipelineLayout")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: Option<NonNull<VkDisplayKHR>>,
) -> VkResult {
    unimplemented!("vkGetDrmDisplayEXT(physicalDevice, drmFd, connectorId, display")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    display: Option<NonNull<wl_display>>,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice, queueFamilyIndex, display")
}

#[no_mangle]
pub unsafe extern "C" fn vkDebugMarkerSetObjectNameEXT(
    device: VkDevice,
    pNameInfo: Option<NonNull<VkDebugMarkerObjectNameInfoEXT>>,
) -> VkResult {
    unimplemented!("vkDebugMarkerSetObjectNameEXT(device, pNameInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireXlibDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    dpy: Option<NonNull<Display>>,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireXlibDisplayEXT(physicalDevice, dpy, display")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImage(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: Option<NonNull<VkImageCopy>>,
) {
    unimplemented!(
        "vkCmdCopyImage(
        commandBuffer,
        srcImage,
        srcImageLayout,
        dstImage,
        dstImageLayout,
        regionCount,
        pRegions,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkBindImageMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: Option<NonNull<VkBindImageMemoryInfo>>,
) -> VkResult {
    unimplemented!("vkBindImageMemory2(device, bindInfoCount, pBindInfos")
}

#[no_mangle]
pub unsafe extern "C" fn vkDisplayPowerControlEXT(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayPowerInfo: Option<NonNull<VkDisplayPowerInfoEXT>>,
) -> VkResult {
    unimplemented!("vkDisplayPowerControlEXT(device, display, pDisplayPowerInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPastPresentationTimingGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pPresentationTimingCount: Option<NonNull<u32>>,
    pPresentationTimings: Option<NonNull<VkPastPresentationTimingGOOGLE>>,
) -> VkResult {
    unimplemented!(
        "vkGetPastPresentationTimingGOOGLE(
        device,
        swapchain,
        pPresentationTimingCount,
        pPresentationTimings,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDynamicRenderingTilePropertiesQCOM(
    device: VkDevice,
    pRenderingInfo: Option<NonNull<VkRenderingInfo>>,
    pProperties: Option<NonNull<VkTilePropertiesQCOM>>,
) -> VkResult {
    unimplemented!("vkGetDynamicRenderingTilePropertiesQCOM(device, pRenderingInfo, pProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRayTracingPipelinesNV(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: Option<NonNull<VkRayTracingPipelineCreateInfoNV>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelines: Option<NonNull<VkPipeline>>,
) -> VkResult {
    unimplemented!(
        "vkCreateRayTracingPipelinesNV(
        device,
        pipelineCache,
        createInfoCount,
        pCreateInfos,
        pAllocator,
        pPipelines,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyBuffer(
    device: VkDevice,
    buffer: VkBuffer,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyBuffer(device, buffer, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
) {
    unimplemented!("vkCmdBeginQuery(commandBuffer, queryPool, query, flags")
}

#[no_mangle]
pub unsafe extern "C" fn vkReleasePerformanceConfigurationINTEL(
    device: VkDevice,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!("vkReleasePerformanceConfigurationINTEL(device, configuration")
}

#[no_mangle]
pub unsafe extern "C" fn vkFlushMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: Option<NonNull<VkMappedMemoryRange>>,
) -> VkResult {
    unimplemented!("vkFlushMappedMemoryRanges(device, memoryRangeCount, pMemoryRanges")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyShaderModule(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyShaderModule(device, shaderModule, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    physicalDevice: VkPhysicalDevice,
    pTimeDomainCount: Option<NonNull<u32>>,
    pTimeDomains: Option<NonNull<VkTimeDomainEXT>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
        physicalDevice,
        pTimeDomainCount,
        pTimeDomains,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilOp(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
) {
    unimplemented!(
        "vkCmdSetStencilOp(
        commandBuffer,
        faceMask,
        failOp,
        passOp,
        depthFailOp,
        compareOp,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetVideoSessionMemoryRequirementsKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pMemoryRequirementsCount: Option<NonNull<u32>>,
    pMemoryRequirements: Option<NonNull<VkVideoSessionMemoryRequirementsKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetVideoSessionMemoryRequirementsKHR(
        device,
        videoSession,
        pMemoryRequirementsCount,
        pMemoryRequirements,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMicromapToMemoryEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyMicromapToMemoryInfoEXT>>,
) {
    unimplemented!("vkCmdCopyMicromapToMemoryEXT(commandBuffer, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayPlaneProperties2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayPlaneProperties2KHR(physicalDevice, pPropertyCount, pProperties"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkMapMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    ppData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkMapMemory(device, memory, offset, size, flags, ppData")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainImagesKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: Option<NonNull<u32>>,
    pSwapchainImages: Option<NonNull<VkImage>>,
) -> VkResult {
    unimplemented!(
        "vkGetSwapchainImagesKHR(device, swapchain, pSwapchainImageCount, pSwapchainImages"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSemaphoreWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: Option<NonNull<VkSemaphoreGetWin32HandleInfoKHR>>,
    pHandle: Option<NonNull<HANDLE>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreWin32HandleKHR(device, pGetWin32HandleInfo, pHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkImportSemaphoreWin32HandleKHR(
    device: VkDevice,
    pImportSemaphoreWin32HandleInfo: Option<NonNull<VkImportSemaphoreWin32HandleInfoKHR>>,
) -> VkResult {
    unimplemented!("vkImportSemaphoreWin32HandleKHR(device, pImportSemaphoreWin32HandleInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pEndCodingInfo: Option<NonNull<VkVideoEndCodingInfoKHR>>,
) {
    unimplemented!("vkCmdEndVideoCodingKHR(commandBuffer, pEndCodingInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateShadersEXT(
    device: VkDevice,
    createInfoCount: u32,
    pCreateInfos: Option<NonNull<VkShaderCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pShaders: Option<NonNull<VkShaderEXT>>,
) -> VkResult {
    unimplemented!("vkCreateShadersEXT(device, createInfoCount, pCreateInfos, pAllocator, pShaders")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceProperties2(
    physicalDevice: VkPhysicalDevice,
    pProperties: Option<NonNull<VkPhysicalDeviceProperties2>>,
) {
    unimplemented!("vkGetPhysicalDeviceProperties2(physicalDevice, pProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkDebugMarkerSetObjectTagEXT(
    device: VkDevice,
    pTagInfo: Option<NonNull<VkDebugMarkerObjectTagInfoEXT>>,
) -> VkResult {
    unimplemented!("vkDebugMarkerSetObjectTagEXT(device, pTagInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageReductionModeNV(
    commandBuffer: VkCommandBuffer,
    coverageReductionMode: VkCoverageReductionModeNV,
) {
    unimplemented!("vkCmdSetCoverageReductionModeNV(commandBuffer, coverageReductionMode")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSamplerOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkSamplerCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetSamplerOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdTraceRaysNV(
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
    unimplemented!(
        "vkCmdTraceRaysNV(
        commandBuffer,
        raygenShaderBindingTableBuffer,
        raygenShaderBindingOffset,
        missShaderBindingTableBuffer,
        missShaderBindingOffset,
        missShaderBindingStride,
        hitShaderBindingTableBuffer,
        hitShaderBindingOffset,
        hitShaderBindingStride,
        callableShaderBindingTableBuffer,
        callableShaderBindingOffset,
        callableShaderBindingStride,
        width,
        height,
        depth,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEncodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pEncodeInfo: Option<NonNull<VkVideoEncodeInfoKHR>>,
) {
    unimplemented!("vkCmdEncodeVideoKHR(commandBuffer, pEncodeInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSciBufAttributesNV(
    physicalDevice: VkPhysicalDevice,
    pAttributes: NvSciBufAttrList,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSciBufAttributesNV(physicalDevice, pAttributes")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainCounterEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    counter: VkSurfaceCounterFlagBitsEXT,
    pCounterValue: Option<NonNull<u64>>,
) -> VkResult {
    unimplemented!("vkGetSwapchainCounterEXT(device, swapchain, counter, pCounterValue")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetPrivateData(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
) -> VkResult {
    unimplemented!("vkSetPrivateData(device, objectType, objectHandle, privateDataSlot, data")
}

#[no_mangle]
pub unsafe extern "C" fn vkAllocateCommandBuffers(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkCommandBufferAllocateInfo>>,
    pCommandBuffers: Option<NonNull<VkCommandBuffer>>,
) -> VkResult {
    unimplemented!("vkAllocateCommandBuffers(device, pAllocateInfo, pCommandBuffers")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindIndexBuffer(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    indexType: VkIndexType,
) {
    unimplemented!("vkCmdBindIndexBuffer(commandBuffer, buffer, offset, indexType")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawIndirect(commandBuffer, buffer, offset, drawCount, stride")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSparseMemoryRequirements2(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageSparseMemoryRequirementsInfo2>>,
    pSparseMemoryRequirementCount: Option<NonNull<u32>>,
    pSparseMemoryRequirements: Option<NonNull<VkSparseImageMemoryRequirements2>>,
) {
    unimplemented!(
        "vkGetImageSparseMemoryRequirements2(
        device,
        pInfo,
        pSparseMemoryRequirementCount,
        pSparseMemoryRequirements,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDescriptorSetLayout(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDescriptorSetLayout(device, descriptorSetLayout, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindDescriptorSets(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: Option<NonNull<VkDescriptorSet>>,
    dynamicOffsetCount: u32,
    pDynamicOffsets: Option<NonNull<u32>>,
) {
    unimplemented!(
        "vkCmdBindDescriptorSets(
        commandBuffer,
        pipelineBindPoint,
        layout,
        firstSet,
        descriptorSetCount,
        pDescriptorSets,
        dynamicOffsetCount,
        pDynamicOffsets,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceOpticalFlowImageFormatsNV(
    physicalDevice: VkPhysicalDevice,
    pOpticalFlowImageFormatInfo: Option<NonNull<VkOpticalFlowImageFormatInfoNV>>,
    pFormatCount: Option<NonNull<u32>>,
    pImageFormatProperties: Option<NonNull<VkOpticalFlowImageFormatPropertiesNV>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceOpticalFlowImageFormatsNV(
        physicalDevice,
        pOpticalFlowImageFormatInfo,
        pFormatCount,
        pImageFormatProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewport(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: Option<NonNull<VkViewport>>,
) {
    unimplemented!("vkCmdSetViewport(commandBuffer, firstViewport, viewportCount, pViewports")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetSampleMaskEXT(
    commandBuffer: VkCommandBuffer,
    samples: VkSampleCountFlagBits,
    pSampleMask: Option<NonNull<VkSampleMask>>,
) {
    unimplemented!("vkCmdSetSampleMaskEXT(commandBuffer, samples, pSampleMask")
}

#[no_mangle]
pub unsafe extern "C" fn vkAllocateDescriptorSets(
    device: VkDevice,
    pAllocateInfo: Option<NonNull<VkDescriptorSetAllocateInfo>>,
    pDescriptorSets: Option<NonNull<VkDescriptorSet>>,
) -> VkResult {
    unimplemented!("vkAllocateDescriptorSets(device, pAllocateInfo, pDescriptorSets")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceXlibPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    dpy: Option<NonNull<Display>>,
    visualID: VisualID,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceXlibPresentationSupportKHR(
        physicalDevice,
        queueFamilyIndex,
        dpy,
        visualID,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDirectFBSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkDirectFBSurfaceCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateDirectFBSurfaceEXT(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilWriteMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    writeMask: u32,
) {
    unimplemented!("vkCmdSetStencilWriteMask(commandBuffer, faceMask, writeMask")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateImagePipeSurfaceFUCHSIA(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkImagePipeSurfaceCreateInfoFUCHSIA>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateImagePipeSurfaceFUCHSIA(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateWaylandSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkWaylandSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateWaylandSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetLocalDimmingAMD(
    device: VkDevice,
    swapChain: VkSwapchainKHR,
    localDimmingEnable: VkBool32,
) {
    unimplemented!("vkSetLocalDimmingAMD(device, swapChain, localDimmingEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteTimestamp2(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdWriteTimestamp2(commandBuffer, stage, queryPool, query")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSampler(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSamplerCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSampler: Option<NonNull<VkSampler>>,
) -> VkResult {
    unimplemented!("vkCreateSampler(device, pCreateInfo, pAllocator, pSampler")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfacePresentModesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pPresentModeCount: Option<NonNull<u32>>,
    pPresentModes: Option<NonNull<VkPresentModeKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSurfacePresentModesKHR(
        physicalDevice,
        surface,
        pPresentModeCount,
        pPresentModes,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindVertexBuffers2(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: Option<NonNull<VkBuffer>>,
    pOffsets: Option<NonNull<VkDeviceSize>>,
    pSizes: Option<NonNull<VkDeviceSize>>,
    pStrides: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!(
        "vkCmdBindVertexBuffers2(
        commandBuffer,
        firstBinding,
        bindingCount,
        pBuffers,
        pOffsets,
        pSizes,
        pStrides,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceLayerProperties(
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkLayerProperties>>,
) -> VkResult {
    unimplemented!("vkEnumerateInstanceLayerProperties(pPropertyCount, pProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDeferredOperationKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDeferredOperationKHR(device, operation, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetExclusiveScissorEnableNV(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissorEnables: Option<NonNull<VkBool32>>,
) {
    unimplemented!(
        "vkCmdSetExclusiveScissorEnableNV(
        commandBuffer,
        firstExclusiveScissor,
        exclusiveScissorCount,
        pExclusiveScissorEnables,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetLayoutSizeEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    pLayoutSizeInBytes: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!("vkGetDescriptorSetLayoutSizeEXT(device, layout, pLayoutSizeInBytes")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: Option<NonNull<VkFormatProperties2>>,
) {
    unimplemented!("vkGetPhysicalDeviceFormatProperties2(physicalDevice, format, pFormatProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdInsertDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: Option<NonNull<VkDebugUtilsLabelEXT>>,
) {
    unimplemented!("vkCmdInsertDebugUtilsLabelEXT(commandBuffer, pLabelInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportSwizzleNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportSwizzles: Option<NonNull<VkViewportSwizzleNV>>,
) {
    unimplemented!(
        "vkCmdSetViewportSwizzleNV(
        commandBuffer,
        firstViewport,
        viewportCount,
        pViewportSwizzles,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageMemoryRequirements(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements>>,
) {
    unimplemented!("vkGetImageMemoryRequirements(device, image, pMemoryRequirements")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginTransformFeedbackEXT(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: Option<NonNull<VkBuffer>>,
    pCounterBufferOffsets: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!(
        "vkCmdBeginTransformFeedbackEXT(
        commandBuffer,
        firstCounterBuffer,
        counterBufferCount,
        pCounterBuffers,
        pCounterBufferOffsets,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetExtraPrimitiveOverestimationSizeEXT(
    commandBuffer: VkCommandBuffer,
    extraPrimitiveOverestimationSize: f32,
) {
    unimplemented!("vkCmdSetExtraPrimitiveOverestimationSizeEXT(commandBuffer, extraPrimitiveOverestimationSize")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindVertexBuffers(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: Option<NonNull<VkBuffer>>,
    pOffsets: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!(
        "vkCmdBindVertexBuffers(
        commandBuffer,
        firstBinding,
        bindingCount,
        pBuffers,
        pOffsets,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushConstants(
    commandBuffer: VkCommandBuffer,
    layout: VkPipelineLayout,
    stageFlags: VkShaderStageFlags,
    offset: u32,
    size: u32,
    pValues: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!("vkCmdPushConstants(commandBuffer, layout, stageFlags, offset, size, pValues")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdDebugMarkerEndEXT(commandBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetFrontFace(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace) {
    unimplemented!("vkCmdSetFrontFace(commandBuffer, frontFace")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateViSurfaceNN(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkViSurfaceCreateInfoNN>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateViSurfaceNN(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryAndroidHardwareBufferANDROID(
    device: VkDevice,
    pInfo: Option<NonNull<VkMemoryGetAndroidHardwareBufferInfoANDROID>>,
    pBuffer: Option<NonNull<AHardwareBuffer>>,
) -> VkResult {
    unimplemented!("vkGetMemoryAndroidHardwareBufferANDROID(device, pInfo, pBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: Option<NonNull<VkDependencyInfo>>,
) {
    unimplemented!("vkCmdSetEvent2(commandBuffer, event, pDependencyInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPipelineBarrier(
    commandBuffer: VkCommandBuffer,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    dependencyFlags: VkDependencyFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: Option<NonNull<VkMemoryBarrier>>,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: Option<NonNull<VkBufferMemoryBarrier>>,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: Option<NonNull<VkImageMemoryBarrier>>,
) {
    unimplemented!(
        "vkCmdPipelineBarrier(
        commandBuffer,
        srcStageMask,
        dstStageMask,
        dependencyFlags,
        memoryBarrierCount,
        pMemoryBarriers,
        bufferMemoryBarrierCount,
        pBufferMemoryBarriers,
        imageMemoryBarrierCount,
        pImageMemoryBarriers,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyAccelerationStructureInfoKHR>>,
) -> VkResult {
    unimplemented!("vkCopyAccelerationStructureKHR(device, deferredOperation, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties2(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: Option<NonNull<u32>>,
    pQueueFamilyProperties: Option<NonNull<VkQueueFamilyProperties2>>,
) {
    unimplemented!(
        "vkGetPhysicalDeviceQueueFamilyProperties2(
        physicalDevice,
        pQueueFamilyPropertyCount,
        pQueueFamilyProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateMetalSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkMetalSurfaceCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateMetalSurfaceEXT(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksNV(
    commandBuffer: VkCommandBuffer,
    taskCount: u32,
    firstTask: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksNV(commandBuffer, taskCount, firstTask")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySamplerYcbcrConversion(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySamplerYcbcrConversion(device, ycbcrConversion, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceScreenPresentationSupportQNX(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    window: Option<NonNull<_screen_window>>,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceScreenPresentationSupportQNX(physicalDevice, queueFamilyIndex, window"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireWinrtDisplayNV(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireWinrtDisplayNV(physicalDevice, display")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDecompressMemoryIndirectCountNV(
    commandBuffer: VkCommandBuffer,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDecompressMemoryIndirectCountNV(
        commandBuffer,
        indirectCommandsAddress,
        indirectCommandsCountAddress,
        stride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindInvocationMaskHUAWEI(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    unimplemented!("vkCmdBindInvocationMaskHUAWEI(commandBuffer, imageView, imageLayout")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetShaderBinaryDataEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pDataSize: Option<NonNull<isize>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetShaderBinaryDataEXT(device, shader, pDataSize, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryFdKHR(
    device: VkDevice,
    pGetFdInfo: Option<NonNull<VkMemoryGetFdInfoKHR>>,
    pFd: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!("vkGetMemoryFdKHR(device, pGetFdInfo, pFd")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateCommandPool(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkCommandPoolCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pCommandPool: Option<NonNull<VkCommandPool>>,
) -> VkResult {
    unimplemented!("vkCreateCommandPool(device, pCreateInfo, pAllocator, pCommandPool")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSemaphoreZirconHandleFUCHSIA(
    device: VkDevice,
    pGetZirconHandleInfo: Option<NonNull<VkSemaphoreGetZirconHandleInfoFUCHSIA>>,
    pZirconHandle: Option<NonNull<zx_handle_t>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreZirconHandleFUCHSIA(device, pGetZirconHandleInfo, pZirconHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDebugUtilsMessengerEXT(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDebugUtilsMessengerEXT(instance, messenger, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetSampleLocationsEXT(
    commandBuffer: VkCommandBuffer,
    pSampleLocationsInfo: Option<NonNull<VkSampleLocationsInfoEXT>>,
) {
    unimplemented!("vkCmdSetSampleLocationsEXT(commandBuffer, pSampleLocationsInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyQueryPoolResults(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) {
    unimplemented!(
        "vkCmdCopyQueryPoolResults(
        commandBuffer,
        queryPool,
        firstQuery,
        queryCount,
        dstBuffer,
        dstOffset,
        stride,
        flags,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorEXT(
    device: VkDevice,
    pDescriptorInfo: Option<NonNull<VkDescriptorGetInfoEXT>>,
    dataSize: isize,
    pDescriptor: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!("vkGetDescriptorEXT(device, pDescriptorInfo, dataSize, pDescriptor")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDecodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pDecodeInfo: Option<NonNull<VkVideoDecodeInfoKHR>>,
) {
    unimplemented!("vkCmdDecodeVideoKHR(commandBuffer, pDecodeInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceMemoryCommitment(
    device: VkDevice,
    memory: VkDeviceMemory,
    pCommittedMemoryInBytes: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!("vkGetDeviceMemoryCommitment(device, memory, pCommittedMemoryInBytes")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSamplerYcbcrConversion(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSamplerYcbcrConversionCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pYcbcrConversion: Option<NonNull<VkSamplerYcbcrConversion>>,
) -> VkResult {
    unimplemented!(
        "vkCreateSamplerYcbcrConversion(device, pCreateInfo, pAllocator, pYcbcrConversion"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkReleaseFullScreenExclusiveModeEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkReleaseFullScreenExclusiveModeEXT(device, swapchain")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportWithCount(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: Option<NonNull<VkViewport>>,
) {
    unimplemented!("vkCmdSetViewportWithCount(commandBuffer, viewportCount, pViewports")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayPlaneCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    mode: VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: Option<NonNull<VkDisplayPlaneCapabilitiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayPlaneCapabilitiesKHR(physicalDevice, mode, planeIndex, pCapabilities"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateValidationCacheEXT(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkValidationCacheCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pValidationCache: Option<NonNull<VkValidationCacheEXT>>,
) -> VkResult {
    unimplemented!("vkCreateValidationCacheEXT(device, pCreateInfo, pAllocator, pValidationCache")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRayTracingPipelinesKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: Option<NonNull<VkRayTracingPipelineCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPipelines: Option<NonNull<VkPipeline>>,
) -> VkResult {
    unimplemented!(
        "vkCreateRayTracingPipelinesKHR(
        device,
        deferredOperation,
        pipelineCache,
        createInfoCount,
        pCreateInfos,
        pAllocator,
        pPipelines,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPreprocessGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: Option<NonNull<VkGeneratedCommandsInfoNV>>,
) {
    unimplemented!("vkCmdPreprocessGeneratedCommandsNV(commandBuffer, pGeneratedCommandsInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateAndroidSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkAndroidSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateAndroidSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageModulationModeNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationMode: VkCoverageModulationModeNV,
) {
    unimplemented!("vkCmdSetCoverageModulationModeNV(commandBuffer, coverageModulationMode")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawIndexed(
    commandBuffer: VkCommandBuffer,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
) {
    unimplemented!(
        "vkCmdDrawIndexed(
        commandBuffer,
        indexCount,
        instanceCount,
        firstIndex,
        vertexOffset,
        firstInstance,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImageToBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: Option<NonNull<VkCopyImageToBufferInfo2>>,
) {
    unimplemented!("vkCmdCopyImageToBuffer2(commandBuffer, pCopyImageToBufferInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineStippleEnableEXT(
    commandBuffer: VkCommandBuffer,
    stippledLineEnable: VkBool32,
) {
    unimplemented!("vkCmdSetLineStippleEnableEXT(commandBuffer, stippledLineEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyImage(
    device: VkDevice,
    image: VkImage,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyImage(device, image, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDescriptorSetLayout(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorSetLayoutCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSetLayout: Option<NonNull<VkDescriptorSetLayout>>,
) -> VkResult {
    unimplemented!("vkCreateDescriptorSetLayout(device, pCreateInfo, pAllocator, pSetLayout")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyCuModuleNVX(
    device: VkDevice,
    module: VkCuModuleNVX,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyCuModuleNVX(device, module, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryRemoteAddressNV(
    device: VkDevice,
    pMemoryGetRemoteAddressInfo: Option<NonNull<VkMemoryGetRemoteAddressInfoNV>>,
    pAddress: Option<NonNull<VkRemoteAddressNV>>,
) -> VkResult {
    unimplemented!("vkGetMemoryRemoteAddressNV(device, pMemoryGetRemoteAddressInfo, pAddress")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPerformanceMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: Option<NonNull<VkPerformanceMarkerInfoINTEL>>,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceMarkerINTEL(commandBuffer, pMarkerInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDispatchIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    unimplemented!("vkCmdDispatchIndirect(commandBuffer, buffer, offset")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFaultData(
    device: VkDevice,
    faultQueryBehavior: VkFaultQueryBehavior,
    pUnrecordedFaults: Option<NonNull<VkBool32>>,
    pFaultCount: Option<NonNull<u32>>,
    pFaults: Option<NonNull<VkFaultData>>,
) -> VkResult {
    unimplemented!(
        "vkGetFaultData(
        device,
        faultQueryBehavior,
        pUnrecordedFaults,
        pFaultCount,
        pFaults,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueSubmit2(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: Option<NonNull<VkSubmitInfo2>>,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueSubmit2(queue, submitCount, pSubmits, fence")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDebugMarkerInsertEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: Option<NonNull<VkDebugMarkerMarkerInfoEXT>>,
) {
    unimplemented!("vkCmdDebugMarkerInsertEXT(commandBuffer, pMarkerInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetViewportShadingRatePaletteNV(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pShadingRatePalettes: Option<NonNull<VkShadingRatePaletteNV>>,
) {
    unimplemented!(
        "vkCmdSetViewportShadingRatePaletteNV(
        commandBuffer,
        firstViewport,
        viewportCount,
        pShadingRatePalettes,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSetKHR(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: Option<NonNull<VkWriteDescriptorSet>>,
) {
    unimplemented!(
        "vkCmdPushDescriptorSetKHR(
        commandBuffer,
        pipelineBindPoint,
        layout,
        set,
        descriptorWriteCount,
        pDescriptorWrites,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32) {
    unimplemented!("vkCmdSetDeviceMask(commandBuffer, deviceMask")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorBlendEquationEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEquations: Option<NonNull<VkColorBlendEquationEXT>>,
) {
    unimplemented!(
        "vkCmdSetColorBlendEquationEXT(
        commandBuffer,
        firstAttachment,
        attachmentCount,
        pColorBlendEquations,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetRasterizationSamplesEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationSamples: VkSampleCountFlagBits,
) {
    unimplemented!("vkCmdSetRasterizationSamplesEXT(commandBuffer, rasterizationSamples")
}

#[no_mangle]
pub unsafe extern "C" fn vkBeginCommandBuffer(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: Option<NonNull<VkCommandBufferBeginInfo>>,
) -> VkResult {
    unimplemented!("vkBeginCommandBuffer(commandBuffer, pBeginInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdClearDepthStencilImage(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pDepthStencil: Option<NonNull<VkClearDepthStencilValue>>,
    rangeCount: u32,
    pRanges: Option<NonNull<VkImageSubresourceRange>>,
) {
    unimplemented!(
        "vkCmdClearDepthStencilImage(
        commandBuffer,
        image,
        imageLayout,
        pDepthStencil,
        rangeCount,
        pRanges,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetShadingRateImageEnableNV(
    commandBuffer: VkCommandBuffer,
    shadingRateImageEnable: VkBool32,
) {
    unimplemented!("vkCmdSetShadingRateImageEnableNV(commandBuffer, shadingRateImageEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSemaphoreFdKHR(
    device: VkDevice,
    pGetFdInfo: Option<NonNull<VkSemaphoreGetFdInfoKHR>>,
    pFd: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreFdKHR(device, pGetFdInfo, pFd")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDescriptorSetHostMappingVALVE(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    ppData: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!("vkGetDescriptorSetHostMappingVALVE(device, descriptorSet, ppData")
}

#[no_mangle]
pub unsafe extern "C" fn vkFreeMemory(
    device: VkDevice,
    memory: VkDeviceMemory,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkFreeMemory(device, memory, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumeratePhysicalDeviceGroups(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: Option<NonNull<u32>>,
    pPhysicalDeviceGroupProperties: Option<NonNull<VkPhysicalDeviceGroupProperties>>,
) -> VkResult {
    unimplemented!(
        "vkEnumeratePhysicalDeviceGroups(
        instance,
        pPhysicalDeviceGroupCount,
        pPhysicalDeviceGroupProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceQueue2(
    device: VkDevice,
    pQueueInfo: Option<NonNull<VkDeviceQueueInfo2>>,
    pQueue: Option<NonNull<VkQueue>>,
) {
    unimplemented!("vkGetDeviceQueue2(device, pQueueInfo, pQueue")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRenderPass2(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkRenderPassCreateInfo2>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pRenderPass: Option<NonNull<VkRenderPass>>,
) -> VkResult {
    unimplemented!("vkCreateRenderPass2(device, pCreateInfo, pAllocator, pRenderPass")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDispatch(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDispatch(commandBuffer, groupCountX, groupCountY, groupCountZ")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyAccelerationStructureNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyAccelerationStructureNV(device, accelerationStructure, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
    physicalDevice: VkPhysicalDevice,
    pCombinationCount: Option<NonNull<u32>>,
    pCombinations: Option<NonNull<VkFramebufferMixedSamplesCombinationNV>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
        physicalDevice,
        pCombinationCount,
        pCombinations,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPrimitiveTopology(
    commandBuffer: VkCommandBuffer,
    primitiveTopology: VkPrimitiveTopology,
) {
    unimplemented!("vkCmdSetPrimitiveTopology(commandBuffer, primitiveTopology")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteAccelerationStructuresPropertiesKHR(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: Option<NonNull<VkAccelerationStructureKHR>>,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!(
        "vkCmdWriteAccelerationStructuresPropertiesKHR(
        commandBuffer,
        accelerationStructureCount,
        pAccelerationStructures,
        queryType,
        queryPool,
        firstQuery,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyImageView(
    device: VkDevice,
    imageView: VkImageView,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyImageView(device, imageView, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkImportSemaphoreZirconHandleFUCHSIA(
    device: VkDevice,
    pImportSemaphoreZirconHandleInfo: Option<NonNull<VkImportSemaphoreZirconHandleInfoFUCHSIA>>,
) -> VkResult {
    unimplemented!("vkImportSemaphoreZirconHandleFUCHSIA(device, pImportSemaphoreZirconHandleInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyImageToBuffer(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: Option<NonNull<VkBufferImageCopy>>,
) {
    unimplemented!(
        "vkCmdCopyImageToBuffer(
        commandBuffer,
        srcImage,
        srcImageLayout,
        dstBuffer,
        regionCount,
        pRegions,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRayTracingShaderGroupStackSizeKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    group: u32,
    groupShader: VkShaderGroupShaderKHR,
) -> VkDeviceSize {
    unimplemented!("vkGetRayTracingShaderGroupStackSizeKHR(device, pipeline, group, groupShader")
}

#[no_mangle]
pub unsafe extern "C" fn vkResetDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    flags: VkDescriptorPoolResetFlags,
) -> VkResult {
    unimplemented!("vkResetDescriptorPool(device, descriptorPool, flags")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineCacheData(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pDataSize: Option<NonNull<isize>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetPipelineCacheData(device, pipelineCache, pDataSize, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceQueue(
    device: VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: Option<NonNull<VkQueue>>,
) {
    unimplemented!("vkGetDeviceQueue(device, queueFamilyIndex, queueIndex, pQueue")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFenceSciSyncFenceNV(
    device: VkDevice,
    pGetSciSyncHandleInfo: Option<NonNull<VkFenceGetSciSyncInfoNV>>,
    pHandle: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetFenceSciSyncFenceNV(device, pGetSciSyncHandleInfo, pHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetVertexInputEXT(
    commandBuffer: VkCommandBuffer,
    vertexBindingDescriptionCount: u32,
    pVertexBindingDescriptions: Option<NonNull<VkVertexInputBindingDescription2EXT>>,
    vertexAttributeDescriptionCount: u32,
    pVertexAttributeDescriptions: Option<NonNull<VkVertexInputAttributeDescription2EXT>>,
) {
    unimplemented!(
        "vkCmdSetVertexInputEXT(
        commandBuffer,
        vertexBindingDescriptionCount,
        pVertexBindingDescriptions,
        vertexAttributeDescriptionCount,
        pVertexAttributeDescriptions,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferCollectionPropertiesFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pProperties: Option<NonNull<VkBufferCollectionPropertiesFUCHSIA>>,
) -> VkResult {
    unimplemented!("vkGetBufferCollectionPropertiesFUCHSIA(device, collection, pProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoarseSampleOrderNV(
    commandBuffer: VkCommandBuffer,
    sampleOrderType: VkCoarseSampleOrderTypeNV,
    customSampleOrderCount: u32,
    pCustomSampleOrders: Option<NonNull<VkCoarseSampleOrderCustomNV>>,
) {
    unimplemented!(
        "vkCmdSetCoarseSampleOrderNV(
        commandBuffer,
        sampleOrderType,
        customSampleOrderCount,
        pCustomSampleOrders,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdFillBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
) {
    unimplemented!("vkCmdFillBuffer(commandBuffer, dstBuffer, dstOffset, size, data")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetBlendConstants(
    commandBuffer: VkCommandBuffer,
    blendConstants: *const f32,
) {
    let _ = unsafe { std::slice::from_raw_parts(blendConstants, 4) };
    unimplemented!("vkCmdSetBlendConstants(commandBuffer, blendConstants")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceImageFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    pImageFormatInfo: Option<NonNull<VkPhysicalDeviceImageFormatInfo2>>,
    pImageFormatProperties: Option<NonNull<VkImageFormatProperties2>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceImageFormatProperties2(
        physicalDevice,
        pImageFormatInfo,
        pImageFormatProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateEvent(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkEventCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pEvent: Option<NonNull<VkEvent>>,
) -> VkResult {
    unimplemented!("vkCreateEvent(device, pCreateInfo, pAllocator, pEvent")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDevice(
    device: VkDevice,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDevice(device, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkBindBufferMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: Option<NonNull<VkBindBufferMemoryInfo>>,
) -> VkResult {
    unimplemented!("vkBindBufferMemory2(device, bindInfoCount, pBindInfos")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureHandleNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    dataSize: isize,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!(
        "vkGetAccelerationStructureHandleNV(device, accelerationStructure, dataSize, pData"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkWaitForFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: Option<NonNull<VkFence>>,
    waitAll: VkBool32,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitForFences(device, fenceCount, pFences, waitAll, timeout")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndConditionalRenderingEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndConditionalRenderingEXT(commandBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceMultisamplePropertiesEXT(
    physicalDevice: VkPhysicalDevice,
    samples: VkSampleCountFlagBits,
    pMultisampleProperties: Option<NonNull<VkMultisamplePropertiesEXT>>,
) {
    unimplemented!("vkGetPhysicalDeviceMultisamplePropertiesEXT(physicalDevice, samples, pMultisampleProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSubresourceLayout2EXT(
    device: VkDevice,
    image: VkImage,
    pSubresource: Option<NonNull<VkImageSubresource2EXT>>,
    pLayout: Option<NonNull<VkSubresourceLayout2EXT>>,
) {
    unimplemented!("vkGetImageSubresourceLayout2EXT(device, image, pSubresource, pLayout")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyCommandPool(device, commandPool, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDisplayPlaneSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkDisplaySurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateDisplayPlaneSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMemoryIndirectNV(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdCopyMemoryIndirectNV(commandBuffer, copyBufferAddress, copyCount, stride")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineExecutableStatisticsKHR(
    device: VkDevice,
    pExecutableInfo: Option<NonNull<VkPipelineExecutableInfoKHR>>,
    pStatisticCount: Option<NonNull<u32>>,
    pStatistics: Option<NonNull<VkPipelineExecutableStatisticKHR>>,
) -> VkResult {
    unimplemented!("vkGetPipelineExecutableStatisticsKHR(device, pExecutableInfo, pStatisticCount, pStatistics")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetFragmentShadingRateKHR(
    commandBuffer: VkCommandBuffer,
    pFragmentSize: Option<NonNull<VkExtent2D>>,
    combinerOps: *const VkFragmentShadingRateCombinerOpKHR,
) {
    let _ = unsafe { std::slice::from_raw_parts(combinerOps, 2) };
    unimplemented!("vkCmdSetFragmentShadingRateKHR(commandBuffer, pFragmentSize, combinerOps")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayProperties2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayProperties2KHR(physicalDevice, pPropertyCount, pProperties"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageSubresourceLayout(
    device: VkDevice,
    image: VkImage,
    pSubresource: Option<NonNull<VkImageSubresource>>,
    pLayout: Option<NonNull<VkSubresourceLayout>>,
) {
    unimplemented!("vkGetImageSubresourceLayout(device, image, pSubresource, pLayout")
}

#[no_mangle]
pub unsafe extern "C" fn vkImportFenceSciSyncFenceNV(
    device: VkDevice,
    pImportFenceSciSyncInfo: Option<NonNull<VkImportFenceSciSyncInfoNV>>,
) -> VkResult {
    unimplemented!("vkImportFenceSciSyncFenceNV(device, pImportFenceSciSyncInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthTestEnable(
    commandBuffer: VkCommandBuffer,
    depthTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthTestEnable(commandBuffer, depthTestEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceVideoCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoProfile: Option<NonNull<VkVideoProfileInfoKHR>>,
    pCapabilities: Option<NonNull<VkVideoCapabilitiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceVideoCapabilitiesKHR(physicalDevice, pVideoProfile, pCapabilities"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDraw(
    commandBuffer: VkCommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
) {
    unimplemented!(
        "vkCmdDraw(
        commandBuffer,
        vertexCount,
        instanceCount,
        firstVertex,
        firstInstance,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateImageView(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkImageViewCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pView: Option<NonNull<VkImageView>>,
) -> VkResult {
    unimplemented!("vkCreateImageView(device, pCreateInfo, pAllocator, pView")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyInstance(
    instance: VkInstance,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyInstance(instance, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyFence(
    device: VkDevice,
    fence: VkFence,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyFence(device, fence, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPerformanceStreamMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: Option<NonNull<VkPerformanceStreamMarkerInfoINTEL>>,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceStreamMarkerINTEL(commandBuffer, pMarkerInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetAlphaToOneEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToOneEnable: VkBool32,
) {
    unimplemented!("vkCmdSetAlphaToOneEnableEXT(commandBuffer, alphaToOneEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCompileDeferredNV(
    device: VkDevice,
    pipeline: VkPipeline,
    shader: u32,
) -> VkResult {
    unimplemented!("vkCompileDeferredNV(device, pipeline, shader")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageMemoryRequirements2(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageMemoryRequirementsInfo2>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetImageMemoryRequirements2(device, pInfo, pMemoryRequirements")
}

#[no_mangle]
pub unsafe extern "C" fn vkBindBufferMemory(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult {
    unimplemented!("vkBindBufferMemory(device, buffer, memory, memoryOffset")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPatchControlPointsEXT(
    commandBuffer: VkCommandBuffer,
    patchControlPoints: u32,
) {
    unimplemented!("vkCmdSetPatchControlPointsEXT(commandBuffer, patchControlPoints")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceGroupSurfacePresentModes2EXT(
    device: VkDevice,
    pSurfaceInfo: Option<NonNull<VkPhysicalDeviceSurfaceInfo2KHR>>,
    pModes: Option<NonNull<VkDeviceGroupPresentModeFlagsKHR>>,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupSurfacePresentModes2EXT(device, pSurfaceInfo, pModes")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetSampleLocationsEnableEXT(
    commandBuffer: VkCommandBuffer,
    sampleLocationsEnable: VkBool32,
) {
    unimplemented!("vkCmdSetSampleLocationsEnableEXT(commandBuffer, sampleLocationsEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAndroidHardwareBufferPropertiesANDROID(
    device: VkDevice,
    buffer: Option<NonNull<AHardwareBuffer>>,
    pProperties: Option<NonNull<VkAndroidHardwareBufferPropertiesANDROID>>,
) -> VkResult {
    unimplemented!("vkGetAndroidHardwareBufferPropertiesANDROID(device, buffer, pProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyBufferCollectionFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyBufferCollectionFUCHSIA(device, collection, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMultiIndexedEXT(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pIndexInfo: Option<NonNull<VkMultiDrawIndexedInfoEXT>>,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
    pVertexOffset: Option<NonNull<i32>>,
) {
    unimplemented!(
        "vkCmdDrawMultiIndexedEXT(
        commandBuffer,
        drawCount,
        pIndexInfo,
        instanceCount,
        firstInstance,
        stride,
        pVertexOffset,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorWriteEnableEXT(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pColorWriteEnables: Option<NonNull<VkBool32>>,
) {
    unimplemented!("vkCmdSetColorWriteEnableEXT(commandBuffer, attachmentCount, pColorWriteEnables")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetGeneratedCommandsMemoryRequirementsNV(
    device: VkDevice,
    pInfo: Option<NonNull<VkGeneratedCommandsMemoryRequirementsInfoNV>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetGeneratedCommandsMemoryRequirementsNV(device, pInfo, pMemoryRequirements")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceRefreshableObjectTypesKHR(
    physicalDevice: VkPhysicalDevice,
    pRefreshableObjectTypeCount: Option<NonNull<u32>>,
    pRefreshableObjectTypes: Option<NonNull<VkObjectType>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceRefreshableObjectTypesKHR(
        physicalDevice,
        pRefreshableObjectTypeCount,
        pRefreshableObjectTypes,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectEXT(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksIndirectEXT(commandBuffer, buffer, offset, drawCount, stride")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeferredOperationResultKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkGetDeferredOperationResultKHR(device, operation")
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueInsertDebugUtilsLabelEXT(
    queue: VkQueue,
    pLabelInfo: Option<NonNull<VkDebugUtilsLabelEXT>>,
) {
    unimplemented!("vkQueueInsertDebugUtilsLabelEXT(queue, pLabelInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkBufferCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetBufferOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndDebugUtilsLabelEXT(commandBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndRendering(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndRendering(commandBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireNextImageKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    pImageIndex: Option<NonNull<u32>>,
) -> VkResult {
    unimplemented!(
        "vkAcquireNextImageKHR(device, swapchain, timeout, semaphore, fence, pImageIndex"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPerformanceParameterINTEL(
    device: VkDevice,
    parameter: VkPerformanceParameterTypeINTEL,
    pValue: Option<NonNull<VkPerformanceValueINTEL>>,
) -> VkResult {
    unimplemented!("vkGetPerformanceParameterINTEL(device, parameter, pValue")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetShaderInfoAMD(
    device: VkDevice,
    pipeline: VkPipeline,
    shaderStage: VkShaderStageFlagBits,
    infoType: VkShaderInfoTypeAMD,
    pInfoSize: Option<NonNull<isize>>,
    pInfo: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetShaderInfoAMD(device, pipeline, shaderStage, infoType, pInfoSize, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkWriteAccelerationStructuresPropertiesKHR(
    device: VkDevice,
    accelerationStructureCount: u32,
    pAccelerationStructures: Option<NonNull<VkAccelerationStructureKHR>>,
    queryType: VkQueryType,
    dataSize: isize,
    pData: Option<NonNull<std::ffi::c_void>>,
    stride: isize,
) -> VkResult {
    unimplemented!(
        "vkWriteAccelerationStructuresPropertiesKHR(
        device,
        accelerationStructureCount,
        pAccelerationStructures,
        queryType,
        dataSize,
        pData,
        stride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireNextImage2KHR(
    device: VkDevice,
    pAcquireInfo: Option<NonNull<VkAcquireNextImageInfoKHR>>,
    pImageIndex: Option<NonNull<u32>>,
) -> VkResult {
    unimplemented!("vkAcquireNextImage2KHR(device, pAcquireInfo, pImageIndex")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorBlendAdvancedEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendAdvanced: Option<NonNull<VkColorBlendAdvancedEXT>>,
) {
    unimplemented!(
        "vkCmdSetColorBlendAdvancedEXT(
        commandBuffer,
        firstAttachment,
        attachmentCount,
        pColorBlendAdvanced,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainGrallocUsage2ANDROID(
    device: VkDevice,
    format: VkFormat,
    imageUsage: VkImageUsageFlags,
    swapchainImageUsage: VkSwapchainImageUsageFlagsANDROID,
    grallocConsumerUsage: Option<NonNull<u64>>,
    grallocProducerUsage: Option<NonNull<u64>>,
) -> VkResult {
    unimplemented!(
        "vkGetSwapchainGrallocUsage2ANDROID(
        device,
        format,
        imageUsage,
        swapchainImageUsage,
        grallocConsumerUsage,
        grallocProducerUsage,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32) {
    unimplemented!("vkCmdSetLineWidth(commandBuffer, lineWidth")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDebugMarkerBeginEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: Option<NonNull<VkDebugMarkerMarkerInfoEXT>>,
) {
    unimplemented!("vkCmdDebugMarkerBeginEXT(commandBuffer, pMarkerInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalBufferProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalBufferInfo: Option<NonNull<VkPhysicalDeviceExternalBufferInfo>>,
    pExternalBufferProperties: Option<NonNull<VkExternalBufferProperties>>,
) {
    unimplemented!(
        "vkGetPhysicalDeviceExternalBufferProperties(
        physicalDevice,
        pExternalBufferInfo,
        pExternalBufferProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetExclusiveScissorNV(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissors: Option<NonNull<VkRect2D>>,
) {
    unimplemented!(
        "vkCmdSetExclusiveScissorNV(
        commandBuffer,
        firstExclusiveScissor,
        exclusiveScissorCount,
        pExclusiveScissors,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCullMode(
    commandBuffer: VkCommandBuffer,
    cullMode: VkCullModeFlags,
) {
    unimplemented!("vkCmdSetCullMode(commandBuffer, cullMode")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorBlendEnableEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEnables: Option<NonNull<VkBool32>>,
) {
    unimplemented!(
        "vkCmdSetColorBlendEnableEXT(
        commandBuffer,
        firstAttachment,
        attachmentCount,
        pColorBlendEnables,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetScissor(
    commandBuffer: VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: Option<NonNull<VkRect2D>>,
) {
    unimplemented!("vkCmdSetScissor(commandBuffer, firstScissor, scissorCount, pScissors")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayModeProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayModeProperties2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayModeProperties2KHR(physicalDevice, display, pPropertyCount, pProperties"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdPushDescriptorSetWithTemplateKHR(
    commandBuffer: VkCommandBuffer,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!(
        "vkCmdPushDescriptorSetWithTemplateKHR(
        commandBuffer,
        descriptorUpdateTemplate,
        layout,
        set,
        pData,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureDeviceAddressKHR(
    device: VkDevice,
    pInfo: Option<NonNull<VkAccelerationStructureDeviceAddressInfoKHR>>,
) -> VkDeviceAddress {
    unimplemented!("vkGetAccelerationStructureDeviceAddressKHR(device, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFeatures(
    physicalDevice: VkPhysicalDevice,
    pFeatures: Option<NonNull<VkPhysicalDeviceFeatures>>,
) {
    unimplemented!("vkGetPhysicalDeviceFeatures(physicalDevice, pFeatures")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetColorWriteMaskEXT(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorWriteMasks: Option<NonNull<VkColorComponentFlags>>,
) {
    unimplemented!(
        "vkCmdSetColorWriteMaskEXT(
        commandBuffer,
        firstAttachment,
        attachmentCount,
        pColorWriteMasks,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateBufferCollectionFUCHSIA(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferCollectionCreateInfoFUCHSIA>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pCollection: Option<NonNull<VkBufferCollectionFUCHSIA>>,
) -> VkResult {
    unimplemented!("vkCreateBufferCollectionFUCHSIA(device, pCreateInfo, pAllocator, pCollection")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyAccelerationStructureKHR(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyAccelerationStructureKHR(device, accelerationStructure, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: Option<NonNull<VkVideoBeginCodingInfoKHR>>,
) {
    unimplemented!("vkCmdBeginVideoCodingKHR(commandBuffer, pBeginInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateAccelerationStructureKHR(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkAccelerationStructureCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pAccelerationStructure: Option<NonNull<VkAccelerationStructureKHR>>,
) -> VkResult {
    unimplemented!(
        "vkCreateAccelerationStructureKHR(device, pCreateInfo, pAllocator, pAccelerationStructure"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: Option<NonNull<u32>>,
    pQueueFamilyProperties: Option<NonNull<VkQueueFamilyProperties>>,
) {
    unimplemented!(
        "vkGetPhysicalDeviceQueueFamilyProperties(
        physicalDevice,
        pQueueFamilyPropertyCount,
        pQueueFamilyProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBuildMicromapsEXT(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: Option<NonNull<VkMicromapBuildInfoEXT>>,
) {
    unimplemented!("vkCmdBuildMicromapsEXT(commandBuffer, infoCount, pInfos")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceFormatsKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceFormatCount: Option<NonNull<u32>>,
    pSurfaceFormats: Option<NonNull<VkSurfaceFormatKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSurfaceFormatsKHR(
        physicalDevice,
        surface,
        pSurfaceFormatCount,
        pSurfaceFormats,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFenceSciSyncObjNV(
    device: VkDevice,
    pGetSciSyncHandleInfo: Option<NonNull<VkFenceGetSciSyncInfoNV>>,
    pHandle: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetFenceSciSyncObjNV(device, pGetSciSyncHandleInfo, pHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) {
    unimplemented!("vkUnmapMemory(device, memory")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilCompareMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    compareMask: u32,
) {
    unimplemented!("vkCmdSetStencilCompareMask(commandBuffer, faceMask, compareMask")
}

#[no_mangle]
pub unsafe extern "C" fn vkBindOpticalFlowSessionImageNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    bindingPoint: VkOpticalFlowSessionBindingPointNV,
    view: VkImageView,
    layout: VkImageLayout,
) -> VkResult {
    unimplemented!("vkBindOpticalFlowSessionImageNV(device, session, bindingPoint, view, layout")
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateDescriptorSetWithTemplate(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!(
        "vkUpdateDescriptorSetWithTemplate(device, descriptorSet, descriptorUpdateTemplate, pData"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRenderAreaGranularity(
    device: VkDevice,
    renderPass: VkRenderPass,
    pGranularity: Option<NonNull<VkExtent2D>>,
) {
    unimplemented!("vkGetRenderAreaGranularity(device, renderPass, pGranularity")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDiscardRectangleEXT(
    commandBuffer: VkCommandBuffer,
    firstDiscardRectangle: u32,
    discardRectangleCount: u32,
    pDiscardRectangles: Option<NonNull<VkRect2D>>,
) {
    unimplemented!(
        "vkCmdSetDiscardRectangleEXT(
        commandBuffer,
        firstDiscardRectangle,
        discardRectangleCount,
        pDiscardRectangles,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectCountEXT(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDrawMeshTasksIndirectCountEXT(
        commandBuffer,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfacePresentModes2EXT(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: Option<NonNull<VkPhysicalDeviceSurfaceInfo2KHR>>,
    pPresentModeCount: Option<NonNull<u32>>,
    pPresentModes: Option<NonNull<VkPresentModeKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSurfacePresentModes2EXT(
        physicalDevice,
        pSurfaceInfo,
        pPresentModeCount,
        pPresentModes,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceImageSparseMemoryRequirements(
    device: VkDevice,
    pInfo: Option<NonNull<VkDeviceImageMemoryRequirements>>,
    pSparseMemoryRequirementCount: Option<NonNull<u32>>,
    pSparseMemoryRequirements: Option<NonNull<VkSparseImageMemoryRequirements2>>,
) {
    unimplemented!(
        "vkGetDeviceImageSparseMemoryRequirements(
        device,
        pInfo,
        pSparseMemoryRequirementCount,
        pSparseMemoryRequirements,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWaitEvents2(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: Option<NonNull<VkEvent>>,
    pDependencyInfos: Option<NonNull<VkDependencyInfo>>,
) {
    unimplemented!("vkCmdWaitEvents2(commandBuffer, eventCount, pEvents, pDependencyInfos")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMultiEXT(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pVertexInfo: Option<NonNull<VkMultiDrawInfoEXT>>,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDrawMultiEXT(
        commandBuffer,
        drawCount,
        pVertexInfo,
        instanceCount,
        firstInstance,
        stride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceMemoryOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: Option<NonNull<VkDeviceMemoryOpaqueCaptureAddressInfo>>,
) -> u64 {
    unimplemented!("vkGetDeviceMemoryOpaqueCaptureAddress(device, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBiasEnable(
    commandBuffer: VkCommandBuffer,
    depthBiasEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthBiasEnable(commandBuffer, depthBiasEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetBufferCollectionImageConstraintsFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pImageConstraintsInfo: Option<NonNull<VkImageConstraintsInfoFUCHSIA>>,
) -> VkResult {
    unimplemented!(
        "vkSetBufferCollectionImageConstraintsFUCHSIA(device, collection, pImageConstraintsInfo"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetImageViewAddressNVX(
    device: VkDevice,
    imageView: VkImageView,
    pProperties: Option<NonNull<VkImageViewAddressPropertiesNVX>>,
) -> VkResult {
    unimplemented!("vkGetImageViewAddressNVX(device, imageView, pProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthClipEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthClipEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClipEnableEXT(commandBuffer, depthClipEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateXcbSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkXcbSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateXcbSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetLogicOpEnableEXT(
    commandBuffer: VkCommandBuffer,
    logicOpEnable: VkBool32,
) {
    unimplemented!("vkCmdSetLogicOpEnableEXT(commandBuffer, logicOpEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDepthBounds(
    commandBuffer: VkCommandBuffer,
    minDepthBounds: f32,
    maxDepthBounds: f32,
) {
    unimplemented!("vkCmdSetDepthBounds(commandBuffer, minDepthBounds, maxDepthBounds")
}

#[no_mangle]
pub unsafe extern "C" fn vkMapMemory2KHR(
    device: VkDevice,
    pMemoryMapInfo: Option<NonNull<VkMemoryMapInfoKHR>>,
    ppData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkMapMemory2KHR(device, pMemoryMapInfo, ppData")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndRenderPass2(
    commandBuffer: VkCommandBuffer,
    pSubpassEndInfo: Option<NonNull<VkSubpassEndInfo>>,
) {
    unimplemented!("vkCmdEndRenderPass2(commandBuffer, pSubpassEndInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdClearAttachments(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pAttachments: Option<NonNull<VkClearAttachment>>,
    rectCount: u32,
    pRects: Option<NonNull<VkClearRect>>,
) {
    unimplemented!(
        "vkCmdClearAttachments(
        commandBuffer,
        attachmentCount,
        pAttachments,
        rectCount,
        pRects,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetFragmentShadingRateEnumNV(
    commandBuffer: VkCommandBuffer,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: *const VkFragmentShadingRateCombinerOpKHR,
) {
    let _ = unsafe { std::slice::from_raw_parts(combinerOps, 2) };
    unimplemented!("vkCmdSetFragmentShadingRateEnumNV(commandBuffer, shadingRate, combinerOps")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteMicromapsPropertiesEXT(
    commandBuffer: VkCommandBuffer,
    micromapCount: u32,
    pMicromaps: Option<NonNull<VkMicromapEXT>>,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
) {
    unimplemented!(
        "vkCmdWriteMicromapsPropertiesEXT(
        commandBuffer,
        micromapCount,
        pMicromaps,
        queryType,
        queryPool,
        firstQuery,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSciSyncAttributesNV(
    physicalDevice: VkPhysicalDevice,
    pSciSyncAttributesInfo: Option<NonNull<VkSciSyncAttributesInfoNV>>,
    pAttributes: NvSciSyncAttrList,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSciSyncAttributesNV(physicalDevice, pSciSyncAttributesInfo, pAttributes")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetShaderModuleIdentifierEXT(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pIdentifier: Option<NonNull<VkShaderModuleIdentifierEXT>>,
) {
    unimplemented!("vkGetShaderModuleIdentifierEXT(device, shaderModule, pIdentifier")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindShadersEXT(
    commandBuffer: VkCommandBuffer,
    stageCount: u32,
    pStages: Option<NonNull<VkShaderStageFlagBits>>,
    pShaders: Option<NonNull<VkShaderEXT>>,
) {
    unimplemented!("vkCmdBindShadersEXT(commandBuffer, stageCount, pStages, pShaders")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDeferredOperationKHR(
    device: VkDevice,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDeferredOperation: Option<NonNull<VkDeferredOperationKHR>>,
) -> VkResult {
    unimplemented!("vkCreateDeferredOperationKHR(device, pAllocator, pDeferredOperation")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdNextSubpass2(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: Option<NonNull<VkSubpassBeginInfo>>,
    pSubpassEndInfo: Option<NonNull<VkSubpassEndInfo>>,
) {
    unimplemented!("vkCmdNextSubpass2(commandBuffer, pSubpassBeginInfo, pSubpassEndInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkDeferredOperationJoinKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkDeferredOperationJoinKHR(device, operation")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetCalibratedTimestampsEXT(
    device: VkDevice,
    timestampCount: u32,
    pTimestampInfos: Option<NonNull<VkCalibratedTimestampInfoEXT>>,
    pTimestamps: Option<NonNull<u64>>,
    pMaxDeviation: Option<NonNull<u64>>,
) -> VkResult {
    unimplemented!(
        "vkGetCalibratedTimestampsEXT(
        device,
        timestampCount,
        pTimestampInfos,
        pTimestamps,
        pMaxDeviation,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkUnmapMemory2KHR(
    device: VkDevice,
    pMemoryUnmapInfo: Option<NonNull<VkMemoryUnmapInfoKHR>>,
) -> VkResult {
    unimplemented!("vkUnmapMemory2KHR(device, pMemoryUnmapInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindShadingRateImageNV(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    unimplemented!("vkCmdBindShadingRateImageNV(commandBuffer, imageView, imageLayout")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
    index: u32,
) {
    unimplemented!("vkCmdBeginQueryIndexedEXT(commandBuffer, queryPool, query, flags, index")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyDebugReportCallbackEXT(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDebugReportCallbackEXT(instance, callback, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySemaphore(
    device: VkDevice,
    semaphore: VkSemaphore,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySemaphore(device, semaphore, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDiscardRectangleModeEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleMode: VkDiscardRectangleModeEXT,
) {
    unimplemented!("vkCmdSetDiscardRectangleModeEXT(commandBuffer, discardRectangleMode")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResetQueryPool(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    unimplemented!("vkCmdResetQueryPool(commandBuffer, queryPool, firstQuery, queryCount")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateBufferView(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferViewCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pView: Option<NonNull<VkBufferView>>,
) -> VkResult {
    unimplemented!("vkCreateBufferView(device, pCreateInfo, pAllocator, pView")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetConservativeRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
) {
    unimplemented!(
        "vkCmdSetConservativeRasterizationModeEXT(commandBuffer, conservativeRasterizationMode"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyMemoryToMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyMemoryToMicromapInfoEXT>>,
) -> VkResult {
    unimplemented!("vkCopyMemoryToMicromapEXT(device, deferredOperation, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceBufferMemoryRequirements(
    device: VkDevice,
    pInfo: Option<NonNull<VkDeviceBufferMemoryRequirements>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetDeviceBufferMemoryRequirements(device, pInfo, pMemoryRequirements")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdTraceRaysIndirectKHR(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: Option<NonNull<VkStridedDeviceAddressRegionKHR>>,
    pMissShaderBindingTable: Option<NonNull<VkStridedDeviceAddressRegionKHR>>,
    pHitShaderBindingTable: Option<NonNull<VkStridedDeviceAddressRegionKHR>>,
    pCallableShaderBindingTable: Option<NonNull<VkStridedDeviceAddressRegionKHR>>,
    indirectDeviceAddress: VkDeviceAddress,
) {
    unimplemented!(
        "vkCmdTraceRaysIndirectKHR(
        commandBuffer,
        pRaygenShaderBindingTable,
        pMissShaderBindingTable,
        pHitShaderBindingTable,
        pCallableShaderBindingTable,
        indirectDeviceAddress,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySurfaceKHR(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySurfaceKHR(instance, surface, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: Option<NonNull<VkDebugUtilsLabelEXT>>,
) {
    unimplemented!("vkCmdBeginDebugUtilsLabelEXT(commandBuffer, pLabelInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyMemoryToAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyMemoryToAccelerationStructureInfoKHR>>,
) {
    unimplemented!("vkCmdCopyMemoryToAccelerationStructureKHR(commandBuffer, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkImportSemaphoreFdKHR(
    device: VkDevice,
    pImportSemaphoreFdInfo: Option<NonNull<VkImportSemaphoreFdInfoKHR>>,
) -> VkResult {
    unimplemented!("vkImportSemaphoreFdKHR(device, pImportSemaphoreFdInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueBindSparse(
    queue: VkQueue,
    bindInfoCount: u32,
    pBindInfo: Option<NonNull<VkBindSparseInfo>>,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueBindSparse(queue, bindInfoCount, pBindInfo, fence")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryWin32HandleNV(
    device: VkDevice,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    pHandle: Option<NonNull<HANDLE>>,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandleNV(device, memory, handleType, pHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyVideoSessionParametersKHR(device, videoSessionParameters, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    unimplemented!("vkCmdResetEvent(commandBuffer, event, stageMask")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectNV(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksIndirectNV(commandBuffer, buffer, offset, drawCount, stride")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryHostPointerPropertiesEXT(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHostPointer: Option<NonNull<std::ffi::c_void>>,
    pMemoryHostPointerProperties: Option<NonNull<VkMemoryHostPointerPropertiesEXT>>,
) -> VkResult {
    unimplemented!(
        "vkGetMemoryHostPointerPropertiesEXT(
        device,
        handleType,
        pHostPointer,
        pMemoryHostPointerProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteTimestamp(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdWriteTimestamp(commandBuffer, pipelineStage, queryPool, query")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateAccelerationStructureNV(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkAccelerationStructureCreateInfoNV>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pAccelerationStructure: Option<NonNull<VkAccelerationStructureNV>>,
) -> VkResult {
    unimplemented!(
        "vkCreateAccelerationStructureNV(device, pCreateInfo, pAllocator, pAccelerationStructure"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryFdPropertiesKHR(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: int,
    pMemoryFdProperties: Option<NonNull<VkMemoryFdPropertiesKHR>>,
) -> VkResult {
    unimplemented!("vkGetMemoryFdPropertiesKHR(device, handleType, fd, pMemoryFdProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetAlphaToCoverageEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToCoverageEnable: VkBool32,
) {
    unimplemented!("vkCmdSetAlphaToCoverageEnableEXT(commandBuffer, alphaToCoverageEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectCountNV(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
) {
    unimplemented!(
        "vkCmdDrawMeshTasksIndirectCountNV(
        commandBuffer,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: Option<NonNull<VkBool32>>,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice, queueFamilyIndex, surface, pSupported")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyValidationCacheEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyValidationCacheEXT(device, validationCache, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdResetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
) {
    unimplemented!("vkCmdResetEvent2(commandBuffer, event, stageMask")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceFormats2KHR(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: Option<NonNull<VkPhysicalDeviceSurfaceInfo2KHR>>,
    pSurfaceFormatCount: Option<NonNull<u32>>,
    pSurfaceFormats: Option<NonNull<VkSurfaceFormat2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSurfaceFormats2KHR(
        physicalDevice,
        pSurfaceInfo,
        pSurfaceFormatCount,
        pSurfaceFormats,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    dfb: Option<NonNull<IDirectFB>>,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceDirectFBPresentationSupportEXT(physicalDevice, queueFamilyIndex, dfb"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateBuffer(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pBuffer: Option<NonNull<VkBuffer>>,
) -> VkResult {
    unimplemented!("vkCreateBuffer(device, pCreateInfo, pAllocator, pBuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetCommandPoolMemoryConsumption(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBuffer: VkCommandBuffer,
    pConsumption: Option<NonNull<VkCommandPoolMemoryConsumption>>,
) {
    unimplemented!(
        "vkGetCommandPoolMemoryConsumption(device, commandPool, commandBuffer, pConsumption"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBuildAccelerationStructuresKHR(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: Option<NonNull<VkAccelerationStructureBuildGeometryInfoKHR>>,
    ppBuildRangeInfos: Option<NonNull<VkAccelerationStructureBuildRangeInfoKHR>>,
) {
    unimplemented!(
        "vkCmdBuildAccelerationStructuresKHR(commandBuffer, infoCount, pInfos, ppBuildRangeInfos"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: Option<NonNull<VkSurfaceCapabilitiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice, surface, pSurfaceCapabilities"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalFenceProperties(
    physicalDevice: VkPhysicalDevice,
    pExternalFenceInfo: Option<NonNull<VkPhysicalDeviceExternalFenceInfo>>,
    pExternalFenceProperties: Option<NonNull<VkExternalFenceProperties>>,
) {
    unimplemented!(
        "vkGetPhysicalDeviceExternalFenceProperties(
        physicalDevice,
        pExternalFenceInfo,
        pExternalFenceProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetCoverageToColorEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorEnable: VkBool32,
) {
    unimplemented!("vkCmdSetCoverageToColorEnableNV(commandBuffer, coverageToColorEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroySwapchainKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySwapchainKHR(device, swapchain, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceGroupPeerMemoryFeatures(
    device: VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: Option<NonNull<VkPeerMemoryFeatureFlags>>,
) {
    unimplemented!(
        "vkGetDeviceGroupPeerMemoryFeatures(
        device,
        heapIndex,
        localDeviceIndex,
        remoteDeviceIndex,
        pPeerMemoryFeatures,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCopyAccelerationStructureToMemoryKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyAccelerationStructureToMemoryInfoKHR>>,
) -> VkResult {
    unimplemented!("vkCopyAccelerationStructureToMemoryKHR(device, deferredOperation, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateSemaphore(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSemaphoreCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSemaphore: Option<NonNull<VkSemaphore>>,
) -> VkResult {
    unimplemented!("vkCreateSemaphore(device, pCreateInfo, pAllocator, pSemaphore")
}

#[no_mangle]
pub unsafe extern "C" fn vkWaitSemaphores(
    device: VkDevice,
    pWaitInfo: Option<NonNull<VkSemaphoreWaitInfo>>,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitSemaphores(device, pWaitInfo, timeout")
}

#[no_mangle]
pub unsafe extern "C" fn vkInvalidateMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: Option<NonNull<VkMappedMemoryRange>>,
) -> VkResult {
    unimplemented!("vkInvalidateMappedMemoryRanges(device, memoryRangeCount, pMemoryRanges")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetBufferMemoryRequirements(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements>>,
) {
    unimplemented!("vkGetBufferMemoryRequirements(device, buffer, pMemoryRequirements")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSemaphoreSciSyncObjNV(
    device: VkDevice,
    pGetSciSyncInfo: Option<NonNull<VkSemaphoreGetSciSyncInfoNV>>,
    pHandle: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreSciSyncObjNV(device, pGetSciSyncInfo, pHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDebugUtilsMessengerEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkDebugUtilsMessengerCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMessenger: Option<NonNull<VkDebugUtilsMessengerEXT>>,
) -> VkResult {
    unimplemented!("vkCreateDebugUtilsMessengerEXT(instance, pCreateInfo, pAllocator, pMessenger")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDevicePresentRectanglesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pRectCount: Option<NonNull<u32>>,
    pRects: Option<NonNull<VkRect2D>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDevicePresentRectanglesKHR(physicalDevice, surface, pRectCount, pRects"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateCuModuleNVX(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkCuModuleCreateInfoNVX>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pModule: Option<NonNull<VkCuModuleNVX>>,
) -> VkResult {
    unimplemented!("vkCreateCuModuleNVX(device, pCreateInfo, pAllocator, pModule")
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireDrmDisplayEXT(physicalDevice, drmFd, display")
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyFramebuffer(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyFramebuffer(device, framebuffer, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkDeviceWaitIdle(device: VkDevice) -> VkResult {
    unimplemented!("vkDeviceWaitIdle(device")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetShaderModuleCreateInfoIdentifierEXT(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkShaderModuleCreateInfo>>,
    pIdentifier: Option<NonNull<VkShaderModuleIdentifierEXT>>,
) {
    unimplemented!("vkGetShaderModuleCreateInfoIdentifierEXT(device, pCreateInfo, pIdentifier")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateVideoSessionParametersKHR(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkVideoSessionParametersCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pVideoSessionParameters: Option<NonNull<VkVideoSessionParametersKHR>>,
) -> VkResult {
    unimplemented!("vkCreateVideoSessionParametersKHR(device, pCreateInfo, pAllocator, pVideoSessionParameters")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPipelineExecutablePropertiesKHR(
    device: VkDevice,
    pPipelineInfo: Option<NonNull<VkPipelineInfoKHR>>,
    pExecutableCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkPipelineExecutablePropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPipelineExecutablePropertiesKHR(device, pPipelineInfo, pExecutableCount, pProperties"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBuildAccelerationStructuresIndirectKHR(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: Option<NonNull<VkAccelerationStructureBuildGeometryInfoKHR>>,
    pIndirectDeviceAddresses: Option<NonNull<VkDeviceAddress>>,
    pIndirectStrides: Option<NonNull<u32>>,
    ppMaxPrimitiveCounts: Option<NonNull<u32>>,
) {
    unimplemented!(
        "vkCmdBuildAccelerationStructuresIndirectKHR(
        commandBuffer,
        infoCount,
        pInfos,
        pIndirectDeviceAddresses,
        pIndirectStrides,
        ppMaxPrimitiveCounts,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdWriteBufferMarkerAMD(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
) {
    unimplemented!(
        "vkCmdWriteBufferMarkerAMD(commandBuffer, pipelineStage, dstBuffer, dstOffset, marker"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
    physicalDevice: VkPhysicalDevice,
    pPerformanceQueryCreateInfo: Option<NonNull<VkQueryPoolPerformanceCreateInfoKHR>>,
    pNumPasses: Option<NonNull<u32>>,
) {
    unimplemented!(
        "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
        physicalDevice,
        pPerformanceQueryCreateInfo,
        pNumPasses,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceImageFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    pImageFormatProperties: Option<NonNull<VkImageFormatProperties>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceImageFormatProperties(
        physicalDevice,
        format,
        type_,
        tiling,
        usage,
        flags,
        pImageFormatProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateVideoSessionKHR(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkVideoSessionCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pVideoSession: Option<NonNull<VkVideoSessionKHR>>,
) -> VkResult {
    unimplemented!("vkCreateVideoSessionKHR(device, pCreateInfo, pAllocator, pVideoSession")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
    pExternalImageFormatProperties: Option<NonNull<VkExternalImageFormatPropertiesNV>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
        physicalDevice,
        format,
        type_,
        tiling,
        usage,
        flags,
        externalHandleType,
        pExternalImageFormatProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkWaitForPresentKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    presentId: u64,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitForPresentKHR(device, swapchain, presentId, timeout")
}

#[no_mangle]
pub unsafe extern "C" fn vkResetCommandBuffer(
    commandBuffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlags,
) -> VkResult {
    unimplemented!("vkResetCommandBuffer(commandBuffer, flags")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetDeviceMemoryPriorityEXT(
    device: VkDevice,
    memory: VkDeviceMemory,
    priority: f32,
) {
    unimplemented!("vkSetDeviceMemoryPriorityEXT(device, memory, priority")
}

#[no_mangle]
pub unsafe extern "C" fn vkAcquireProfilingLockKHR(
    device: VkDevice,
    pInfo: Option<NonNull<VkAcquireProfilingLockInfoKHR>>,
) -> VkResult {
    unimplemented!("vkAcquireProfilingLockKHR(device, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMicromapBuildSizesEXT(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: Option<NonNull<VkMicromapBuildInfoEXT>>,
    pSizeInfo: Option<NonNull<VkMicromapBuildSizesInfoEXT>>,
) {
    unimplemented!("vkGetMicromapBuildSizesEXT(device, buildType, pBuildInfo, pSizeInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetQueryPoolResults(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dataSize: isize,
    pData: Option<NonNull<std::ffi::c_void>>,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) -> VkResult {
    unimplemented!(
        "vkGetQueryPoolResults(
        device, queryPool, firstQuery, queryCount, dataSize, pData, stride, flags,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateRenderPass(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkRenderPassCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pRenderPass: Option<NonNull<VkRenderPass>>,
) -> VkResult {
    unimplemented!("vkCreateRenderPass(device, pCreateInfo, pAllocator, pRenderPass")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceFaultInfoEXT(
    device: VkDevice,
    pFaultCounts: Option<NonNull<VkDeviceFaultCountsEXT>>,
    pFaultInfo: Option<NonNull<VkDeviceFaultInfoEXT>>,
) -> VkResult {
    unimplemented!("vkGetDeviceFaultInfoEXT(device, pFaultCounts, pFaultInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryZirconHandlePropertiesFUCHSIA(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    zirconHandle: zx_handle_t,
    pMemoryZirconHandleProperties: Option<NonNull<VkMemoryZirconHandlePropertiesFUCHSIA>>,
) -> VkResult {
    unimplemented!(
        "vkGetMemoryZirconHandlePropertiesFUCHSIA(
        device,
        handleType,
        zirconHandle,
        pMemoryZirconHandleProperties,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdTraceRaysKHR(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: Option<NonNull<VkStridedDeviceAddressRegionKHR>>,
    pMissShaderBindingTable: Option<NonNull<VkStridedDeviceAddressRegionKHR>>,
    pHitShaderBindingTable: Option<NonNull<VkStridedDeviceAddressRegionKHR>>,
    pCallableShaderBindingTable: Option<NonNull<VkStridedDeviceAddressRegionKHR>>,
    width: u32,
    height: u32,
    depth: u32,
) {
    unimplemented!(
        "vkCmdTraceRaysKHR(
        commandBuffer,
        pRaygenShaderBindingTable,
        pMissShaderBindingTable,
        pHitShaderBindingTable,
        pCallableShaderBindingTable,
        width,
        height,
        depth,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDisplayPlaneSupportedDisplaysKHR(
    physicalDevice: VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: Option<NonNull<u32>>,
    pDisplays: Option<NonNull<VkDisplayKHR>>,
) -> VkResult {
    unimplemented!("vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice, planeIndex, pDisplayCount, pDisplays")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdUpdateBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    dataSize: VkDeviceSize,
    pData: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!("vkCmdUpdateBuffer(commandBuffer, dstBuffer, dstOffset, dataSize, pData")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    unimplemented!("vkCmdSetEvent(commandBuffer, event, stageMask")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetTessellationDomainOriginEXT(
    commandBuffer: VkCommandBuffer,
    domainOrigin: VkTessellationDomainOrigin,
) {
    unimplemented!("vkCmdSetTessellationDomainOriginEXT(commandBuffer, domainOrigin")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdEndQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdEndQuery(commandBuffer, queryPool, query")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayPropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice, pPropertyCount, pProperties"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceFragmentShadingRatesKHR(
    physicalDevice: VkPhysicalDevice,
    pFragmentShadingRateCount: Option<NonNull<u32>>,
    pFragmentShadingRates: Option<NonNull<VkPhysicalDeviceFragmentShadingRateKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceFragmentShadingRatesKHR(
        physicalDevice,
        pFragmentShadingRateCount,
        pFragmentShadingRates,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyPrivateDataSlot(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyPrivateDataSlot(device, privateDataSlot, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceProcAddr(
    device: VkDevice,
    pName: Option<NonNull<std::ffi::c_char>>,
) -> PFN_vkVoidFunction {
    unimplemented!("vkGetDeviceProcAddr(device, pName")
}

#[no_mangle]
pub unsafe extern "C" fn vkTrimCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
) {
    unimplemented!("vkTrimCommandPool(device, commandPool, flags")
}

#[no_mangle]
pub unsafe extern "C" fn vkReleaseDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkReleaseDisplayEXT(physicalDevice, display")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDescriptorUpdateTemplate(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorUpdateTemplateCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDescriptorUpdateTemplate: Option<NonNull<VkDescriptorUpdateTemplate>>,
) -> VkResult {
    unimplemented!("vkCreateDescriptorUpdateTemplate(device, pCreateInfo, pAllocator, pDescriptorUpdateTemplate")
}

#[no_mangle]
pub unsafe extern "C" fn vkQueueSetPerformanceConfigurationINTEL(
    queue: VkQueue,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!("vkQueueSetPerformanceConfigurationINTEL(queue, configuration")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPrimitiveRestartEnable(
    commandBuffer: VkCommandBuffer,
    primitiveRestartEnable: VkBool32,
) {
    unimplemented!("vkCmdSetPrimitiveRestartEnable(commandBuffer, primitiveRestartEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetAttachmentFeedbackLoopEnableEXT(
    commandBuffer: VkCommandBuffer,
    aspectMask: VkImageAspectFlags,
) {
    unimplemented!("vkCmdSetAttachmentFeedbackLoopEnableEXT(commandBuffer, aspectMask")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetMemoryZirconHandleFUCHSIA(
    device: VkDevice,
    pGetZirconHandleInfo: Option<NonNull<VkMemoryGetZirconHandleInfoFUCHSIA>>,
    pZirconHandle: Option<NonNull<zx_handle_t>>,
) -> VkResult {
    unimplemented!("vkGetMemoryZirconHandleFUCHSIA(device, pGetZirconHandleInfo, pZirconHandle")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateFramebuffer(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkFramebufferCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFramebuffer: Option<NonNull<VkFramebuffer>>,
) -> VkResult {
    unimplemented!("vkCreateFramebuffer(device, pCreateInfo, pAllocator, pFramebuffer")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetScissorWithCount(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: Option<NonNull<VkRect2D>>,
) {
    unimplemented!("vkCmdSetScissorWithCount(commandBuffer, scissorCount, pScissors")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdNextSubpass(
    commandBuffer: VkCommandBuffer,
    contents: VkSubpassContents,
) {
    unimplemented!("vkCmdNextSubpass(commandBuffer, contents")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBindTransformFeedbackBuffersEXT(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: Option<NonNull<VkBuffer>>,
    pOffsets: Option<NonNull<VkDeviceSize>>,
    pSizes: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!(
        "vkCmdBindTransformFeedbackBuffersEXT(
        commandBuffer,
        firstBinding,
        bindingCount,
        pBuffers,
        pOffsets,
        pSizes,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: isize,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!(
        "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
        device, pipeline, firstGroup, groupCount, dataSize, pData,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginRendering(
    commandBuffer: VkCommandBuffer,
    pRenderingInfo: Option<NonNull<VkRenderingInfo>>,
) {
    unimplemented!("vkCmdBeginRendering(commandBuffer, pRenderingInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureMemoryRequirementsNV(
    device: VkDevice,
    pInfo: Option<NonNull<VkAccelerationStructureMemoryRequirementsInfoNV>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2KHR>>,
) {
    unimplemented!(
        "vkGetAccelerationStructureMemoryRequirementsNV(device, pInfo, pMemoryRequirements"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyCuFunctionNVX(
    device: VkDevice,
    function: VkCuFunctionNVX,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyCuFunctionNVX(device, function, pAllocator")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetDescriptorBufferOffsetsEXT(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    setCount: u32,
    pBufferIndices: Option<NonNull<u32>>,
    pOffsets: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!(
        "vkCmdSetDescriptorBufferOffsetsEXT(
        commandBuffer,
        pipelineBindPoint,
        layout,
        firstSet,
        setCount,
        pBufferIndices,
        pOffsets,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceMicromapCompatibilityEXT(
    device: VkDevice,
    pVersionInfo: Option<NonNull<VkMicromapVersionInfoEXT>>,
    pCompatibility: Option<NonNull<VkAccelerationStructureCompatibilityKHR>>,
) {
    unimplemented!("vkGetDeviceMicromapCompatibilityEXT(device, pVersionInfo, pCompatibility")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdDecompressMemoryNV(
    commandBuffer: VkCommandBuffer,
    decompressRegionCount: u32,
    pDecompressMemoryRegions: Option<NonNull<VkDecompressMemoryRegionNV>>,
) {
    unimplemented!(
        "vkCmdDecompressMemoryNV(
        commandBuffer,
        decompressRegionCount,
        pDecompressMemoryRegions,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkUpdateDescriptorSets(
    device: VkDevice,
    descriptorWriteCount: u32,
    pDescriptorWrites: Option<NonNull<VkWriteDescriptorSet>>,
    descriptorCopyCount: u32,
    pDescriptorCopies: Option<NonNull<VkCopyDescriptorSet>>,
) {
    unimplemented!(
        "vkUpdateDescriptorSets(
        device,
        descriptorWriteCount,
        pDescriptorWrites,
        descriptorCopyCount,
        pDescriptorCopies,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(
    instance: VkInstance,
    pName: Option<NonNull<std::ffi::c_char>>,
) -> PFN_vkVoidFunction {
    unimplemented!("vkGetInstanceProcAddr(instance, pName")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetFenceFdKHR(
    device: VkDevice,
    pGetFdInfo: Option<NonNull<VkFenceGetFdInfoKHR>>,
    pFd: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!("vkGetFenceFdKHR(device, pGetFdInfo, pFd")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSemaphoreCounterValue(
    device: VkDevice,
    semaphore: VkSemaphore,
    pValue: Option<NonNull<u64>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreCounterValue(device, semaphore, pValue")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetStencilTestEnable(
    commandBuffer: VkCommandBuffer,
    stencilTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetStencilTestEnable(commandBuffer, stencilTestEnable")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDebugReportCallbackEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkDebugReportCallbackCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pCallback: Option<NonNull<VkDebugReportCallbackEXT>>,
) -> VkResult {
    unimplemented!("vkCreateDebugReportCallbackEXT(instance, pCreateInfo, pAllocator, pCallback")
}

#[no_mangle]
pub unsafe extern "C" fn vkBuildMicromapsEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: Option<NonNull<VkMicromapBuildInfoEXT>>,
) -> VkResult {
    unimplemented!("vkBuildMicromapsEXT(device, deferredOperation, infoCount, pInfos")
}

#[no_mangle]
pub unsafe extern "C" fn vkSetBufferCollectionBufferConstraintsFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pBufferConstraintsInfo: Option<NonNull<VkBufferConstraintsInfoFUCHSIA>>,
) -> VkResult {
    unimplemented!(
        "vkSetBufferCollectionBufferConstraintsFUCHSIA(device, collection, pBufferConstraintsInfo"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetPhysicalDeviceToolProperties(
    physicalDevice: VkPhysicalDevice,
    pToolCount: Option<NonNull<u32>>,
    pToolProperties: Option<NonNull<VkPhysicalDeviceToolProperties>>,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceToolProperties(physicalDevice, pToolCount, pToolProperties")
}

#[no_mangle]
pub unsafe extern "C" fn vkBuildAccelerationStructuresKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: Option<NonNull<VkAccelerationStructureBuildGeometryInfoKHR>>,
    ppBuildRangeInfos: Option<NonNull<VkAccelerationStructureBuildRangeInfoKHR>>,
) -> VkResult {
    unimplemented!(
        "vkBuildAccelerationStructuresKHR(
        device,
        deferredOperation,
        infoCount,
        pInfos,
        ppBuildRangeInfos,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkGetDeviceGroupSurfacePresentModesKHR(
    device: VkDevice,
    surface: VkSurfaceKHR,
    pModes: Option<NonNull<VkDeviceGroupPresentModeFlagsKHR>>,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupSurfacePresentModesKHR(device, surface, pModes")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetSwapchainGrallocUsageANDROID(
    device: VkDevice,
    format: VkFormat,
    imageUsage: VkImageUsageFlags,
    grallocUsage: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!("vkGetSwapchainGrallocUsageANDROID(device, format, imageUsage, grallocUsage")
}

#[no_mangle]
pub unsafe extern "C" fn vkSubmitDebugUtilsMessageEXT(
    instance: VkInstance,
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: Option<NonNull<VkDebugUtilsMessengerCallbackDataEXT>>,
) {
    unimplemented!(
        "vkSubmitDebugUtilsMessageEXT(instance, messageSeverity, messageTypes, pCallbackData"
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdSetPolygonModeEXT(
    commandBuffer: VkCommandBuffer,
    polygonMode: VkPolygonMode,
) {
    unimplemented!("vkCmdSetPolygonModeEXT(commandBuffer, polygonMode")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginRenderPass(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: Option<NonNull<VkRenderPassBeginInfo>>,
    contents: VkSubpassContents,
) {
    unimplemented!("vkCmdBeginRenderPass(commandBuffer, pRenderPassBegin, contents")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdCopyAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyAccelerationStructureInfoKHR>>,
) {
    unimplemented!("vkCmdCopyAccelerationStructureKHR(commandBuffer, pInfo")
}

#[no_mangle]
pub unsafe extern "C" fn vkCmdBeginConditionalRenderingEXT(
    commandBuffer: VkCommandBuffer,
    pConditionalRenderingBegin: Option<NonNull<VkConditionalRenderingBeginInfoEXT>>,
) {
    unimplemented!("vkCmdBeginConditionalRenderingEXT(commandBuffer, pConditionalRenderingBegin")
}

#[no_mangle]
pub unsafe extern "C" fn vkResetQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    unimplemented!("vkResetQueryPool(device, queryPool, firstQuery, queryCount")
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateImage(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkImageCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pImage: Option<NonNull<VkImage>>,
) -> VkResult {
    unimplemented!("vkCreateImage(device, pCreateInfo, pAllocator, pImage")
}

#[no_mangle]
pub unsafe extern "C" fn vkWriteMicromapsPropertiesEXT(
    device: VkDevice,
    micromapCount: u32,
    pMicromaps: Option<NonNull<VkMicromapEXT>>,
    queryType: VkQueryType,
    dataSize: isize,
    pData: Option<NonNull<std::ffi::c_void>>,
    stride: isize,
) -> VkResult {
    unimplemented!(
        "vkWriteMicromapsPropertiesEXT(
        device,
        micromapCount,
        pMicromaps,
        queryType,
        dataSize,
        pData,
        stride,
    "
    )
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateDisplayModeKHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pCreateInfo: Option<NonNull<VkDisplayModeCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMode: Option<NonNull<VkDisplayModeKHR>>,
) -> VkResult {
    unimplemented!("vkCreateDisplayModeKHR(physicalDevice, display, pCreateInfo, pAllocator, pMode")
}

#[no_mangle]
pub unsafe extern "C" fn vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkAccelerationStructureCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}
