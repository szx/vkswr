/* Runtime crate function wrappers. */
#![allow(non_snake_case)]
#![allow(unused)]

use crate::buffer::*;
use crate::command_buffer::*;
use crate::descriptor::*;
use crate::image::*;
use crate::memory::*;
use crate::pipeline::*;
use crate::sampler::*;
use crate::swapchain::*;
use headers::vk_decls::*;
use headers::vk_defs::*;
use runtime::*;
use std::sync::{Arc, Weak};

pub unsafe extern "C" fn vkCreateInstance(
    pCreateInfo: Option<NonNull<VkInstanceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pInstance: Option<NonNull<VkInstance>>,
) -> VkResult {
    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO
    );

    let _ = pAllocator;

    let Some(mut pInstance) = pInstance else {
        unreachable!()
    };

    *pInstance.as_ptr() = Instance::new();

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkEnumeratePhysicalDevices(
    instance: VkInstance,
    pPhysicalDeviceCount: Option<NonNull<u32>>,
    pPhysicalDevices: Option<NonNull<VkPhysicalDevice>>,
) -> VkResult {
    let Some(instance) = Instance::from_handle(instance) else {
        unreachable!()
    };

    let Some(pPhysicalDeviceCount) = pPhysicalDeviceCount else {
        unreachable!()
    };

    pPhysicalDevices.map_or_else(
        || {
            *pPhysicalDeviceCount.as_ptr() = PhysicalDevice::physical_device_count() as u32;
            VkResult::VK_SUCCESS
        },
        |pPhysicalDevices| {
            *pPhysicalDevices.as_ptr() = PhysicalDevice::get();
            VkResult::VK_SUCCESS
        },
    )
}

pub unsafe extern "C" fn vkGetPhysicalDeviceProperties(
    physicalDevice: VkPhysicalDevice,
    pProperties: Option<NonNull<VkPhysicalDeviceProperties>>,
) {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pProperties) = pProperties else {
        unreachable!()
    };

    // SPEC: "Returns properties of a physical device"
    *pProperties.as_ptr() = physicalDevice.lock().properties();
}

pub unsafe extern "C" fn vkGetPhysicalDeviceMemoryProperties(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: Option<NonNull<VkPhysicalDeviceMemoryProperties>>,
) {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pMemoryProperties) = pMemoryProperties else {
        unreachable!()
    };

    // SPEC: "Reports memory information for the specified physical device"
    *pMemoryProperties.as_ptr() = physicalDevice.lock().memory_properties();
}

pub unsafe extern "C" fn vkGetPhysicalDeviceFeatures(
    physicalDevice: VkPhysicalDevice,
    pFeatures: Option<NonNull<VkPhysicalDeviceFeatures>>,
) {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pFeatures) = pFeatures else {
        unreachable!()
    };

    // SPEC: "Reports capabilities of a physical device"
    *pFeatures.as_ptr() = physicalDevice.lock().features();
}

pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: Option<NonNull<u32>>,
    pQueueFamilyProperties: Option<NonNull<VkQueueFamilyProperties>>,
) {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pQueueFamilyPropertyCount) = pQueueFamilyPropertyCount else {
        unreachable!()
    };

    // SPEC: "Reports properties of the queues of the specified physical device"
    if pQueueFamilyProperties.is_none() {
        *pQueueFamilyPropertyCount.as_ptr() =
            physicalDevice.lock().queue_family_properties().len() as u32;
    } else {
        let Some(pQueueFamilyProperties) = pQueueFamilyProperties else {
            unreachable!()
        };
        let queue_family_properties = physicalDevice.lock().queue_family_properties();
        pQueueFamilyProperties.as_ptr().copy_from(
            queue_family_properties.as_ptr(),
            queue_family_properties.len(),
        );
    }
}

pub unsafe extern "C" fn vkEnumerateInstanceExtensionProperties(
    pLayerName: Option<NonNull<std::ffi::c_char>>,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult {
    assert_eq!(pLayerName, None);
    if pProperties.is_none() {
        if let Some(pPropertyCount) = pPropertyCount {
            *pPropertyCount.as_ptr() = Instance::extension_count() as u32;
        }
    } else {
        let Some(pProperties) = pProperties else {
            unreachable!()
        };

        pLayerName.map_or_else(
            || {
                let properties = Instance::extension_properties();
                std::ptr::copy_nonoverlapping(
                    properties.as_ptr(),
                    pProperties.as_ptr(),
                    properties.len(),
                );
            },
            |pLayerName| {
                let Ok(layerName) = std::ffi::CStr::from_ptr(pLayerName.as_ptr()).to_str() else {
                    unreachable!()
                };
                todo!("layerName: {layerName}");
            },
        );
    }
    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkEnumerateDeviceExtensionProperties(
    physicalDevice: VkPhysicalDevice,
    pLayerName: Option<NonNull<std::ffi::c_char>>,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pPropertyCount) = pPropertyCount else {
        unreachable!()
    };

    if pLayerName.is_none() {
        if pProperties.is_none() {
            *pPropertyCount.as_ptr() = PhysicalDevice::extension_count() as u32;
        } else {
            let Some(pProperties) = pProperties else {
                unreachable!()
            };
            let properties = PhysicalDevice::extension_properties();
            std::ptr::copy_nonoverlapping(
                properties.as_ptr(),
                pProperties.as_ptr(),
                properties.len(),
            );
        }
    } else {
        let Some(pLayerName) = pLayerName else {
            unreachable!()
        };
        let Ok(layerName) = std::ffi::CStr::from_ptr(pLayerName.as_ptr()).to_str() else {
            unreachable!()
        };
        todo!("the device extensions provided by {layerName} layer are returned")
    }
    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkCreateDevice(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: Option<NonNull<VkDeviceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDevice: Option<NonNull<VkDevice>>,
) -> VkResult {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO
    );
    let queue_create_info = create_info.pQueueCreateInfos;

    let _ = pAllocator;

    let Some(pDevice) = pDevice else {
        unreachable!()
    };

    let Some(queue_create_info) = create_info.pQueueCreateInfos else {
        return VkResult::VK_ERROR_INITIALIZATION_FAILED;
    };
    let queue_create_info = queue_create_info.as_ref();

    *pDevice.as_ptr() = LogicalDevice::new(physicalDevice.clone(), queue_create_info);
    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkGetDeviceProcAddr(
    device: VkDevice,
    pName: Option<NonNull<std::ffi::c_char>>,
) -> PFN_vkVoidFunction {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pName) = pName else { unreachable!() };
    let Ok(pName) = std::ffi::CStr::from_ptr(pName.as_ptr()).to_str() else {
        unreachable!()
    };

    // SPEC: The overhead of the internal dispatch for VkDevice objects can be avoided by obtaining
    // device-specific function pointers for any commands that use a device or device-child object
    // as their dispatchable object.

    match pName {
        /* Vulkan Core 1.0 device commands. */
        "vkDestroyDevice" => unsafe { std::mem::transmute(vkDestroyDevice as *const ()) },
        "vkGetDeviceQueue" => unsafe { std::mem::transmute(vkGetDeviceQueue as *const ()) },
        "vkQueueSubmit" => unsafe { std::mem::transmute(vkQueueSubmit as *const ()) },
        "vkQueueWaitIdle" => unsafe { std::mem::transmute(vkQueueWaitIdle as *const ()) },
        "vkDeviceWaitIdle" => unsafe { std::mem::transmute(vkDeviceWaitIdle as *const ()) },
        "vkAllocateMemory" => unsafe { std::mem::transmute(vkAllocateMemory as *const ()) },
        "vkFreeMemory" => unsafe { std::mem::transmute(vkFreeMemory as *const ()) },
        "vkMapMemory" => unsafe { std::mem::transmute(vkMapMemory as *const ()) },
        "vkUnmapMemory" => unsafe { std::mem::transmute(vkUnmapMemory as *const ()) },
        "vkFlushMappedMemoryRanges" => unsafe {
            std::mem::transmute(vkFlushMappedMemoryRanges as *const ())
        },
        "vkInvalidateMappedMemoryRanges" => unsafe {
            std::mem::transmute(vkInvalidateMappedMemoryRanges as *const ())
        },
        "vkGetDeviceMemoryCommitment" => unsafe {
            std::mem::transmute(vkGetDeviceMemoryCommitment as *const ())
        },
        "vkBindBufferMemory" => unsafe { std::mem::transmute(vkBindBufferMemory as *const ()) },
        "vkBindImageMemory" => unsafe { std::mem::transmute(vkBindImageMemory as *const ()) },
        "vkGetBufferMemoryRequirements" => unsafe {
            std::mem::transmute(vkGetBufferMemoryRequirements as *const ())
        },
        "vkGetImageMemoryRequirements" => unsafe {
            std::mem::transmute(vkGetImageMemoryRequirements as *const ())
        },
        "vkGetImageSparseMemoryRequirements" => unsafe {
            std::mem::transmute(vkGetImageSparseMemoryRequirements as *const ())
        },
        "vkQueueBindSparse" => unsafe { std::mem::transmute(vkQueueBindSparse as *const ()) },
        "vkCreateFence" => unsafe { std::mem::transmute(vkCreateFence as *const ()) },
        "vkDestroyFence" => unsafe { std::mem::transmute(vkDestroyFence as *const ()) },
        "vkResetFences" => unsafe { std::mem::transmute(vkResetFences as *const ()) },
        "vkGetFenceStatus" => unsafe { std::mem::transmute(vkGetFenceStatus as *const ()) },
        "vkWaitForFences" => unsafe { std::mem::transmute(vkWaitForFences as *const ()) },
        "vkCreateSemaphore" => unsafe { std::mem::transmute(vkCreateSemaphore as *const ()) },
        "vkDestroySemaphore" => unsafe { std::mem::transmute(vkDestroySemaphore as *const ()) },
        "vkCreateEvent" => unsafe { std::mem::transmute(vkCreateEvent as *const ()) },
        "vkDestroyEvent" => unsafe { std::mem::transmute(vkDestroyEvent as *const ()) },
        "vkGetEventStatus" => unsafe { std::mem::transmute(vkGetEventStatus as *const ()) },
        "vkSetEvent" => unsafe { std::mem::transmute(vkSetEvent as *const ()) },
        "vkResetEvent" => unsafe { std::mem::transmute(vkResetEvent as *const ()) },
        "vkCreateQueryPool" => unsafe { std::mem::transmute(vkCreateQueryPool as *const ()) },
        "vkDestroyQueryPool" => unsafe { std::mem::transmute(vkDestroyQueryPool as *const ()) },
        "vkGetQueryPoolResults" => unsafe {
            std::mem::transmute(vkGetQueryPoolResults as *const ())
        },
        "vkCreateBuffer" => unsafe { std::mem::transmute(vkCreateBuffer as *const ()) },
        "vkDestroyBuffer" => unsafe { std::mem::transmute(vkDestroyBuffer as *const ()) },
        "vkCreateBufferView" => unsafe { std::mem::transmute(vkCreateBufferView as *const ()) },
        "vkDestroyBufferView" => unsafe { std::mem::transmute(vkDestroyBufferView as *const ()) },
        "vkCreateImage" => unsafe { std::mem::transmute(vkCreateImage as *const ()) },
        "vkDestroyImage" => unsafe { std::mem::transmute(vkDestroyImage as *const ()) },
        "vkGetImageSubresourceLayout" => unsafe {
            std::mem::transmute(vkGetImageSubresourceLayout as *const ())
        },
        "vkCreateImageView" => unsafe { std::mem::transmute(vkCreateImageView as *const ()) },
        "vkDestroyImageView" => unsafe { std::mem::transmute(vkDestroyImageView as *const ()) },
        "vkCreateShaderModule" => unsafe { std::mem::transmute(vkCreateShaderModule as *const ()) },
        "vkDestroyShaderModule" => unsafe {
            std::mem::transmute(vkDestroyShaderModule as *const ())
        },
        "vkCreatePipelineCache" => unsafe {
            std::mem::transmute(vkCreatePipelineCache as *const ())
        },
        "vkDestroyPipelineCache" => unsafe {
            std::mem::transmute(vkDestroyPipelineCache as *const ())
        },
        "vkGetPipelineCacheData" => unsafe {
            std::mem::transmute(vkGetPipelineCacheData as *const ())
        },
        "vkMergePipelineCaches" => unsafe {
            std::mem::transmute(vkMergePipelineCaches as *const ())
        },
        "vkCreateGraphicsPipelines" => unsafe {
            std::mem::transmute(vkCreateGraphicsPipelines as *const ())
        },
        "vkCreateComputePipelines" => unsafe {
            std::mem::transmute(vkCreateComputePipelines as *const ())
        },
        "vkDestroyPipeline" => unsafe { std::mem::transmute(vkDestroyPipeline as *const ()) },
        "vkCreatePipelineLayout" => unsafe {
            std::mem::transmute(vkCreatePipelineLayout as *const ())
        },
        "vkDestroyPipelineLayout" => unsafe {
            std::mem::transmute(vkDestroyPipelineLayout as *const ())
        },
        "vkCreateSampler" => unsafe { std::mem::transmute(vkCreateSampler as *const ()) },
        "vkDestroySampler" => unsafe { std::mem::transmute(vkDestroySampler as *const ()) },
        "vkCreateDescriptorSetLayout" => unsafe {
            std::mem::transmute(vkCreateDescriptorSetLayout as *const ())
        },
        "vkDestroyDescriptorSetLayout" => unsafe {
            std::mem::transmute(vkDestroyDescriptorSetLayout as *const ())
        },
        "vkCreateDescriptorPool" => unsafe {
            std::mem::transmute(vkCreateDescriptorPool as *const ())
        },
        "vkDestroyDescriptorPool" => unsafe {
            std::mem::transmute(vkDestroyDescriptorPool as *const ())
        },
        "vkResetDescriptorPool" => unsafe {
            std::mem::transmute(vkResetDescriptorPool as *const ())
        },
        "vkAllocateDescriptorSets" => unsafe {
            std::mem::transmute(vkAllocateDescriptorSets as *const ())
        },
        "vkFreeDescriptorSets" => unsafe { std::mem::transmute(vkFreeDescriptorSets as *const ()) },
        "vkUpdateDescriptorSets" => unsafe {
            std::mem::transmute(vkUpdateDescriptorSets as *const ())
        },
        "vkCreateFramebuffer" => unsafe { std::mem::transmute(vkCreateFramebuffer as *const ()) },
        "vkDestroyFramebuffer" => unsafe { std::mem::transmute(vkDestroyFramebuffer as *const ()) },
        "vkCreateRenderPass" => unsafe { std::mem::transmute(vkCreateRenderPass as *const ()) },
        "vkDestroyRenderPass" => unsafe { std::mem::transmute(vkDestroyRenderPass as *const ()) },
        "vkGetRenderAreaGranularity" => unsafe {
            std::mem::transmute(vkGetRenderAreaGranularity as *const ())
        },
        "vkCreateCommandPool" => unsafe { std::mem::transmute(vkCreateCommandPool as *const ()) },
        "vkDestroyCommandPool" => unsafe { std::mem::transmute(vkDestroyCommandPool as *const ()) },
        "vkResetCommandPool" => unsafe { std::mem::transmute(vkResetCommandPool as *const ()) },
        "vkAllocateCommandBuffers" => unsafe {
            std::mem::transmute(vkAllocateCommandBuffers as *const ())
        },
        "vkFreeCommandBuffers" => unsafe { std::mem::transmute(vkFreeCommandBuffers as *const ()) },
        "vkBeginCommandBuffer" => unsafe { std::mem::transmute(vkBeginCommandBuffer as *const ()) },
        "vkEndCommandBuffer" => unsafe { std::mem::transmute(vkEndCommandBuffer as *const ()) },
        "vkResetCommandBuffer" => unsafe { std::mem::transmute(vkResetCommandBuffer as *const ()) },
        "vkCmdBindPipeline" => unsafe { std::mem::transmute(vkCmdBindPipeline as *const ()) },
        "vkCmdSetViewport" => unsafe { std::mem::transmute(vkCmdSetViewport as *const ()) },
        "vkCmdSetScissor" => unsafe { std::mem::transmute(vkCmdSetScissor as *const ()) },
        "vkCmdSetLineWidth" => unsafe { std::mem::transmute(vkCmdSetLineWidth as *const ()) },
        "vkCmdSetDepthBias" => unsafe { std::mem::transmute(vkCmdSetDepthBias as *const ()) },
        "vkCmdSetBlendConstants" => unsafe {
            std::mem::transmute(vkCmdSetBlendConstants as *const ())
        },
        "vkCmdSetDepthBounds" => unsafe { std::mem::transmute(vkCmdSetDepthBounds as *const ()) },
        "vkCmdSetStencilCompareMask" => unsafe {
            std::mem::transmute(vkCmdSetStencilCompareMask as *const ())
        },
        "vkCmdSetStencilWriteMask" => unsafe {
            std::mem::transmute(vkCmdSetStencilWriteMask as *const ())
        },
        "vkCmdSetStencilReference" => unsafe {
            std::mem::transmute(vkCmdSetStencilReference as *const ())
        },
        "vkCmdBindDescriptorSets" => unsafe {
            std::mem::transmute(vkCmdBindDescriptorSets as *const ())
        },
        "vkCmdBindIndexBuffer" => unsafe { std::mem::transmute(vkCmdBindIndexBuffer as *const ()) },
        "vkCmdBindVertexBuffers" => unsafe {
            std::mem::transmute(vkCmdBindVertexBuffers as *const ())
        },
        "vkCmdDraw" => unsafe { std::mem::transmute(vkCmdDraw as *const ()) },
        "vkCmdDrawIndexed" => unsafe { std::mem::transmute(vkCmdDrawIndexed as *const ()) },
        "vkCmdDrawIndirect" => unsafe { std::mem::transmute(vkCmdDrawIndirect as *const ()) },
        "vkCmdDrawIndexedIndirect" => unsafe {
            std::mem::transmute(vkCmdDrawIndexedIndirect as *const ())
        },
        "vkCmdDispatch" => unsafe { std::mem::transmute(vkCmdDispatch as *const ()) },
        "vkCmdDispatchIndirect" => unsafe {
            std::mem::transmute(vkCmdDispatchIndirect as *const ())
        },
        "vkCmdCopyBuffer" => unsafe { std::mem::transmute(vkCmdCopyBuffer as *const ()) },
        "vkCmdCopyImage" => unsafe { std::mem::transmute(vkCmdCopyImage as *const ()) },
        "vkCmdBlitImage" => unsafe { std::mem::transmute(vkCmdBlitImage as *const ()) },
        "vkCmdCopyBufferToImage" => unsafe {
            std::mem::transmute(vkCmdCopyBufferToImage as *const ())
        },
        "vkCmdCopyImageToBuffer" => unsafe {
            std::mem::transmute(vkCmdCopyImageToBuffer as *const ())
        },
        "vkCmdUpdateBuffer" => unsafe { std::mem::transmute(vkCmdUpdateBuffer as *const ()) },
        "vkCmdFillBuffer" => unsafe { std::mem::transmute(vkCmdFillBuffer as *const ()) },
        "vkCmdClearColorImage" => unsafe { std::mem::transmute(vkCmdClearColorImage as *const ()) },
        "vkCmdClearDepthStencilImage" => unsafe {
            std::mem::transmute(vkCmdClearDepthStencilImage as *const ())
        },
        "vkCmdClearAttachments" => unsafe {
            std::mem::transmute(vkCmdClearAttachments as *const ())
        },
        "vkCmdResolveImage" => unsafe { std::mem::transmute(vkCmdResolveImage as *const ()) },
        "vkCmdSetEvent" => unsafe { std::mem::transmute(vkCmdSetEvent as *const ()) },
        "vkCmdResetEvent" => unsafe { std::mem::transmute(vkCmdResetEvent as *const ()) },
        "vkCmdWaitEvents" => unsafe { std::mem::transmute(vkCmdWaitEvents as *const ()) },
        "vkCmdPipelineBarrier" => unsafe { std::mem::transmute(vkCmdPipelineBarrier as *const ()) },
        "vkCmdBeginQuery" => unsafe { std::mem::transmute(vkCmdBeginQuery as *const ()) },
        "vkCmdEndQuery" => unsafe { std::mem::transmute(vkCmdEndQuery as *const ()) },
        "vkCmdResetQueryPool" => unsafe { std::mem::transmute(vkCmdResetQueryPool as *const ()) },
        "vkCmdWriteTimestamp" => unsafe { std::mem::transmute(vkCmdWriteTimestamp as *const ()) },
        "vkCmdCopyQueryPoolResults" => unsafe {
            std::mem::transmute(vkCmdCopyQueryPoolResults as *const ())
        },
        "vkCmdPushConstants" => unsafe { std::mem::transmute(vkCmdPushConstants as *const ()) },
        "vkCmdBeginRenderPass" => unsafe { std::mem::transmute(vkCmdBeginRenderPass as *const ()) },
        "vkCmdNextSubpass" => unsafe { std::mem::transmute(vkCmdNextSubpass as *const ()) },
        "vkCmdEndRenderPass" => unsafe { std::mem::transmute(vkCmdEndRenderPass as *const ()) },
        "vkCmdExecuteCommands" => unsafe { std::mem::transmute(vkCmdExecuteCommands as *const ()) },
        /* VK_KHR_swapchain extension device commands */
        "vkCreateSwapchainKHR" => unsafe { std::mem::transmute(vkCreateSwapchainKHR as *const ()) },
        "vkDestroySwapchainKHR" => unsafe {
            std::mem::transmute(vkDestroySwapchainKHR as *const ())
        },
        "vkGetSwapchainImagesKHR" => unsafe {
            std::mem::transmute(vkGetSwapchainImagesKHR as *const ())
        },
        "vkAcquireNextImageKHR" => unsafe {
            std::mem::transmute(vkAcquireNextImageKHR as *const ())
        },
        "vkQueuePresentKHR" => unsafe { std::mem::transmute(vkQueuePresentKHR as *const ()) },
        "vkGetDeviceGroupPresentCapabilitiesKHR" => unsafe {
            std::mem::transmute(vkGetDeviceGroupPresentCapabilitiesKHR as *const ())
        },
        "vkGetDeviceGroupSurfacePresentModesKHR" => unsafe {
            std::mem::transmute(vkGetDeviceGroupSurfacePresentModesKHR as *const ())
        },
        "vkAcquireNextImage2KHR" => unsafe {
            std::mem::transmute(vkAcquireNextImage2KHR as *const ())
        },
        &_ => None, // unreachable!("pName: {}", pName) TODO: Vulkan 1.1 Core commands.
    }
}

pub unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: Option<NonNull<VkFormatProperties>>,
) {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pFormatProperties) = pFormatProperties else {
        unreachable!()
    };

    // SPEC: "Reports capabilities of a physical device"
    *pFormatProperties.as_ptr() = physicalDevice.lock().format_properties(format);
}

pub unsafe extern "C" fn vkGetPhysicalDeviceImageFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    pImageFormatProperties: Option<NonNull<VkImageFormatProperties>>,
) -> VkResult {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pImageFormatProperties) = pImageFormatProperties else {
        unreachable!()
    };

    let properties = physicalDevice
        .lock()
        .image_format_properties(format, type_, tiling, usage, flags);
    if let Some(properties) = properties {
        *pImageFormatProperties.as_ptr() = properties;
        VkResult::VK_SUCCESS
    } else {
        VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED
    }
}

pub unsafe extern "C" fn vkDestroyDevice(
    device: VkDevice,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let _ = pAllocator;

    LogicalDevice::drop_handle(device);
}

pub unsafe extern "C" fn vkDestroyInstance(
    instance: VkInstance,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let _ = pAllocator;

    Instance::drop_handle(instance);
}

/* Vulkan Core 1.0 device commands  */

pub unsafe extern "C" fn vkGetDeviceQueue(
    device: VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: Option<NonNull<VkQueue>>,
) {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pQueue) = pQueue else { unreachable!() };

    let queue = device.lock().queue(queueFamilyIndex, queueIndex);
    *pQueue.as_ptr() = queue.lock().get_handle();
}

pub unsafe extern "C" fn vkCreateFence(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkFenceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFence: Option<NonNull<VkFence>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO
    );

    let _ = pAllocator;

    let Some(mut pFence) = pFence else {
        unreachable!()
    };

    *pFence.as_ptr() = Fence::create(device, create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkCreateSemaphore(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSemaphoreCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSemaphore: Option<NonNull<VkSemaphore>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO
    );

    let _ = pAllocator;

    let Some(mut pSemaphore) = pSemaphore else {
        unreachable!()
    };

    *pSemaphore.as_ptr() = Semaphore::create(create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkWaitForFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: Option<NonNull<VkFence>>,
    waitAll: VkBool32,
    timeout: u64,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    if fenceCount == 0 {
        return VkResult::VK_SUCCESS;
    }

    let Some(mut pFences) = pFences else {
        unreachable!()
    };
    let pFences = std::slice::from_raw_parts(pFences.as_ptr(), fenceCount as usize);
    let fences = pFences
        .iter()
        .map(|&handle| Fence::from_handle(handle).unwrap())
        .collect::<Vec<_>>();

    device.lock().wait_for_fences(fences, waitAll != 0, timeout);
    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkResetFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: Option<NonNull<VkFence>>,
) -> VkResult {
    let Some(device) = LogicalDevice::from_handle(device) else {
        unreachable!()
    };

    if fenceCount == 0 {
        return VkResult::VK_SUCCESS;
    }

    let Some(mut pFences) = pFences else {
        unreachable!()
    };
    let pFences = std::slice::from_raw_parts(pFences.as_ptr(), fenceCount as usize);
    let mut fences = pFences
        .iter()
        .map(|&handle| Fence::from_handle(handle).unwrap())
        .collect::<Vec<_>>();

    device.lock().reset_fences(fences);
    VkResult::VK_SUCCESS
}

/* VK_KHR_surface extension instance commands */

pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: Option<NonNull<VkBool32>>,
) -> VkResult {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pSupported) = pSupported else {
        unreachable!()
    };

    *pSupported.as_ptr() = physicalDevice
        .lock()
        .surface_support(queueFamilyIndex, surface) as VkBool32;

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkGetPhysicalDeviceSurfacePresentModesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pPresentModeCount: Option<NonNull<u32>>,
    pPresentModes: Option<NonNull<VkPresentModeKHR>>,
) -> VkResult {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pPresentModeCount) = pPresentModeCount else {
        unreachable!()
    };

    pPresentModes.map_or_else(
        || {
            *pPresentModeCount.as_ptr() = physicalDevice.lock().present_modes().len() as u32;
        },
        |pPresentModes| {
            let present_modes = physicalDevice.lock().present_modes();
            std::ptr::copy_nonoverlapping(
                present_modes.as_ptr(),
                pPresentModes.as_ptr(),
                *pPresentModeCount.as_ptr() as usize,
            );
        },
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceFormatsKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceFormatCount: Option<NonNull<u32>>,
    pSurfaceFormats: Option<NonNull<VkSurfaceFormatKHR>>,
) -> VkResult {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pSurfaceFormatCount) = pSurfaceFormatCount else {
        unreachable!()
    };

    pSurfaceFormats.map_or_else(
        || {
            *pSurfaceFormatCount.as_ptr() = physicalDevice.lock().surface_formats().len() as u32;
        },
        |pSurfaceFormats| {
            let surface_formats = physicalDevice.lock().surface_formats();
            std::ptr::copy_nonoverlapping(
                surface_formats.as_ptr(),
                pSurfaceFormats.as_ptr(),
                *pSurfaceFormatCount.as_ptr() as usize,
            );
        },
    );

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: Option<NonNull<VkSurfaceCapabilitiesKHR>>,
) -> VkResult {
    let Some(physicalDevice) = PhysicalDevice::from_handle(physicalDevice) else {
        unreachable!()
    };

    let Some(pSurfaceCapabilities) = pSurfaceCapabilities else {
        unreachable!()
    };

    *pSurfaceCapabilities.as_ptr() = physicalDevice.lock().surface_capabilities();

    VkResult::VK_SUCCESS
}

/* unimplemented */

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

pub unsafe extern "C" fn vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
    device: VkDevice,
    renderpass: VkRenderPass,
    pMaxWorkgroupSize: Option<NonNull<VkExtent2D>>,
) -> VkResult {
    unimplemented!(
        "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(device, renderpass, pMaxWorkgroupSize"
    )
}

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

pub unsafe extern "C" fn vkCmdSetProvokingVertexModeEXT(
    commandBuffer: VkCommandBuffer,
    provokingVertexMode: VkProvokingVertexModeEXT,
) {
    unimplemented!("vkCmdSetProvokingVertexModeEXT(commandBuffer, provokingVertexMode")
}

pub unsafe extern "C" fn vkCreatePrivateDataSlot(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkPrivateDataSlotCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pPrivateDataSlot: Option<NonNull<VkPrivateDataSlot>>,
) -> VkResult {
    unimplemented!("vkCreatePrivateDataSlot(device, pCreateInfo, pAllocator, pPrivateDataSlot")
}

pub unsafe extern "C" fn vkCopyMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyMicromapInfoEXT>>,
) -> VkResult {
    unimplemented!("vkCopyMicromapEXT(device, deferredOperation, pInfo")
}

pub unsafe extern "C" fn vkCreateScreenSurfaceQNX(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkScreenSurfaceCreateInfoQNX>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateScreenSurfaceQNX(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkDestroyIndirectCommandsLayoutNV(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyIndirectCommandsLayoutNV(device, indirectCommandsLayout, pAllocator")
}

pub unsafe extern "C" fn vkCmdSetCoverageModulationTableEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableEnable: VkBool32,
) {
    unimplemented!(
        "vkCmdSetCoverageModulationTableEnableNV(commandBuffer, coverageModulationTableEnable"
    )
}

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

pub unsafe extern "C" fn vkCmdRefreshObjectsKHR(
    commandBuffer: VkCommandBuffer,
    pRefreshObjects: Option<NonNull<VkRefreshObjectListKHR>>,
) {
    unimplemented!("vkCmdRefreshObjectsKHR(commandBuffer, pRefreshObjects")
}

pub unsafe extern "C" fn vkCmdPipelineBarrier2(
    commandBuffer: VkCommandBuffer,
    pDependencyInfo: Option<NonNull<VkDependencyInfo>>,
) {
    unimplemented!("vkCmdPipelineBarrier2(commandBuffer, pDependencyInfo")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceWin32PresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice, queueFamilyIndex"
    )
}

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

pub unsafe extern "C" fn vkCmdSetDepthClampEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthClampEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClampEnableEXT(commandBuffer, depthClampEnable")
}

pub unsafe extern "C" fn vkGetBufferDeviceAddress(
    device: VkDevice,
    pInfo: Option<NonNull<VkBufferDeviceAddressInfo>>,
) -> VkDeviceAddress {
    unimplemented!("vkGetBufferDeviceAddress(device, pInfo")
}

pub unsafe extern "C" fn vkBindAccelerationStructureMemoryNV(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: Option<NonNull<VkBindAccelerationStructureMemoryInfoNV>>,
) -> VkResult {
    unimplemented!("vkBindAccelerationStructureMemoryNV(device, bindInfoCount, pBindInfos")
}

pub unsafe extern "C" fn vkCmdCopyBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferInfo: Option<NonNull<VkCopyBufferInfo2>>,
) {
    unimplemented!("vkCmdCopyBuffer2(commandBuffer, pCopyBufferInfo")
}

pub unsafe extern "C" fn vkGetBufferOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: Option<NonNull<VkBufferDeviceAddressInfo>>,
) -> u64 {
    unimplemented!("vkGetBufferOpaqueCaptureAddress(device, pInfo")
}

pub unsafe extern "C" fn vkGetDeferredOperationMaxConcurrencyKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> u32 {
    unimplemented!("vkGetDeferredOperationMaxConcurrencyKHR(device, operation")
}

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

pub unsafe extern "C" fn vkGetImageOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetImageOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}

pub unsafe extern "C" fn vkGetQueueCheckpointDataNV(
    queue: VkQueue,
    pCheckpointDataCount: Option<NonNull<u32>>,
    pCheckpointData: Option<NonNull<VkCheckpointDataNV>>,
) {
    unimplemented!("vkGetQueueCheckpointDataNV(queue, pCheckpointDataCount, pCheckpointData")
}

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

pub unsafe extern "C" fn vkResetCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolResetFlags,
) -> VkResult {
    unimplemented!("vkResetCommandPool(device, commandPool, flags")
}

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

pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayPlanePropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice, pPropertyCount, pProperties"
    )
}

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

pub unsafe extern "C" fn vkCmdSetDepthWriteEnable(
    commandBuffer: VkCommandBuffer,
    depthWriteEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthWriteEnable(commandBuffer, depthWriteEnable")
}

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

pub unsafe extern "C" fn vkGetBufferMemoryRequirements2(
    device: VkDevice,
    pInfo: Option<NonNull<VkBufferMemoryRequirementsInfo2>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetBufferMemoryRequirements2(device, pInfo, pMemoryRequirements")
}

pub unsafe extern "C" fn vkCreateOpticalFlowSessionNV(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkOpticalFlowSessionCreateInfoNV>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSession: Option<NonNull<VkOpticalFlowSessionNV>>,
) -> VkResult {
    unimplemented!("vkCreateOpticalFlowSessionNV(device, pCreateInfo, pAllocator, pSession")
}

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

pub unsafe extern "C" fn vkQueueEndDebugUtilsLabelEXT(queue: VkQueue) {
    unimplemented!("vkQueueEndDebugUtilsLabelEXT(queue")
}

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

pub unsafe extern "C" fn vkCmdDrawClusterHUAWEI(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDrawClusterHUAWEI(commandBuffer, groupCountX, groupCountY, groupCountZ")
}

pub unsafe extern "C" fn vkCmdSetRayTracingPipelineStackSizeKHR(
    commandBuffer: VkCommandBuffer,
    pipelineStackSize: u32,
) {
    unimplemented!("vkCmdSetRayTracingPipelineStackSizeKHR(commandBuffer, pipelineStackSize")
}

pub unsafe extern "C" fn vkCopyMemoryToAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyMemoryToAccelerationStructureInfoKHR>>,
) -> VkResult {
    unimplemented!("vkCopyMemoryToAccelerationStructureKHR(device, deferredOperation, pInfo")
}

pub unsafe extern "C" fn vkDestroyEvent(
    device: VkDevice,
    event: VkEvent,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyEvent(device, event, pAllocator")
}

pub unsafe extern "C" fn vkDestroyDescriptorUpdateTemplate(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDescriptorUpdateTemplate(device, descriptorUpdateTemplate, pAllocator")
}

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

pub unsafe extern "C" fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndRenderPass(commandBuffer")
}

pub unsafe extern "C" fn vkGetPrivateData(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: Option<NonNull<u64>>,
) {
    unimplemented!("vkGetPrivateData(device, objectType, objectHandle, privateDataSlot, pData")
}

pub unsafe extern "C" fn vkCmdBeginRenderPass2(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: Option<NonNull<VkRenderPassBeginInfo>>,
    pSubpassBeginInfo: Option<NonNull<VkSubpassBeginInfo>>,
) {
    unimplemented!("vkCmdBeginRenderPass2(commandBuffer, pRenderPassBegin, pSubpassBeginInfo")
}

pub unsafe extern "C" fn vkCmdBindDescriptorBufferEmbeddedSamplersEXT(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
) {
    unimplemented!("vkCmdBindDescriptorBufferEmbeddedSamplersEXT(commandBuffer, pipelineBindPoint, layout, set")
}

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

pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: Option<NonNull<VkSurfaceCapabilities2EXT>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceSurfaceCapabilities2EXT(physicalDevice, surface, pSurfaceCapabilities"
    )
}

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

pub unsafe extern "C" fn vkCmdDrawMeshTasksEXT(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksEXT(commandBuffer, groupCountX, groupCountY, groupCountZ")
}

pub unsafe extern "C" fn vkSignalSemaphore(
    device: VkDevice,
    pSignalInfo: Option<NonNull<VkSemaphoreSignalInfo>>,
) -> VkResult {
    unimplemented!("vkSignalSemaphore(device, pSignalInfo")
}

pub unsafe extern "C" fn vkCmdDrawIndexedIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawIndexedIndirect(commandBuffer, buffer, offset, drawCount, stride")
}

pub unsafe extern "C" fn vkCmdSetRasterizerDiscardEnable(
    commandBuffer: VkCommandBuffer,
    rasterizerDiscardEnable: VkBool32,
) {
    unimplemented!("vkCmdSetRasterizerDiscardEnable(commandBuffer, rasterizerDiscardEnable")
}

pub unsafe extern "C" fn vkCmdSetLogicOpEXT(commandBuffer: VkCommandBuffer, logicOp: VkLogicOp) {
    unimplemented!("vkCmdSetLogicOpEXT(commandBuffer, logicOp")
}

pub unsafe extern "C" fn vkCmdEndQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    index: u32,
) {
    unimplemented!("vkCmdEndQueryIndexedEXT(commandBuffer, queryPool, query, index")
}

pub unsafe extern "C" fn vkCmdSetLineRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    lineRasterizationMode: VkLineRasterizationModeEXT,
) {
    unimplemented!("vkCmdSetLineRasterizationModeEXT(commandBuffer, lineRasterizationMode")
}

pub unsafe extern "C" fn vkCmdCopyImage2(
    commandBuffer: VkCommandBuffer,
    pCopyImageInfo: Option<NonNull<VkCopyImageInfo2>>,
) {
    unimplemented!("vkCmdCopyImage2(commandBuffer, pCopyImageInfo")
}

pub unsafe extern "C" fn vkDestroyOpticalFlowSessionNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyOpticalFlowSessionNV(device, session, pAllocator")
}

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

pub unsafe extern "C" fn vkCmdSetDiscardRectangleEnableEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDiscardRectangleEnableEXT(commandBuffer, discardRectangleEnable")
}

pub unsafe extern "C" fn vkFreeCommandBuffers(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: Option<NonNull<VkCommandBuffer>>,
) {
    unimplemented!("vkFreeCommandBuffers(device, commandPool, commandBufferCount, pCommandBuffers")
}

pub unsafe extern "C" fn vkGetQueueCheckpointData2NV(
    queue: VkQueue,
    pCheckpointDataCount: Option<NonNull<u32>>,
    pCheckpointData: Option<NonNull<VkCheckpointData2NV>>,
) {
    unimplemented!("vkGetQueueCheckpointData2NV(queue, pCheckpointDataCount, pCheckpointData")
}

pub unsafe extern "C" fn vkCreateMacOSSurfaceMVK(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkMacOSSurfaceCreateInfoMVK>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateMacOSSurfaceMVK(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkSetEvent(device, event")
}

pub unsafe extern "C" fn vkCmdSetStencilReference(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    reference: u32,
) {
    unimplemented!("vkCmdSetStencilReference(commandBuffer, faceMask, reference")
}

pub unsafe extern "C" fn vkCmdCopyBuffer(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: Option<NonNull<VkBufferCopy>>,
) {
    unimplemented!("vkCmdCopyBuffer(commandBuffer, srcBuffer, dstBuffer, regionCount, pRegions")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceMemoryProperties2(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: Option<NonNull<VkPhysicalDeviceMemoryProperties2>>,
) {
    unimplemented!("vkGetPhysicalDeviceMemoryProperties2(physicalDevice, pMemoryProperties")
}

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

pub unsafe extern "C" fn vkGetDisplayPlaneCapabilities2KHR(
    physicalDevice: VkPhysicalDevice,
    pDisplayPlaneInfo: Option<NonNull<VkDisplayPlaneInfo2KHR>>,
    pCapabilities: Option<NonNull<VkDisplayPlaneCapabilities2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetDisplayPlaneCapabilities2KHR(physicalDevice, pDisplayPlaneInfo, pCapabilities"
    )
}

pub unsafe extern "C" fn vkCmdSetCheckpointNV(
    commandBuffer: VkCommandBuffer,
    pCheckpointMarker: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!("vkCmdSetCheckpointNV(commandBuffer, pCheckpointMarker")
}

pub unsafe extern "C" fn vkCreateStreamDescriptorSurfaceGGP(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkStreamDescriptorSurfaceCreateInfoGGP>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateStreamDescriptorSurfaceGGP(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkCmdExecuteGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: Option<NonNull<VkGeneratedCommandsInfoNV>>,
) {
    unimplemented!(
        "vkCmdExecuteGeneratedCommandsNV(commandBuffer, isPreprocessed, pGeneratedCommandsInfo"
    )
}

pub unsafe extern "C" fn vkCreateSemaphoreSciSyncPoolNV(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSemaphoreSciSyncPoolCreateInfoNV>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSemaphorePool: Option<NonNull<VkSemaphoreSciSyncPoolNV>>,
) -> VkResult {
    unimplemented!("vkCreateSemaphoreSciSyncPoolNV(device, pCreateInfo, pAllocator, pSemaphorePool")
}

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

pub unsafe extern "C" fn vkCmdDrawClusterIndirectHUAWEI(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    unimplemented!("vkCmdDrawClusterIndirectHUAWEI(commandBuffer, buffer, offset")
}

pub unsafe extern "C" fn vkDestroySemaphoreSciSyncPoolNV(
    device: VkDevice,
    semaphorePool: VkSemaphoreSciSyncPoolNV,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySemaphoreSciSyncPoolNV(device, semaphorePool, pAllocator")
}

pub unsafe extern "C" fn vkQueueSubmit(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: Option<NonNull<VkSubmitInfo>>,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueSubmit(queue, submitCount, pSubmits, fence")
}

pub unsafe extern "C" fn vkSetHdrMetadataEXT(
    device: VkDevice,
    swapchainCount: u32,
    pSwapchains: Option<NonNull<VkSwapchainKHR>>,
    pMetadata: Option<NonNull<VkHdrMetadataEXT>>,
) {
    unimplemented!("vkSetHdrMetadataEXT(device, swapchainCount, pSwapchains, pMetadata")
}

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

pub unsafe extern "C" fn vkCmdTraceRaysIndirect2KHR(
    commandBuffer: VkCommandBuffer,
    indirectDeviceAddress: VkDeviceAddress,
) {
    unimplemented!("vkCmdTraceRaysIndirect2KHR(commandBuffer, indirectDeviceAddress")
}

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

pub unsafe extern "C" fn vkInitializePerformanceApiINTEL(
    device: VkDevice,
    pInitializeInfo: Option<NonNull<VkInitializePerformanceApiInfoINTEL>>,
) -> VkResult {
    unimplemented!("vkInitializePerformanceApiINTEL(device, pInitializeInfo")
}

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

pub unsafe extern "C" fn vkCmdSetRasterizationStreamEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationStream: u32,
) {
    unimplemented!("vkCmdSetRasterizationStreamEXT(commandBuffer, rasterizationStream")
}

pub unsafe extern "C" fn vkGetImageViewOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageViewCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetImageViewOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}

pub unsafe extern "C" fn vkCreateCuFunctionNVX(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkCuFunctionCreateInfoNVX>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFunction: Option<NonNull<VkCuFunctionNVX>>,
) -> VkResult {
    unimplemented!("vkCreateCuFunctionNVX(device, pCreateInfo, pAllocator, pFunction")
}

pub unsafe extern "C" fn vkDestroyMicromapEXT(
    device: VkDevice,
    micromap: VkMicromapEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyMicromapEXT(device, micromap, pAllocator")
}

pub unsafe extern "C" fn vkEnumerateDeviceLayerProperties(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkLayerProperties>>,
) -> VkResult {
    unimplemented!("vkEnumerateDeviceLayerProperties(physicalDevice, pPropertyCount, pProperties")
}

pub unsafe extern "C" fn vkReleaseSwapchainImagesEXT(
    device: VkDevice,
    pReleaseInfo: Option<NonNull<VkReleaseSwapchainImagesInfoEXT>>,
) -> VkResult {
    unimplemented!("vkReleaseSwapchainImagesEXT(device, pReleaseInfo")
}

pub unsafe extern "C" fn vkCmdExecuteCommands(
    commandBuffer: VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers: Option<NonNull<VkCommandBuffer>>,
) {
    unimplemented!("vkCmdExecuteCommands(commandBuffer, commandBufferCount, pCommandBuffers")
}

pub unsafe extern "C" fn vkGetDescriptorSetLayoutHostMappingInfoVALVE(
    device: VkDevice,
    pBindingReference: Option<NonNull<VkDescriptorSetBindingReferenceVALVE>>,
    pHostMapping: Option<NonNull<VkDescriptorSetLayoutHostMappingInfoVALVE>>,
) {
    unimplemented!(
        "vkGetDescriptorSetLayoutHostMappingInfoVALVE(device, pBindingReference, pHostMapping"
    )
}

pub unsafe extern "C" fn vkAcquireFullScreenExclusiveModeEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkAcquireFullScreenExclusiveModeEXT(device, swapchain")
}

pub unsafe extern "C" fn vkCmdCopyAccelerationStructureNV(
    commandBuffer: VkCommandBuffer,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    mode: VkCopyAccelerationStructureModeKHR,
) {
    unimplemented!("vkCmdCopyAccelerationStructureNV(commandBuffer, dst, src, mode")
}

pub unsafe extern "C" fn vkCmdCopyBufferToImage2(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: Option<NonNull<VkCopyBufferToImageInfo2>>,
) {
    unimplemented!("vkCmdCopyBufferToImage2(commandBuffer, pCopyBufferToImageInfo")
}

pub unsafe extern "C" fn vkCreateIOSSurfaceMVK(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkIOSSurfaceCreateInfoMVK>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateIOSSurfaceMVK(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkUpdateVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pUpdateInfo: Option<NonNull<VkVideoSessionParametersUpdateInfoKHR>>,
) -> VkResult {
    unimplemented!("vkUpdateVideoSessionParametersKHR(device, videoSessionParameters, pUpdateInfo")
}

pub unsafe extern "C" fn vkDestroyShaderEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyShaderEXT(device, shader, pAllocator")
}

pub unsafe extern "C" fn vkCmdWriteBufferMarker2AMD(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
) {
    unimplemented!("vkCmdWriteBufferMarker2AMD(commandBuffer, stage, dstBuffer, dstOffset, marker")
}

pub unsafe extern "C" fn vkEnumerateInstanceVersion(pApiVersion: Option<NonNull<u32>>) -> VkResult {
    unimplemented!("vkEnumerateInstanceVersion(pApiVersion")
}

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

pub unsafe extern "C" fn vkGetValidationCacheDataEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pDataSize: Option<NonNull<isize>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetValidationCacheDataEXT(device, validationCache, pDataSize, pData")
}

pub unsafe extern "C" fn vkAcquirePerformanceConfigurationINTEL(
    device: VkDevice,
    pAcquireInfo: Option<NonNull<VkPerformanceConfigurationAcquireInfoINTEL>>,
    pConfiguration: Option<NonNull<VkPerformanceConfigurationINTEL>>,
) -> VkResult {
    unimplemented!("vkAcquirePerformanceConfigurationINTEL(device, pAcquireInfo, pConfiguration")
}

pub unsafe extern "C" fn vkCreateMicromapEXT(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkMicromapCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMicromap: Option<NonNull<VkMicromapEXT>>,
) -> VkResult {
    unimplemented!("vkCreateMicromapEXT(device, pCreateInfo, pAllocator, pMicromap")
}

pub unsafe extern "C" fn vkCreateQueryPool(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkQueryPoolCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pQueryPool: Option<NonNull<VkQueryPool>>,
) -> VkResult {
    unimplemented!("vkCreateQueryPool(device, pCreateInfo, pAllocator, pQueryPool")
}

pub unsafe extern "C" fn vkGetDescriptorSetLayoutSupport(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorSetLayoutCreateInfo>>,
    pSupport: Option<NonNull<VkDescriptorSetLayoutSupport>>,
) {
    unimplemented!("vkGetDescriptorSetLayoutSupport(device, pCreateInfo, pSupport")
}

pub unsafe extern "C" fn vkCmdControlVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pCodingControlInfo: Option<NonNull<VkVideoCodingControlInfoKHR>>,
) {
    unimplemented!("vkCmdControlVideoCodingKHR(commandBuffer, pCodingControlInfo")
}

pub unsafe extern "C" fn vkSetDebugUtilsObjectNameEXT(
    device: VkDevice,
    pNameInfo: Option<NonNull<VkDebugUtilsObjectNameInfoEXT>>,
) -> VkResult {
    unimplemented!("vkSetDebugUtilsObjectNameEXT(device, pNameInfo")
}

pub unsafe extern "C" fn vkCreateXlibSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkXlibSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateXlibSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

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

pub unsafe extern "C" fn vkCmdCuLaunchKernelNVX(
    commandBuffer: VkCommandBuffer,
    pLaunchInfo: Option<NonNull<VkCuLaunchInfoNVX>>,
) {
    unimplemented!("vkCmdCuLaunchKernelNVX(commandBuffer, pLaunchInfo")
}

pub unsafe extern "C" fn vkDestroyQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyQueryPool(device, queryPool, pAllocator")
}

pub unsafe extern "C" fn vkQueueBeginDebugUtilsLabelEXT(
    queue: VkQueue,
    pLabelInfo: Option<NonNull<VkDebugUtilsLabelEXT>>,
) {
    unimplemented!("vkQueueBeginDebugUtilsLabelEXT(queue, pLabelInfo")
}

pub unsafe extern "C" fn vkCmdCopyAccelerationStructureToMemoryKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyAccelerationStructureToMemoryInfoKHR>>,
) {
    unimplemented!("vkCmdCopyAccelerationStructureToMemoryKHR(commandBuffer, pInfo")
}

pub unsafe extern "C" fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkGetEventStatus(device, event")
}

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

pub unsafe extern "C" fn vkGetDeviceImageMemoryRequirements(
    device: VkDevice,
    pInfo: Option<NonNull<VkDeviceImageMemoryRequirements>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetDeviceImageMemoryRequirements(device, pInfo, pMemoryRequirements")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceFeatures2(
    physicalDevice: VkPhysicalDevice,
    pFeatures: Option<NonNull<VkPhysicalDeviceFeatures2>>,
) {
    unimplemented!("vkGetPhysicalDeviceFeatures2(physicalDevice, pFeatures")
}

pub unsafe extern "C" fn vkDestroyVideoSessionKHR(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyVideoSessionKHR(device, videoSession, pAllocator")
}

pub unsafe extern "C" fn vkCreateHeadlessSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkHeadlessSurfaceCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateHeadlessSurfaceEXT(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkGetMemorySciBufNV(
    device: VkDevice,
    pGetSciBufInfo: Option<NonNull<VkMemoryGetSciBufInfoNV>>,
    pHandle: Option<NonNull<NvSciBufObj>>,
) -> VkResult {
    unimplemented!("vkGetMemorySciBufNV(device, pGetSciBufInfo, pHandle")
}

pub unsafe extern "C" fn vkGetMemoryWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: Option<NonNull<VkMemoryGetWin32HandleInfoKHR>>,
    pHandle: Option<NonNull<HANDLE>>,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandleKHR(device, pGetWin32HandleInfo, pHandle")
}

pub unsafe extern "C" fn vkImportFenceWin32HandleKHR(
    device: VkDevice,
    pImportFenceWin32HandleInfo: Option<NonNull<VkImportFenceWin32HandleInfoKHR>>,
) -> VkResult {
    unimplemented!("vkImportFenceWin32HandleKHR(device, pImportFenceWin32HandleInfo")
}

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

pub unsafe extern "C" fn vkGetRandROutputDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    dpy: Option<NonNull<Display>>,
    rrOutput: RROutput,
    pDisplay: Option<NonNull<VkDisplayKHR>>,
) -> VkResult {
    unimplemented!("vkGetRandROutputDisplayEXT(physicalDevice, dpy, rrOutput, pDisplay")
}

pub unsafe extern "C" fn vkGetRefreshCycleDurationGOOGLE(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pDisplayTimingProperties: Option<NonNull<VkRefreshCycleDurationGOOGLE>>,
) -> VkResult {
    unimplemented!("vkGetRefreshCycleDurationGOOGLE(device, swapchain, pDisplayTimingProperties")
}

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

pub unsafe extern "C" fn vkDestroyBufferView(
    device: VkDevice,
    bufferView: VkBufferView,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyBufferView(device, bufferView, pAllocator")
}

pub unsafe extern "C" fn vkGetImageViewHandleNVX(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageViewHandleInfoNVX>>,
) -> u32 {
    unimplemented!("vkGetImageViewHandleNVX(device, pInfo")
}

pub unsafe extern "C" fn vkCreateWin32SurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkWin32SurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateWin32SurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkSetDebugUtilsObjectTagEXT(
    device: VkDevice,
    pTagInfo: Option<NonNull<VkDebugUtilsObjectTagInfoEXT>>,
) -> VkResult {
    unimplemented!("vkSetDebugUtilsObjectTagEXT(device, pTagInfo")
}

pub unsafe extern "C" fn vkCmdSetLineStippleEXT(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
) {
    unimplemented!("vkCmdSetLineStippleEXT(commandBuffer, lineStippleFactor, lineStipplePattern")
}

pub unsafe extern "C" fn vkMergePipelineCaches(
    device: VkDevice,
    dstCache: VkPipelineCache,
    srcCacheCount: u32,
    pSrcCaches: Option<NonNull<VkPipelineCache>>,
) -> VkResult {
    unimplemented!("vkMergePipelineCaches(device, dstCache, srcCacheCount, pSrcCaches")
}

pub unsafe extern "C" fn vkImportSemaphoreSciSyncObjNV(
    device: VkDevice,
    pImportSemaphoreSciSyncInfo: Option<NonNull<VkImportSemaphoreSciSyncInfoNV>>,
) -> VkResult {
    unimplemented!("vkImportSemaphoreSciSyncObjNV(device, pImportSemaphoreSciSyncInfo")
}

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

pub unsafe extern "C" fn vkMergeValidationCachesEXT(
    device: VkDevice,
    dstCache: VkValidationCacheEXT,
    srcCacheCount: u32,
    pSrcCaches: Option<NonNull<VkValidationCacheEXT>>,
) -> VkResult {
    unimplemented!("vkMergeValidationCachesEXT(device, dstCache, srcCacheCount, pSrcCaches")
}

pub unsafe extern "C" fn vkImportFenceFdKHR(
    device: VkDevice,
    pImportFenceFdInfo: Option<NonNull<VkImportFenceFdInfoKHR>>,
) -> VkResult {
    unimplemented!("vkImportFenceFdKHR(device, pImportFenceFdInfo")
}

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

pub unsafe extern "C" fn vkCmdSetDepthClipNegativeOneToOneEXT(
    commandBuffer: VkCommandBuffer,
    negativeOneToOne: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClipNegativeOneToOneEXT(commandBuffer, negativeOneToOne")
}

pub unsafe extern "C" fn vkGetDescriptorSetLayoutBindingOffsetEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    binding: u32,
    pOffset: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!("vkGetDescriptorSetLayoutBindingOffsetEXT(device, layout, binding, pOffset")
}

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

pub unsafe extern "C" fn vkGetFenceWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: Option<NonNull<VkFenceGetWin32HandleInfoKHR>>,
    pHandle: Option<NonNull<HANDLE>>,
) -> VkResult {
    unimplemented!("vkGetFenceWin32HandleKHR(device, pGetWin32HandleInfo, pHandle")
}

pub unsafe extern "C" fn vkImportFenceSciSyncObjNV(
    device: VkDevice,
    pImportFenceSciSyncInfo: Option<NonNull<VkImportFenceSciSyncInfoNV>>,
) -> VkResult {
    unimplemented!("vkImportFenceSciSyncObjNV(device, pImportFenceSciSyncInfo")
}

pub unsafe extern "C" fn vkGetWinrtDisplayNV(
    physicalDevice: VkPhysicalDevice,
    deviceRelativeId: u32,
    pDisplay: Option<NonNull<VkDisplayKHR>>,
) -> VkResult {
    unimplemented!("vkGetWinrtDisplayNV(physicalDevice, deviceRelativeId, pDisplay")
}

pub unsafe extern "C" fn vkCmdSubpassShadingHUAWEI(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdSubpassShadingHUAWEI(commandBuffer")
}

pub unsafe extern "C" fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult {
    unimplemented!("vkGetFenceStatus(device, fence")
}

pub unsafe extern "C" fn vkGetSwapchainStatusKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkGetSwapchainStatusKHR(device, swapchain")
}

pub unsafe extern "C" fn vkGetDeviceGroupPresentCapabilitiesKHR(
    device: VkDevice,
    pDeviceGroupPresentCapabilities: Option<NonNull<VkDeviceGroupPresentCapabilitiesKHR>>,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupPresentCapabilitiesKHR(device, pDeviceGroupPresentCapabilities")
}

pub unsafe extern "C" fn vkCmdSetDepthBoundsTestEnable(
    commandBuffer: VkCommandBuffer,
    depthBoundsTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthBoundsTestEnable(commandBuffer, depthBoundsTestEnable")
}

pub unsafe extern "C" fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult {
    unimplemented!("vkResetEvent(device, event")
}

pub unsafe extern "C" fn vkCmdSetViewportWScalingEnableNV(
    commandBuffer: VkCommandBuffer,
    viewportWScalingEnable: VkBool32,
) {
    unimplemented!("vkCmdSetViewportWScalingEnableNV(commandBuffer, viewportWScalingEnable")
}

pub unsafe extern "C" fn vkCmdBlitImage2(
    commandBuffer: VkCommandBuffer,
    pBlitImageInfo: Option<NonNull<VkBlitImageInfo2>>,
) {
    unimplemented!("vkCmdBlitImage2(commandBuffer, pBlitImageInfo")
}

pub unsafe extern "C" fn vkCmdOpticalFlowExecuteNV(
    commandBuffer: VkCommandBuffer,
    session: VkOpticalFlowSessionNV,
    pExecuteInfo: Option<NonNull<VkOpticalFlowExecuteInfoNV>>,
) {
    unimplemented!("vkCmdOpticalFlowExecuteNV(commandBuffer, session, pExecuteInfo")
}

pub unsafe extern "C" fn vkCmdSetDepthCompareOp(
    commandBuffer: VkCommandBuffer,
    depthCompareOp: VkCompareOp,
) {
    unimplemented!("vkCmdSetDepthCompareOp(commandBuffer, depthCompareOp")
}

pub unsafe extern "C" fn vkGetPipelinePropertiesEXT(
    device: VkDevice,
    pPipelineInfo: Option<NonNull<VkPipelineInfoEXT>>,
    pPipelineProperties: Option<NonNull<VkBaseOutStructure>>,
) -> VkResult {
    unimplemented!("vkGetPipelinePropertiesEXT(device, pPipelineInfo, pPipelineProperties")
}

pub unsafe extern "C" fn vkExportMetalObjectsEXT(
    device: VkDevice,
    pMetalObjectsInfo: Option<NonNull<VkExportMetalObjectsInfoEXT>>,
) {
    unimplemented!("vkExportMetalObjectsEXT(device, pMetalObjectsInfo")
}

pub unsafe extern "C" fn vkRegisterDeviceEventEXT(
    device: VkDevice,
    pDeviceEventInfo: Option<NonNull<VkDeviceEventInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFence: Option<NonNull<VkFence>>,
) -> VkResult {
    unimplemented!("vkRegisterDeviceEventEXT(device, pDeviceEventInfo, pAllocator, pFence")
}

pub unsafe extern "C" fn vkCmdBindDescriptorBuffersEXT(
    commandBuffer: VkCommandBuffer,
    bufferCount: u32,
    pBindingInfos: Option<NonNull<VkDescriptorBufferBindingInfoEXT>>,
) {
    unimplemented!("vkCmdBindDescriptorBuffersEXT(commandBuffer, bufferCount, pBindingInfos")
}

pub unsafe extern "C" fn vkCopyMicromapToMemoryEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyMicromapToMemoryInfoEXT>>,
) -> VkResult {
    unimplemented!("vkCopyMicromapToMemoryEXT(device, deferredOperation, pInfo")
}

pub unsafe extern "C" fn vkReleaseProfilingLockKHR(device: VkDevice) {
    unimplemented!("vkReleaseProfilingLockKHR(device")
}

pub unsafe extern "C" fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult {
    unimplemented!("vkEndCommandBuffer(commandBuffer")
}

pub unsafe extern "C" fn vkCmdSetCoverageToColorLocationNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorLocation: u32,
) {
    unimplemented!("vkCmdSetCoverageToColorLocationNV(commandBuffer, coverageToColorLocation")
}

pub unsafe extern "C" fn vkCmdCopyMemoryToMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyMemoryToMicromapInfoEXT>>,
) {
    unimplemented!("vkCmdCopyMemoryToMicromapEXT(commandBuffer, pInfo")
}

pub unsafe extern "C" fn vkGetDeviceAccelerationStructureCompatibilityKHR(
    device: VkDevice,
    pVersionInfo: Option<NonNull<VkAccelerationStructureVersionInfoKHR>>,
    pCompatibility: Option<NonNull<VkAccelerationStructureCompatibilityKHR>>,
) {
    unimplemented!(
        "vkGetDeviceAccelerationStructureCompatibilityKHR(device, pVersionInfo, pCompatibility"
    )
}

pub unsafe extern "C" fn vkCmdBindPipeline(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
) {
    unimplemented!("vkCmdBindPipeline(commandBuffer, pipelineBindPoint, pipeline")
}

pub unsafe extern "C" fn vkGetImageDrmFormatModifierPropertiesEXT(
    device: VkDevice,
    image: VkImage,
    pProperties: Option<NonNull<VkImageDrmFormatModifierPropertiesEXT>>,
) -> VkResult {
    unimplemented!("vkGetImageDrmFormatModifierPropertiesEXT(device, image, pProperties")
}

pub unsafe extern "C" fn vkCmdResolveImage2(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: Option<NonNull<VkResolveImageInfo2>>,
) {
    unimplemented!("vkCmdResolveImage2(commandBuffer, pResolveImageInfo")
}

pub unsafe extern "C" fn vkCmdCopyMicromapEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyMicromapInfoEXT>>,
) {
    unimplemented!("vkCmdCopyMicromapEXT(commandBuffer, pInfo")
}

pub unsafe extern "C" fn vkQueueWaitIdle(queue: VkQueue) -> VkResult {
    unimplemented!("vkQueueWaitIdle(queue")
}

pub unsafe extern "C" fn vkCmdSetRepresentativeFragmentTestEnableNV(
    commandBuffer: VkCommandBuffer,
    representativeFragmentTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetRepresentativeFragmentTestEnableNV(commandBuffer, representativeFragmentTestEnable")
}

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

pub unsafe extern "C" fn vkUninitializePerformanceApiINTEL(device: VkDevice) {
    unimplemented!("vkUninitializePerformanceApiINTEL(device")
}

pub unsafe extern "C" fn vkAcquireImageANDROID(
    device: VkDevice,
    image: VkImage,
    nativeFenceFd: int,
    semaphore: VkSemaphore,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkAcquireImageANDROID(device, image, nativeFenceFd, semaphore, fence")
}

pub unsafe extern "C" fn vkCmdSetPerformanceOverrideINTEL(
    commandBuffer: VkCommandBuffer,
    pOverrideInfo: Option<NonNull<VkPerformanceOverrideInfoINTEL>>,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceOverrideINTEL(commandBuffer, pOverrideInfo")
}
pub unsafe extern "C" fn vkGetDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: Option<NonNull<VkDisplayKHR>>,
) -> VkResult {
    unimplemented!("vkGetDrmDisplayEXT(physicalDevice, drmFd, connectorId, display")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    display: Option<NonNull<wl_display>>,
) -> VkBool32 {
    unimplemented!("vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice, queueFamilyIndex, display")
}

pub unsafe extern "C" fn vkDebugMarkerSetObjectNameEXT(
    device: VkDevice,
    pNameInfo: Option<NonNull<VkDebugMarkerObjectNameInfoEXT>>,
) -> VkResult {
    unimplemented!("vkDebugMarkerSetObjectNameEXT(device, pNameInfo")
}

pub unsafe extern "C" fn vkAcquireXlibDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    dpy: Option<NonNull<Display>>,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireXlibDisplayEXT(physicalDevice, dpy, display")
}

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

pub unsafe extern "C" fn vkBindImageMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: Option<NonNull<VkBindImageMemoryInfo>>,
) -> VkResult {
    unimplemented!("vkBindImageMemory2(device, bindInfoCount, pBindInfos")
}

pub unsafe extern "C" fn vkDisplayPowerControlEXT(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayPowerInfo: Option<NonNull<VkDisplayPowerInfoEXT>>,
) -> VkResult {
    unimplemented!("vkDisplayPowerControlEXT(device, display, pDisplayPowerInfo")
}

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

pub unsafe extern "C" fn vkGetDynamicRenderingTilePropertiesQCOM(
    device: VkDevice,
    pRenderingInfo: Option<NonNull<VkRenderingInfo>>,
    pProperties: Option<NonNull<VkTilePropertiesQCOM>>,
) -> VkResult {
    unimplemented!("vkGetDynamicRenderingTilePropertiesQCOM(device, pRenderingInfo, pProperties")
}

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

pub unsafe extern "C" fn vkCmdBeginQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
) {
    unimplemented!("vkCmdBeginQuery(commandBuffer, queryPool, query, flags")
}

pub unsafe extern "C" fn vkReleasePerformanceConfigurationINTEL(
    device: VkDevice,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!("vkReleasePerformanceConfigurationINTEL(device, configuration")
}

pub unsafe extern "C" fn vkFlushMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: Option<NonNull<VkMappedMemoryRange>>,
) -> VkResult {
    unimplemented!("vkFlushMappedMemoryRanges(device, memoryRangeCount, pMemoryRanges")
}

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

pub unsafe extern "C" fn vkCmdCopyMicromapToMemoryEXT(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyMicromapToMemoryInfoEXT>>,
) {
    unimplemented!("vkCmdCopyMicromapToMemoryEXT(commandBuffer, pInfo")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayPlaneProperties2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayPlaneProperties2KHR(physicalDevice, pPropertyCount, pProperties"
    )
}

pub unsafe extern "C" fn vkGetSemaphoreWin32HandleKHR(
    device: VkDevice,
    pGetWin32HandleInfo: Option<NonNull<VkSemaphoreGetWin32HandleInfoKHR>>,
    pHandle: Option<NonNull<HANDLE>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreWin32HandleKHR(device, pGetWin32HandleInfo, pHandle")
}

pub unsafe extern "C" fn vkImportSemaphoreWin32HandleKHR(
    device: VkDevice,
    pImportSemaphoreWin32HandleInfo: Option<NonNull<VkImportSemaphoreWin32HandleInfoKHR>>,
) -> VkResult {
    unimplemented!("vkImportSemaphoreWin32HandleKHR(device, pImportSemaphoreWin32HandleInfo")
}

pub unsafe extern "C" fn vkCmdEndVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pEndCodingInfo: Option<NonNull<VkVideoEndCodingInfoKHR>>,
) {
    unimplemented!("vkCmdEndVideoCodingKHR(commandBuffer, pEndCodingInfo")
}

pub unsafe extern "C" fn vkCreateShadersEXT(
    device: VkDevice,
    createInfoCount: u32,
    pCreateInfos: Option<NonNull<VkShaderCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pShaders: Option<NonNull<VkShaderEXT>>,
) -> VkResult {
    unimplemented!("vkCreateShadersEXT(device, createInfoCount, pCreateInfos, pAllocator, pShaders")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceProperties2(
    physicalDevice: VkPhysicalDevice,
    pProperties: Option<NonNull<VkPhysicalDeviceProperties2>>,
) {
    unimplemented!("vkGetPhysicalDeviceProperties2(physicalDevice, pProperties")
}

pub unsafe extern "C" fn vkDebugMarkerSetObjectTagEXT(
    device: VkDevice,
    pTagInfo: Option<NonNull<VkDebugMarkerObjectTagInfoEXT>>,
) -> VkResult {
    unimplemented!("vkDebugMarkerSetObjectTagEXT(device, pTagInfo")
}

pub unsafe extern "C" fn vkCmdSetCoverageReductionModeNV(
    commandBuffer: VkCommandBuffer,
    coverageReductionMode: VkCoverageReductionModeNV,
) {
    unimplemented!("vkCmdSetCoverageReductionModeNV(commandBuffer, coverageReductionMode")
}

pub unsafe extern "C" fn vkGetSamplerOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkSamplerCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetSamplerOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}

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

pub unsafe extern "C" fn vkCmdEncodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pEncodeInfo: Option<NonNull<VkVideoEncodeInfoKHR>>,
) {
    unimplemented!("vkCmdEncodeVideoKHR(commandBuffer, pEncodeInfo")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceSciBufAttributesNV(
    physicalDevice: VkPhysicalDevice,
    pAttributes: NvSciBufAttrList,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSciBufAttributesNV(physicalDevice, pAttributes")
}

pub unsafe extern "C" fn vkGetSwapchainCounterEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    counter: VkSurfaceCounterFlagBitsEXT,
    pCounterValue: Option<NonNull<u64>>,
) -> VkResult {
    unimplemented!("vkGetSwapchainCounterEXT(device, swapchain, counter, pCounterValue")
}

pub unsafe extern "C" fn vkSetPrivateData(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
) -> VkResult {
    unimplemented!("vkSetPrivateData(device, objectType, objectHandle, privateDataSlot, data")
}

pub unsafe extern "C" fn vkCmdBindIndexBuffer(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    indexType: VkIndexType,
) {
    unimplemented!("vkCmdBindIndexBuffer(commandBuffer, buffer, offset, indexType")
}

pub unsafe extern "C" fn vkCmdDrawIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawIndirect(commandBuffer, buffer, offset, drawCount, stride")
}

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

pub unsafe extern "C" fn vkCmdSetViewport(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: Option<NonNull<VkViewport>>,
) {
    unimplemented!("vkCmdSetViewport(commandBuffer, firstViewport, viewportCount, pViewports")
}

pub unsafe extern "C" fn vkCmdSetSampleMaskEXT(
    commandBuffer: VkCommandBuffer,
    samples: VkSampleCountFlagBits,
    pSampleMask: Option<NonNull<VkSampleMask>>,
) {
    unimplemented!("vkCmdSetSampleMaskEXT(commandBuffer, samples, pSampleMask")
}

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

pub unsafe extern "C" fn vkCreateDirectFBSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkDirectFBSurfaceCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateDirectFBSurfaceEXT(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkCmdSetStencilWriteMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    writeMask: u32,
) {
    unimplemented!("vkCmdSetStencilWriteMask(commandBuffer, faceMask, writeMask")
}

pub unsafe extern "C" fn vkCreateImagePipeSurfaceFUCHSIA(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkImagePipeSurfaceCreateInfoFUCHSIA>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateImagePipeSurfaceFUCHSIA(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkCreateWaylandSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkWaylandSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateWaylandSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkSetLocalDimmingAMD(
    device: VkDevice,
    swapChain: VkSwapchainKHR,
    localDimmingEnable: VkBool32,
) {
    unimplemented!("vkSetLocalDimmingAMD(device, swapChain, localDimmingEnable")
}

pub unsafe extern "C" fn vkCmdWriteTimestamp2(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdWriteTimestamp2(commandBuffer, stage, queryPool, query")
}

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

pub unsafe extern "C" fn vkEnumerateInstanceLayerProperties(
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkLayerProperties>>,
) -> VkResult {
    unimplemented!("vkEnumerateInstanceLayerProperties(pPropertyCount, pProperties")
}

pub unsafe extern "C" fn vkDestroyDeferredOperationKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDeferredOperationKHR(device, operation, pAllocator")
}

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

pub unsafe extern "C" fn vkGetDescriptorSetLayoutSizeEXT(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    pLayoutSizeInBytes: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!("vkGetDescriptorSetLayoutSizeEXT(device, layout, pLayoutSizeInBytes")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties2(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: Option<NonNull<VkFormatProperties2>>,
) {
    unimplemented!("vkGetPhysicalDeviceFormatProperties2(physicalDevice, format, pFormatProperties")
}

pub unsafe extern "C" fn vkCmdInsertDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: Option<NonNull<VkDebugUtilsLabelEXT>>,
) {
    unimplemented!("vkCmdInsertDebugUtilsLabelEXT(commandBuffer, pLabelInfo")
}

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

pub unsafe extern "C" fn vkCmdSetExtraPrimitiveOverestimationSizeEXT(
    commandBuffer: VkCommandBuffer,
    extraPrimitiveOverestimationSize: f32,
) {
    unimplemented!("vkCmdSetExtraPrimitiveOverestimationSizeEXT(commandBuffer, extraPrimitiveOverestimationSize")
}

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

pub unsafe extern "C" fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdDebugMarkerEndEXT(commandBuffer")
}

pub unsafe extern "C" fn vkCmdSetFrontFace(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace) {
    unimplemented!("vkCmdSetFrontFace(commandBuffer, frontFace")
}

pub unsafe extern "C" fn vkCreateViSurfaceNN(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkViSurfaceCreateInfoNN>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateViSurfaceNN(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkGetMemoryAndroidHardwareBufferANDROID(
    device: VkDevice,
    pInfo: Option<NonNull<VkMemoryGetAndroidHardwareBufferInfoANDROID>>,
    pBuffer: Option<NonNull<AHardwareBuffer>>,
) -> VkResult {
    unimplemented!("vkGetMemoryAndroidHardwareBufferANDROID(device, pInfo, pBuffer")
}

pub unsafe extern "C" fn vkCmdSetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: Option<NonNull<VkDependencyInfo>>,
) {
    unimplemented!("vkCmdSetEvent2(commandBuffer, event, pDependencyInfo")
}

pub unsafe extern "C" fn vkCopyAccelerationStructureKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyAccelerationStructureInfoKHR>>,
) -> VkResult {
    unimplemented!("vkCopyAccelerationStructureKHR(device, deferredOperation, pInfo")
}

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

pub unsafe extern "C" fn vkCreateMetalSurfaceEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkMetalSurfaceCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateMetalSurfaceEXT(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkCmdDrawMeshTasksNV(
    commandBuffer: VkCommandBuffer,
    taskCount: u32,
    firstTask: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksNV(commandBuffer, taskCount, firstTask")
}

pub unsafe extern "C" fn vkDestroySamplerYcbcrConversion(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySamplerYcbcrConversion(device, ycbcrConversion, pAllocator")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceScreenPresentationSupportQNX(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    window: Option<NonNull<_screen_window>>,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceScreenPresentationSupportQNX(physicalDevice, queueFamilyIndex, window"
    )
}

pub unsafe extern "C" fn vkAcquireWinrtDisplayNV(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireWinrtDisplayNV(physicalDevice, display")
}

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

pub unsafe extern "C" fn vkCmdBindInvocationMaskHUAWEI(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    unimplemented!("vkCmdBindInvocationMaskHUAWEI(commandBuffer, imageView, imageLayout")
}

pub unsafe extern "C" fn vkGetShaderBinaryDataEXT(
    device: VkDevice,
    shader: VkShaderEXT,
    pDataSize: Option<NonNull<isize>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetShaderBinaryDataEXT(device, shader, pDataSize, pData")
}

pub unsafe extern "C" fn vkGetMemoryFdKHR(
    device: VkDevice,
    pGetFdInfo: Option<NonNull<VkMemoryGetFdInfoKHR>>,
    pFd: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!("vkGetMemoryFdKHR(device, pGetFdInfo, pFd")
}

pub unsafe extern "C" fn vkGetSemaphoreZirconHandleFUCHSIA(
    device: VkDevice,
    pGetZirconHandleInfo: Option<NonNull<VkSemaphoreGetZirconHandleInfoFUCHSIA>>,
    pZirconHandle: Option<NonNull<zx_handle_t>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreZirconHandleFUCHSIA(device, pGetZirconHandleInfo, pZirconHandle")
}

pub unsafe extern "C" fn vkDestroyDebugUtilsMessengerEXT(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDebugUtilsMessengerEXT(instance, messenger, pAllocator")
}

pub unsafe extern "C" fn vkCmdSetSampleLocationsEXT(
    commandBuffer: VkCommandBuffer,
    pSampleLocationsInfo: Option<NonNull<VkSampleLocationsInfoEXT>>,
) {
    unimplemented!("vkCmdSetSampleLocationsEXT(commandBuffer, pSampleLocationsInfo")
}

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

pub unsafe extern "C" fn vkGetDescriptorEXT(
    device: VkDevice,
    pDescriptorInfo: Option<NonNull<VkDescriptorGetInfoEXT>>,
    dataSize: isize,
    pDescriptor: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!("vkGetDescriptorEXT(device, pDescriptorInfo, dataSize, pDescriptor")
}

pub unsafe extern "C" fn vkCmdDecodeVideoKHR(
    commandBuffer: VkCommandBuffer,
    pDecodeInfo: Option<NonNull<VkVideoDecodeInfoKHR>>,
) {
    unimplemented!("vkCmdDecodeVideoKHR(commandBuffer, pDecodeInfo")
}

pub unsafe extern "C" fn vkGetDeviceMemoryCommitment(
    device: VkDevice,
    memory: VkDeviceMemory,
    pCommittedMemoryInBytes: Option<NonNull<VkDeviceSize>>,
) {
    unimplemented!("vkGetDeviceMemoryCommitment(device, memory, pCommittedMemoryInBytes")
}

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

pub unsafe extern "C" fn vkReleaseFullScreenExclusiveModeEXT(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
) -> VkResult {
    unimplemented!("vkReleaseFullScreenExclusiveModeEXT(device, swapchain")
}

pub unsafe extern "C" fn vkCmdSetViewportWithCount(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: Option<NonNull<VkViewport>>,
) {
    unimplemented!("vkCmdSetViewportWithCount(commandBuffer, viewportCount, pViewports")
}

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

pub unsafe extern "C" fn vkCreateValidationCacheEXT(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkValidationCacheCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pValidationCache: Option<NonNull<VkValidationCacheEXT>>,
) -> VkResult {
    unimplemented!("vkCreateValidationCacheEXT(device, pCreateInfo, pAllocator, pValidationCache")
}

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

pub unsafe extern "C" fn vkCmdPreprocessGeneratedCommandsNV(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: Option<NonNull<VkGeneratedCommandsInfoNV>>,
) {
    unimplemented!("vkCmdPreprocessGeneratedCommandsNV(commandBuffer, pGeneratedCommandsInfo")
}

pub unsafe extern "C" fn vkCreateAndroidSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkAndroidSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateAndroidSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkCmdSetCoverageModulationModeNV(
    commandBuffer: VkCommandBuffer,
    coverageModulationMode: VkCoverageModulationModeNV,
) {
    unimplemented!("vkCmdSetCoverageModulationModeNV(commandBuffer, coverageModulationMode")
}

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

pub unsafe extern "C" fn vkCmdCopyImageToBuffer2(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: Option<NonNull<VkCopyImageToBufferInfo2>>,
) {
    unimplemented!("vkCmdCopyImageToBuffer2(commandBuffer, pCopyImageToBufferInfo")
}

pub unsafe extern "C" fn vkCmdSetLineStippleEnableEXT(
    commandBuffer: VkCommandBuffer,
    stippledLineEnable: VkBool32,
) {
    unimplemented!("vkCmdSetLineStippleEnableEXT(commandBuffer, stippledLineEnable")
}

pub unsafe extern "C" fn vkDestroyCuModuleNVX(
    device: VkDevice,
    module: VkCuModuleNVX,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyCuModuleNVX(device, module, pAllocator")
}

pub unsafe extern "C" fn vkGetMemoryRemoteAddressNV(
    device: VkDevice,
    pMemoryGetRemoteAddressInfo: Option<NonNull<VkMemoryGetRemoteAddressInfoNV>>,
    pAddress: Option<NonNull<VkRemoteAddressNV>>,
) -> VkResult {
    unimplemented!("vkGetMemoryRemoteAddressNV(device, pMemoryGetRemoteAddressInfo, pAddress")
}

pub unsafe extern "C" fn vkCmdSetPerformanceMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: Option<NonNull<VkPerformanceMarkerInfoINTEL>>,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceMarkerINTEL(commandBuffer, pMarkerInfo")
}

pub unsafe extern "C" fn vkCmdDispatchIndirect(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
) {
    unimplemented!("vkCmdDispatchIndirect(commandBuffer, buffer, offset")
}

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

pub unsafe extern "C" fn vkQueueSubmit2(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: Option<NonNull<VkSubmitInfo2>>,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueSubmit2(queue, submitCount, pSubmits, fence")
}

pub unsafe extern "C" fn vkCmdDebugMarkerInsertEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: Option<NonNull<VkDebugMarkerMarkerInfoEXT>>,
) {
    unimplemented!("vkCmdDebugMarkerInsertEXT(commandBuffer, pMarkerInfo")
}

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

pub unsafe extern "C" fn vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32) {
    unimplemented!("vkCmdSetDeviceMask(commandBuffer, deviceMask")
}

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

pub unsafe extern "C" fn vkCmdSetRasterizationSamplesEXT(
    commandBuffer: VkCommandBuffer,
    rasterizationSamples: VkSampleCountFlagBits,
) {
    unimplemented!("vkCmdSetRasterizationSamplesEXT(commandBuffer, rasterizationSamples")
}

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

pub unsafe extern "C" fn vkCmdSetShadingRateImageEnableNV(
    commandBuffer: VkCommandBuffer,
    shadingRateImageEnable: VkBool32,
) {
    unimplemented!("vkCmdSetShadingRateImageEnableNV(commandBuffer, shadingRateImageEnable")
}

pub unsafe extern "C" fn vkGetSemaphoreFdKHR(
    device: VkDevice,
    pGetFdInfo: Option<NonNull<VkSemaphoreGetFdInfoKHR>>,
    pFd: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreFdKHR(device, pGetFdInfo, pFd")
}

pub unsafe extern "C" fn vkGetDescriptorSetHostMappingVALVE(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    ppData: Option<NonNull<NonNull<std::ffi::c_void>>>,
) {
    unimplemented!("vkGetDescriptorSetHostMappingVALVE(device, descriptorSet, ppData")
}

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

pub unsafe extern "C" fn vkGetDeviceQueue2(
    device: VkDevice,
    pQueueInfo: Option<NonNull<VkDeviceQueueInfo2>>,
    pQueue: Option<NonNull<VkQueue>>,
) {
    unimplemented!("vkGetDeviceQueue2(device, pQueueInfo, pQueue")
}

pub unsafe extern "C" fn vkCreateRenderPass2(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkRenderPassCreateInfo2>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pRenderPass: Option<NonNull<VkRenderPass>>,
) -> VkResult {
    unimplemented!("vkCreateRenderPass2(device, pCreateInfo, pAllocator, pRenderPass")
}

pub unsafe extern "C" fn vkCmdDispatch(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
) {
    unimplemented!("vkCmdDispatch(commandBuffer, groupCountX, groupCountY, groupCountZ")
}

pub unsafe extern "C" fn vkDestroyAccelerationStructureNV(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyAccelerationStructureNV(device, accelerationStructure, pAllocator")
}

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

pub unsafe extern "C" fn vkCmdSetPrimitiveTopology(
    commandBuffer: VkCommandBuffer,
    primitiveTopology: VkPrimitiveTopology,
) {
    unimplemented!("vkCmdSetPrimitiveTopology(commandBuffer, primitiveTopology")
}

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

pub unsafe extern "C" fn vkImportSemaphoreZirconHandleFUCHSIA(
    device: VkDevice,
    pImportSemaphoreZirconHandleInfo: Option<NonNull<VkImportSemaphoreZirconHandleInfoFUCHSIA>>,
) -> VkResult {
    unimplemented!("vkImportSemaphoreZirconHandleFUCHSIA(device, pImportSemaphoreZirconHandleInfo")
}

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

pub unsafe extern "C" fn vkGetRayTracingShaderGroupStackSizeKHR(
    device: VkDevice,
    pipeline: VkPipeline,
    group: u32,
    groupShader: VkShaderGroupShaderKHR,
) -> VkDeviceSize {
    unimplemented!("vkGetRayTracingShaderGroupStackSizeKHR(device, pipeline, group, groupShader")
}

pub unsafe extern "C" fn vkResetDescriptorPool(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    flags: VkDescriptorPoolResetFlags,
) -> VkResult {
    unimplemented!("vkResetDescriptorPool(device, descriptorPool, flags")
}

pub unsafe extern "C" fn vkGetPipelineCacheData(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pDataSize: Option<NonNull<isize>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetPipelineCacheData(device, pipelineCache, pDataSize, pData")
}

pub unsafe extern "C" fn vkGetFenceSciSyncFenceNV(
    device: VkDevice,
    pGetSciSyncHandleInfo: Option<NonNull<VkFenceGetSciSyncInfoNV>>,
    pHandle: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetFenceSciSyncFenceNV(device, pGetSciSyncHandleInfo, pHandle")
}

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

pub unsafe extern "C" fn vkGetBufferCollectionPropertiesFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pProperties: Option<NonNull<VkBufferCollectionPropertiesFUCHSIA>>,
) -> VkResult {
    unimplemented!("vkGetBufferCollectionPropertiesFUCHSIA(device, collection, pProperties")
}

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

pub unsafe extern "C" fn vkCmdFillBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
) {
    unimplemented!("vkCmdFillBuffer(commandBuffer, dstBuffer, dstOffset, size, data")
}

pub unsafe extern "C" fn vkCmdSetBlendConstants(
    commandBuffer: VkCommandBuffer,
    blendConstants: *const f32,
) {
    let _ = unsafe { std::slice::from_raw_parts(blendConstants, 4) };
    unimplemented!("vkCmdSetBlendConstants(commandBuffer, blendConstants")
}

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

pub unsafe extern "C" fn vkCreateEvent(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkEventCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pEvent: Option<NonNull<VkEvent>>,
) -> VkResult {
    unimplemented!("vkCreateEvent(device, pCreateInfo, pAllocator, pEvent")
}

pub unsafe extern "C" fn vkBindBufferMemory2(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: Option<NonNull<VkBindBufferMemoryInfo>>,
) -> VkResult {
    unimplemented!("vkBindBufferMemory2(device, bindInfoCount, pBindInfos")
}

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

pub unsafe extern "C" fn vkCmdEndConditionalRenderingEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndConditionalRenderingEXT(commandBuffer")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceMultisamplePropertiesEXT(
    physicalDevice: VkPhysicalDevice,
    samples: VkSampleCountFlagBits,
    pMultisampleProperties: Option<NonNull<VkMultisamplePropertiesEXT>>,
) {
    unimplemented!("vkGetPhysicalDeviceMultisamplePropertiesEXT(physicalDevice, samples, pMultisampleProperties")
}

pub unsafe extern "C" fn vkGetImageSubresourceLayout2EXT(
    device: VkDevice,
    image: VkImage,
    pSubresource: Option<NonNull<VkImageSubresource2EXT>>,
    pLayout: Option<NonNull<VkSubresourceLayout2EXT>>,
) {
    unimplemented!("vkGetImageSubresourceLayout2EXT(device, image, pSubresource, pLayout")
}

pub unsafe extern "C" fn vkDestroyCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyCommandPool(device, commandPool, pAllocator")
}

pub unsafe extern "C" fn vkCreateDisplayPlaneSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkDisplaySurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    unimplemented!("vkCreateDisplayPlaneSurfaceKHR(instance, pCreateInfo, pAllocator, pSurface")
}

pub unsafe extern "C" fn vkCmdCopyMemoryIndirectNV(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdCopyMemoryIndirectNV(commandBuffer, copyBufferAddress, copyCount, stride")
}

pub unsafe extern "C" fn vkGetPipelineExecutableStatisticsKHR(
    device: VkDevice,
    pExecutableInfo: Option<NonNull<VkPipelineExecutableInfoKHR>>,
    pStatisticCount: Option<NonNull<u32>>,
    pStatistics: Option<NonNull<VkPipelineExecutableStatisticKHR>>,
) -> VkResult {
    unimplemented!("vkGetPipelineExecutableStatisticsKHR(device, pExecutableInfo, pStatisticCount, pStatistics")
}

pub unsafe extern "C" fn vkCmdSetFragmentShadingRateKHR(
    commandBuffer: VkCommandBuffer,
    pFragmentSize: Option<NonNull<VkExtent2D>>,
    combinerOps: *const VkFragmentShadingRateCombinerOpKHR,
) {
    let _ = unsafe { std::slice::from_raw_parts(combinerOps, 2) };
    unimplemented!("vkCmdSetFragmentShadingRateKHR(commandBuffer, pFragmentSize, combinerOps")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayProperties2KHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayProperties2KHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayProperties2KHR(physicalDevice, pPropertyCount, pProperties"
    )
}

pub unsafe extern "C" fn vkImportFenceSciSyncFenceNV(
    device: VkDevice,
    pImportFenceSciSyncInfo: Option<NonNull<VkImportFenceSciSyncInfoNV>>,
) -> VkResult {
    unimplemented!("vkImportFenceSciSyncFenceNV(device, pImportFenceSciSyncInfo")
}

pub unsafe extern "C" fn vkCmdSetDepthTestEnable(
    commandBuffer: VkCommandBuffer,
    depthTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthTestEnable(commandBuffer, depthTestEnable")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceVideoCapabilitiesKHR(
    physicalDevice: VkPhysicalDevice,
    pVideoProfile: Option<NonNull<VkVideoProfileInfoKHR>>,
    pCapabilities: Option<NonNull<VkVideoCapabilitiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceVideoCapabilitiesKHR(physicalDevice, pVideoProfile, pCapabilities"
    )
}

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

pub unsafe extern "C" fn vkDestroyFence(
    device: VkDevice,
    fence: VkFence,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyFence(device, fence, pAllocator")
}

pub unsafe extern "C" fn vkCmdSetPerformanceStreamMarkerINTEL(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: Option<NonNull<VkPerformanceStreamMarkerInfoINTEL>>,
) -> VkResult {
    unimplemented!("vkCmdSetPerformanceStreamMarkerINTEL(commandBuffer, pMarkerInfo")
}

pub unsafe extern "C" fn vkCmdSetAlphaToOneEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToOneEnable: VkBool32,
) {
    unimplemented!("vkCmdSetAlphaToOneEnableEXT(commandBuffer, alphaToOneEnable")
}

pub unsafe extern "C" fn vkCompileDeferredNV(
    device: VkDevice,
    pipeline: VkPipeline,
    shader: u32,
) -> VkResult {
    unimplemented!("vkCompileDeferredNV(device, pipeline, shader")
}

pub unsafe extern "C" fn vkGetImageMemoryRequirements2(
    device: VkDevice,
    pInfo: Option<NonNull<VkImageMemoryRequirementsInfo2>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetImageMemoryRequirements2(device, pInfo, pMemoryRequirements")
}

pub unsafe extern "C" fn vkCmdSetPatchControlPointsEXT(
    commandBuffer: VkCommandBuffer,
    patchControlPoints: u32,
) {
    unimplemented!("vkCmdSetPatchControlPointsEXT(commandBuffer, patchControlPoints")
}

pub unsafe extern "C" fn vkGetDeviceGroupSurfacePresentModes2EXT(
    device: VkDevice,
    pSurfaceInfo: Option<NonNull<VkPhysicalDeviceSurfaceInfo2KHR>>,
    pModes: Option<NonNull<VkDeviceGroupPresentModeFlagsKHR>>,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupSurfacePresentModes2EXT(device, pSurfaceInfo, pModes")
}

pub unsafe extern "C" fn vkCmdSetSampleLocationsEnableEXT(
    commandBuffer: VkCommandBuffer,
    sampleLocationsEnable: VkBool32,
) {
    unimplemented!("vkCmdSetSampleLocationsEnableEXT(commandBuffer, sampleLocationsEnable")
}

pub unsafe extern "C" fn vkGetAndroidHardwareBufferPropertiesANDROID(
    device: VkDevice,
    buffer: Option<NonNull<AHardwareBuffer>>,
    pProperties: Option<NonNull<VkAndroidHardwareBufferPropertiesANDROID>>,
) -> VkResult {
    unimplemented!("vkGetAndroidHardwareBufferPropertiesANDROID(device, buffer, pProperties")
}

pub unsafe extern "C" fn vkDestroyBufferCollectionFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyBufferCollectionFUCHSIA(device, collection, pAllocator")
}

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

pub unsafe extern "C" fn vkCmdSetColorWriteEnableEXT(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pColorWriteEnables: Option<NonNull<VkBool32>>,
) {
    unimplemented!("vkCmdSetColorWriteEnableEXT(commandBuffer, attachmentCount, pColorWriteEnables")
}

pub unsafe extern "C" fn vkGetGeneratedCommandsMemoryRequirementsNV(
    device: VkDevice,
    pInfo: Option<NonNull<VkGeneratedCommandsMemoryRequirementsInfoNV>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetGeneratedCommandsMemoryRequirementsNV(device, pInfo, pMemoryRequirements")
}

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

pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectEXT(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksIndirectEXT(commandBuffer, buffer, offset, drawCount, stride")
}

pub unsafe extern "C" fn vkGetDeferredOperationResultKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkGetDeferredOperationResultKHR(device, operation")
}

pub unsafe extern "C" fn vkQueueInsertDebugUtilsLabelEXT(
    queue: VkQueue,
    pLabelInfo: Option<NonNull<VkDebugUtilsLabelEXT>>,
) {
    unimplemented!("vkQueueInsertDebugUtilsLabelEXT(queue, pLabelInfo")
}

pub unsafe extern "C" fn vkGetBufferOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkBufferCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetBufferOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}

pub unsafe extern "C" fn vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndDebugUtilsLabelEXT(commandBuffer")
}

pub unsafe extern "C" fn vkCmdEndRendering(commandBuffer: VkCommandBuffer) {
    unimplemented!("vkCmdEndRendering(commandBuffer")
}

pub unsafe extern "C" fn vkGetPerformanceParameterINTEL(
    device: VkDevice,
    parameter: VkPerformanceParameterTypeINTEL,
    pValue: Option<NonNull<VkPerformanceValueINTEL>>,
) -> VkResult {
    unimplemented!("vkGetPerformanceParameterINTEL(device, parameter, pValue")
}

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

pub unsafe extern "C" fn vkAcquireNextImage2KHR(
    device: VkDevice,
    pAcquireInfo: Option<NonNull<VkAcquireNextImageInfoKHR>>,
    pImageIndex: Option<NonNull<u32>>,
) -> VkResult {
    unimplemented!("vkAcquireNextImage2KHR(device, pAcquireInfo, pImageIndex")
}

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

pub unsafe extern "C" fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32) {
    unimplemented!("vkCmdSetLineWidth(commandBuffer, lineWidth")
}

pub unsafe extern "C" fn vkCmdDebugMarkerBeginEXT(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: Option<NonNull<VkDebugMarkerMarkerInfoEXT>>,
) {
    unimplemented!("vkCmdDebugMarkerBeginEXT(commandBuffer, pMarkerInfo")
}

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

pub unsafe extern "C" fn vkCmdSetCullMode(
    commandBuffer: VkCommandBuffer,
    cullMode: VkCullModeFlags,
) {
    unimplemented!("vkCmdSetCullMode(commandBuffer, cullMode")
}

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

pub unsafe extern "C" fn vkCmdSetScissor(
    commandBuffer: VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: Option<NonNull<VkRect2D>>,
) {
    unimplemented!("vkCmdSetScissor(commandBuffer, firstScissor, scissorCount, pScissors")
}

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

pub unsafe extern "C" fn vkGetAccelerationStructureDeviceAddressKHR(
    device: VkDevice,
    pInfo: Option<NonNull<VkAccelerationStructureDeviceAddressInfoKHR>>,
) -> VkDeviceAddress {
    unimplemented!("vkGetAccelerationStructureDeviceAddressKHR(device, pInfo")
}

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

pub unsafe extern "C" fn vkCreateBufferCollectionFUCHSIA(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferCollectionCreateInfoFUCHSIA>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pCollection: Option<NonNull<VkBufferCollectionFUCHSIA>>,
) -> VkResult {
    unimplemented!("vkCreateBufferCollectionFUCHSIA(device, pCreateInfo, pAllocator, pCollection")
}

pub unsafe extern "C" fn vkDestroyAccelerationStructureKHR(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyAccelerationStructureKHR(device, accelerationStructure, pAllocator")
}

pub unsafe extern "C" fn vkCmdBeginVideoCodingKHR(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: Option<NonNull<VkVideoBeginCodingInfoKHR>>,
) {
    unimplemented!("vkCmdBeginVideoCodingKHR(commandBuffer, pBeginInfo")
}

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

pub unsafe extern "C" fn vkCmdBuildMicromapsEXT(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: Option<NonNull<VkMicromapBuildInfoEXT>>,
) {
    unimplemented!("vkCmdBuildMicromapsEXT(commandBuffer, infoCount, pInfos")
}

pub unsafe extern "C" fn vkGetFenceSciSyncObjNV(
    device: VkDevice,
    pGetSciSyncHandleInfo: Option<NonNull<VkFenceGetSciSyncInfoNV>>,
    pHandle: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetFenceSciSyncObjNV(device, pGetSciSyncHandleInfo, pHandle")
}

pub unsafe extern "C" fn vkCmdSetStencilCompareMask(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    compareMask: u32,
) {
    unimplemented!("vkCmdSetStencilCompareMask(commandBuffer, faceMask, compareMask")
}

pub unsafe extern "C" fn vkBindOpticalFlowSessionImageNV(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    bindingPoint: VkOpticalFlowSessionBindingPointNV,
    view: VkImageView,
    layout: VkImageLayout,
) -> VkResult {
    unimplemented!("vkBindOpticalFlowSessionImageNV(device, session, bindingPoint, view, layout")
}

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

pub unsafe extern "C" fn vkGetRenderAreaGranularity(
    device: VkDevice,
    renderPass: VkRenderPass,
    pGranularity: Option<NonNull<VkExtent2D>>,
) {
    unimplemented!("vkGetRenderAreaGranularity(device, renderPass, pGranularity")
}

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

pub unsafe extern "C" fn vkCmdWaitEvents2(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: Option<NonNull<VkEvent>>,
    pDependencyInfos: Option<NonNull<VkDependencyInfo>>,
) {
    unimplemented!("vkCmdWaitEvents2(commandBuffer, eventCount, pEvents, pDependencyInfos")
}

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

pub unsafe extern "C" fn vkGetDeviceMemoryOpaqueCaptureAddress(
    device: VkDevice,
    pInfo: Option<NonNull<VkDeviceMemoryOpaqueCaptureAddressInfo>>,
) -> u64 {
    unimplemented!("vkGetDeviceMemoryOpaqueCaptureAddress(device, pInfo")
}

pub unsafe extern "C" fn vkCmdSetDepthBiasEnable(
    commandBuffer: VkCommandBuffer,
    depthBiasEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthBiasEnable(commandBuffer, depthBiasEnable")
}

pub unsafe extern "C" fn vkSetBufferCollectionImageConstraintsFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pImageConstraintsInfo: Option<NonNull<VkImageConstraintsInfoFUCHSIA>>,
) -> VkResult {
    unimplemented!(
        "vkSetBufferCollectionImageConstraintsFUCHSIA(device, collection, pImageConstraintsInfo"
    )
}

pub unsafe extern "C" fn vkGetImageViewAddressNVX(
    device: VkDevice,
    imageView: VkImageView,
    pProperties: Option<NonNull<VkImageViewAddressPropertiesNVX>>,
) -> VkResult {
    unimplemented!("vkGetImageViewAddressNVX(device, imageView, pProperties")
}

pub unsafe extern "C" fn vkCmdSetDepthClipEnableEXT(
    commandBuffer: VkCommandBuffer,
    depthClipEnable: VkBool32,
) {
    unimplemented!("vkCmdSetDepthClipEnableEXT(commandBuffer, depthClipEnable")
}

pub unsafe extern "C" fn vkCmdSetLogicOpEnableEXT(
    commandBuffer: VkCommandBuffer,
    logicOpEnable: VkBool32,
) {
    unimplemented!("vkCmdSetLogicOpEnableEXT(commandBuffer, logicOpEnable")
}

pub unsafe extern "C" fn vkCmdSetDepthBounds(
    commandBuffer: VkCommandBuffer,
    minDepthBounds: f32,
    maxDepthBounds: f32,
) {
    unimplemented!("vkCmdSetDepthBounds(commandBuffer, minDepthBounds, maxDepthBounds")
}

pub unsafe extern "C" fn vkMapMemory2KHR(
    device: VkDevice,
    pMemoryMapInfo: Option<NonNull<VkMemoryMapInfoKHR>>,
    ppData: Option<NonNull<NonNull<std::ffi::c_void>>>,
) -> VkResult {
    unimplemented!("vkMapMemory2KHR(device, pMemoryMapInfo, ppData")
}

pub unsafe extern "C" fn vkCmdEndRenderPass2(
    commandBuffer: VkCommandBuffer,
    pSubpassEndInfo: Option<NonNull<VkSubpassEndInfo>>,
) {
    unimplemented!("vkCmdEndRenderPass2(commandBuffer, pSubpassEndInfo")
}

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

pub unsafe extern "C" fn vkCmdSetFragmentShadingRateEnumNV(
    commandBuffer: VkCommandBuffer,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: *const VkFragmentShadingRateCombinerOpKHR,
) {
    let _ = unsafe { std::slice::from_raw_parts(combinerOps, 2) };
    unimplemented!("vkCmdSetFragmentShadingRateEnumNV(commandBuffer, shadingRate, combinerOps")
}

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

pub unsafe extern "C" fn vkGetPhysicalDeviceSciSyncAttributesNV(
    physicalDevice: VkPhysicalDevice,
    pSciSyncAttributesInfo: Option<NonNull<VkSciSyncAttributesInfoNV>>,
    pAttributes: NvSciSyncAttrList,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceSciSyncAttributesNV(physicalDevice, pSciSyncAttributesInfo, pAttributes")
}

pub unsafe extern "C" fn vkGetShaderModuleIdentifierEXT(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pIdentifier: Option<NonNull<VkShaderModuleIdentifierEXT>>,
) {
    unimplemented!("vkGetShaderModuleIdentifierEXT(device, shaderModule, pIdentifier")
}

pub unsafe extern "C" fn vkCmdBindShadersEXT(
    commandBuffer: VkCommandBuffer,
    stageCount: u32,
    pStages: Option<NonNull<VkShaderStageFlagBits>>,
    pShaders: Option<NonNull<VkShaderEXT>>,
) {
    unimplemented!("vkCmdBindShadersEXT(commandBuffer, stageCount, pStages, pShaders")
}

pub unsafe extern "C" fn vkCreateDeferredOperationKHR(
    device: VkDevice,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDeferredOperation: Option<NonNull<VkDeferredOperationKHR>>,
) -> VkResult {
    unimplemented!("vkCreateDeferredOperationKHR(device, pAllocator, pDeferredOperation")
}

pub unsafe extern "C" fn vkCmdNextSubpass2(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: Option<NonNull<VkSubpassBeginInfo>>,
    pSubpassEndInfo: Option<NonNull<VkSubpassEndInfo>>,
) {
    unimplemented!("vkCmdNextSubpass2(commandBuffer, pSubpassBeginInfo, pSubpassEndInfo")
}

pub unsafe extern "C" fn vkDeferredOperationJoinKHR(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
) -> VkResult {
    unimplemented!("vkDeferredOperationJoinKHR(device, operation")
}

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

pub unsafe extern "C" fn vkUnmapMemory2KHR(
    device: VkDevice,
    pMemoryUnmapInfo: Option<NonNull<VkMemoryUnmapInfoKHR>>,
) -> VkResult {
    unimplemented!("vkUnmapMemory2KHR(device, pMemoryUnmapInfo")
}

pub unsafe extern "C" fn vkCmdBindShadingRateImageNV(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
) {
    unimplemented!("vkCmdBindShadingRateImageNV(commandBuffer, imageView, imageLayout")
}

pub unsafe extern "C" fn vkCmdBeginQueryIndexedEXT(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
    index: u32,
) {
    unimplemented!("vkCmdBeginQueryIndexedEXT(commandBuffer, queryPool, query, flags, index")
}

pub unsafe extern "C" fn vkDestroyDebugReportCallbackEXT(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyDebugReportCallbackEXT(instance, callback, pAllocator")
}

pub unsafe extern "C" fn vkDestroySemaphore(
    device: VkDevice,
    semaphore: VkSemaphore,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroySemaphore(device, semaphore, pAllocator")
}

pub unsafe extern "C" fn vkCmdSetDiscardRectangleModeEXT(
    commandBuffer: VkCommandBuffer,
    discardRectangleMode: VkDiscardRectangleModeEXT,
) {
    unimplemented!("vkCmdSetDiscardRectangleModeEXT(commandBuffer, discardRectangleMode")
}

pub unsafe extern "C" fn vkCmdResetQueryPool(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    unimplemented!("vkCmdResetQueryPool(commandBuffer, queryPool, firstQuery, queryCount")
}

pub unsafe extern "C" fn vkCreateBufferView(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkBufferViewCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pView: Option<NonNull<VkBufferView>>,
) -> VkResult {
    unimplemented!("vkCreateBufferView(device, pCreateInfo, pAllocator, pView")
}

pub unsafe extern "C" fn vkCmdSetConservativeRasterizationModeEXT(
    commandBuffer: VkCommandBuffer,
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
) {
    unimplemented!(
        "vkCmdSetConservativeRasterizationModeEXT(commandBuffer, conservativeRasterizationMode"
    )
}

pub unsafe extern "C" fn vkCopyMemoryToMicromapEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyMemoryToMicromapInfoEXT>>,
) -> VkResult {
    unimplemented!("vkCopyMemoryToMicromapEXT(device, deferredOperation, pInfo")
}

pub unsafe extern "C" fn vkGetDeviceBufferMemoryRequirements(
    device: VkDevice,
    pInfo: Option<NonNull<VkDeviceBufferMemoryRequirements>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2>>,
) {
    unimplemented!("vkGetDeviceBufferMemoryRequirements(device, pInfo, pMemoryRequirements")
}

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

pub unsafe extern "C" fn vkCmdBeginDebugUtilsLabelEXT(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: Option<NonNull<VkDebugUtilsLabelEXT>>,
) {
    unimplemented!("vkCmdBeginDebugUtilsLabelEXT(commandBuffer, pLabelInfo")
}

pub unsafe extern "C" fn vkCmdCopyMemoryToAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyMemoryToAccelerationStructureInfoKHR>>,
) {
    unimplemented!("vkCmdCopyMemoryToAccelerationStructureKHR(commandBuffer, pInfo")
}

pub unsafe extern "C" fn vkImportSemaphoreFdKHR(
    device: VkDevice,
    pImportSemaphoreFdInfo: Option<NonNull<VkImportSemaphoreFdInfoKHR>>,
) -> VkResult {
    unimplemented!("vkImportSemaphoreFdKHR(device, pImportSemaphoreFdInfo")
}

pub unsafe extern "C" fn vkQueueBindSparse(
    queue: VkQueue,
    bindInfoCount: u32,
    pBindInfo: Option<NonNull<VkBindSparseInfo>>,
    fence: VkFence,
) -> VkResult {
    unimplemented!("vkQueueBindSparse(queue, bindInfoCount, pBindInfo, fence")
}

pub unsafe extern "C" fn vkGetMemoryWin32HandleNV(
    device: VkDevice,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    pHandle: Option<NonNull<HANDLE>>,
) -> VkResult {
    unimplemented!("vkGetMemoryWin32HandleNV(device, memory, handleType, pHandle")
}

pub unsafe extern "C" fn vkDestroyVideoSessionParametersKHR(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyVideoSessionParametersKHR(device, videoSessionParameters, pAllocator")
}

pub unsafe extern "C" fn vkCmdResetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    unimplemented!("vkCmdResetEvent(commandBuffer, event, stageMask")
}

pub unsafe extern "C" fn vkCmdDrawMeshTasksIndirectNV(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
) {
    unimplemented!("vkCmdDrawMeshTasksIndirectNV(commandBuffer, buffer, offset, drawCount, stride")
}

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

pub unsafe extern "C" fn vkCmdWriteTimestamp(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdWriteTimestamp(commandBuffer, pipelineStage, queryPool, query")
}

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

pub unsafe extern "C" fn vkGetMemoryFdPropertiesKHR(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: int,
    pMemoryFdProperties: Option<NonNull<VkMemoryFdPropertiesKHR>>,
) -> VkResult {
    unimplemented!("vkGetMemoryFdPropertiesKHR(device, handleType, fd, pMemoryFdProperties")
}

pub unsafe extern "C" fn vkCmdSetAlphaToCoverageEnableEXT(
    commandBuffer: VkCommandBuffer,
    alphaToCoverageEnable: VkBool32,
) {
    unimplemented!("vkCmdSetAlphaToCoverageEnableEXT(commandBuffer, alphaToCoverageEnable")
}

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

pub unsafe extern "C" fn vkDestroyValidationCacheEXT(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyValidationCacheEXT(device, validationCache, pAllocator")
}

pub unsafe extern "C" fn vkCmdResetEvent2(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
) {
    unimplemented!("vkCmdResetEvent2(commandBuffer, event, stageMask")
}

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

pub unsafe extern "C" fn vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    dfb: Option<NonNull<IDirectFB>>,
) -> VkBool32 {
    unimplemented!(
        "vkGetPhysicalDeviceDirectFBPresentationSupportEXT(physicalDevice, queueFamilyIndex, dfb"
    )
}

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

pub unsafe extern "C" fn vkCmdSetCoverageToColorEnableNV(
    commandBuffer: VkCommandBuffer,
    coverageToColorEnable: VkBool32,
) {
    unimplemented!("vkCmdSetCoverageToColorEnableNV(commandBuffer, coverageToColorEnable")
}

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

pub unsafe extern "C" fn vkCopyAccelerationStructureToMemoryKHR(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: Option<NonNull<VkCopyAccelerationStructureToMemoryInfoKHR>>,
) -> VkResult {
    unimplemented!("vkCopyAccelerationStructureToMemoryKHR(device, deferredOperation, pInfo")
}

pub unsafe extern "C" fn vkWaitSemaphores(
    device: VkDevice,
    pWaitInfo: Option<NonNull<VkSemaphoreWaitInfo>>,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitSemaphores(device, pWaitInfo, timeout")
}

pub unsafe extern "C" fn vkInvalidateMappedMemoryRanges(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: Option<NonNull<VkMappedMemoryRange>>,
) -> VkResult {
    unimplemented!("vkInvalidateMappedMemoryRanges(device, memoryRangeCount, pMemoryRanges")
}

pub unsafe extern "C" fn vkGetSemaphoreSciSyncObjNV(
    device: VkDevice,
    pGetSciSyncInfo: Option<NonNull<VkSemaphoreGetSciSyncInfoNV>>,
    pHandle: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreSciSyncObjNV(device, pGetSciSyncInfo, pHandle")
}

pub unsafe extern "C" fn vkCreateDebugUtilsMessengerEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkDebugUtilsMessengerCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMessenger: Option<NonNull<VkDebugUtilsMessengerEXT>>,
) -> VkResult {
    unimplemented!("vkCreateDebugUtilsMessengerEXT(instance, pCreateInfo, pAllocator, pMessenger")
}

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

pub unsafe extern "C" fn vkCreateCuModuleNVX(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkCuModuleCreateInfoNVX>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pModule: Option<NonNull<VkCuModuleNVX>>,
) -> VkResult {
    unimplemented!("vkCreateCuModuleNVX(device, pCreateInfo, pAllocator, pModule")
}

pub unsafe extern "C" fn vkAcquireDrmDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkAcquireDrmDisplayEXT(physicalDevice, drmFd, display")
}

pub unsafe extern "C" fn vkDeviceWaitIdle(device: VkDevice) -> VkResult {
    unimplemented!("vkDeviceWaitIdle(device")
}

pub unsafe extern "C" fn vkGetShaderModuleCreateInfoIdentifierEXT(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkShaderModuleCreateInfo>>,
    pIdentifier: Option<NonNull<VkShaderModuleIdentifierEXT>>,
) {
    unimplemented!("vkGetShaderModuleCreateInfoIdentifierEXT(device, pCreateInfo, pIdentifier")
}

pub unsafe extern "C" fn vkCreateVideoSessionParametersKHR(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkVideoSessionParametersCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pVideoSessionParameters: Option<NonNull<VkVideoSessionParametersKHR>>,
) -> VkResult {
    unimplemented!("vkCreateVideoSessionParametersKHR(device, pCreateInfo, pAllocator, pVideoSessionParameters")
}

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

pub unsafe extern "C" fn vkCreateVideoSessionKHR(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkVideoSessionCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pVideoSession: Option<NonNull<VkVideoSessionKHR>>,
) -> VkResult {
    unimplemented!("vkCreateVideoSessionKHR(device, pCreateInfo, pAllocator, pVideoSession")
}

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

pub unsafe extern "C" fn vkWaitForPresentKHR(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    presentId: u64,
    timeout: u64,
) -> VkResult {
    unimplemented!("vkWaitForPresentKHR(device, swapchain, presentId, timeout")
}

pub unsafe extern "C" fn vkResetCommandBuffer(
    commandBuffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlags,
) -> VkResult {
    unimplemented!("vkResetCommandBuffer(commandBuffer, flags")
}

pub unsafe extern "C" fn vkSetDeviceMemoryPriorityEXT(
    device: VkDevice,
    memory: VkDeviceMemory,
    priority: f32,
) {
    unimplemented!("vkSetDeviceMemoryPriorityEXT(device, memory, priority")
}

pub unsafe extern "C" fn vkAcquireProfilingLockKHR(
    device: VkDevice,
    pInfo: Option<NonNull<VkAcquireProfilingLockInfoKHR>>,
) -> VkResult {
    unimplemented!("vkAcquireProfilingLockKHR(device, pInfo")
}

pub unsafe extern "C" fn vkGetMicromapBuildSizesEXT(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: Option<NonNull<VkMicromapBuildInfoEXT>>,
    pSizeInfo: Option<NonNull<VkMicromapBuildSizesInfoEXT>>,
) {
    unimplemented!("vkGetMicromapBuildSizesEXT(device, buildType, pBuildInfo, pSizeInfo")
}

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

pub unsafe extern "C" fn vkGetDeviceFaultInfoEXT(
    device: VkDevice,
    pFaultCounts: Option<NonNull<VkDeviceFaultCountsEXT>>,
    pFaultInfo: Option<NonNull<VkDeviceFaultInfoEXT>>,
) -> VkResult {
    unimplemented!("vkGetDeviceFaultInfoEXT(device, pFaultCounts, pFaultInfo")
}

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

pub unsafe extern "C" fn vkGetDisplayPlaneSupportedDisplaysKHR(
    physicalDevice: VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: Option<NonNull<u32>>,
    pDisplays: Option<NonNull<VkDisplayKHR>>,
) -> VkResult {
    unimplemented!("vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice, planeIndex, pDisplayCount, pDisplays")
}

pub unsafe extern "C" fn vkCmdUpdateBuffer(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    dataSize: VkDeviceSize,
    pData: Option<NonNull<std::ffi::c_void>>,
) {
    unimplemented!("vkCmdUpdateBuffer(commandBuffer, dstBuffer, dstOffset, dataSize, pData")
}

pub unsafe extern "C" fn vkCmdSetEvent(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
) {
    unimplemented!("vkCmdSetEvent(commandBuffer, event, stageMask")
}

pub unsafe extern "C" fn vkCmdSetTessellationDomainOriginEXT(
    commandBuffer: VkCommandBuffer,
    domainOrigin: VkTessellationDomainOrigin,
) {
    unimplemented!("vkCmdSetTessellationDomainOriginEXT(commandBuffer, domainOrigin")
}

pub unsafe extern "C" fn vkCmdEndQuery(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
) {
    unimplemented!("vkCmdEndQuery(commandBuffer, queryPool, query")
}

pub unsafe extern "C" fn vkGetPhysicalDeviceDisplayPropertiesKHR(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkDisplayPropertiesKHR>>,
) -> VkResult {
    unimplemented!(
        "vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice, pPropertyCount, pProperties"
    )
}

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

pub unsafe extern "C" fn vkDestroyPrivateDataSlot(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyPrivateDataSlot(device, privateDataSlot, pAllocator")
}

pub unsafe extern "C" fn vkTrimCommandPool(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
) {
    unimplemented!("vkTrimCommandPool(device, commandPool, flags")
}

pub unsafe extern "C" fn vkReleaseDisplayEXT(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
) -> VkResult {
    unimplemented!("vkReleaseDisplayEXT(physicalDevice, display")
}

pub unsafe extern "C" fn vkCreateDescriptorUpdateTemplate(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkDescriptorUpdateTemplateCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDescriptorUpdateTemplate: Option<NonNull<VkDescriptorUpdateTemplate>>,
) -> VkResult {
    unimplemented!("vkCreateDescriptorUpdateTemplate(device, pCreateInfo, pAllocator, pDescriptorUpdateTemplate")
}

pub unsafe extern "C" fn vkQueueSetPerformanceConfigurationINTEL(
    queue: VkQueue,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult {
    unimplemented!("vkQueueSetPerformanceConfigurationINTEL(queue, configuration")
}

pub unsafe extern "C" fn vkCmdSetPrimitiveRestartEnable(
    commandBuffer: VkCommandBuffer,
    primitiveRestartEnable: VkBool32,
) {
    unimplemented!("vkCmdSetPrimitiveRestartEnable(commandBuffer, primitiveRestartEnable")
}

pub unsafe extern "C" fn vkCmdSetAttachmentFeedbackLoopEnableEXT(
    commandBuffer: VkCommandBuffer,
    aspectMask: VkImageAspectFlags,
) {
    unimplemented!("vkCmdSetAttachmentFeedbackLoopEnableEXT(commandBuffer, aspectMask")
}

pub unsafe extern "C" fn vkGetMemoryZirconHandleFUCHSIA(
    device: VkDevice,
    pGetZirconHandleInfo: Option<NonNull<VkMemoryGetZirconHandleInfoFUCHSIA>>,
    pZirconHandle: Option<NonNull<zx_handle_t>>,
) -> VkResult {
    unimplemented!("vkGetMemoryZirconHandleFUCHSIA(device, pGetZirconHandleInfo, pZirconHandle")
}

pub unsafe extern "C" fn vkCmdSetScissorWithCount(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: Option<NonNull<VkRect2D>>,
) {
    unimplemented!("vkCmdSetScissorWithCount(commandBuffer, scissorCount, pScissors")
}

pub unsafe extern "C" fn vkCmdNextSubpass(
    commandBuffer: VkCommandBuffer,
    contents: VkSubpassContents,
) {
    unimplemented!("vkCmdNextSubpass(commandBuffer, contents")
}

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

pub unsafe extern "C" fn vkCmdBeginRendering(
    commandBuffer: VkCommandBuffer,
    pRenderingInfo: Option<NonNull<VkRenderingInfo>>,
) {
    unimplemented!("vkCmdBeginRendering(commandBuffer, pRenderingInfo")
}

pub unsafe extern "C" fn vkGetAccelerationStructureMemoryRequirementsNV(
    device: VkDevice,
    pInfo: Option<NonNull<VkAccelerationStructureMemoryRequirementsInfoNV>>,
    pMemoryRequirements: Option<NonNull<VkMemoryRequirements2KHR>>,
) {
    unimplemented!(
        "vkGetAccelerationStructureMemoryRequirementsNV(device, pInfo, pMemoryRequirements"
    )
}

pub unsafe extern "C" fn vkDestroyCuFunctionNVX(
    device: VkDevice,
    function: VkCuFunctionNVX,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    unimplemented!("vkDestroyCuFunctionNVX(device, function, pAllocator")
}

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

pub unsafe extern "C" fn vkGetDeviceMicromapCompatibilityEXT(
    device: VkDevice,
    pVersionInfo: Option<NonNull<VkMicromapVersionInfoEXT>>,
    pCompatibility: Option<NonNull<VkAccelerationStructureCompatibilityKHR>>,
) {
    unimplemented!("vkGetDeviceMicromapCompatibilityEXT(device, pVersionInfo, pCompatibility")
}

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

pub unsafe extern "C" fn vkGetInstanceProcAddr(
    instance: VkInstance,
    pName: Option<NonNull<std::ffi::c_char>>,
) -> PFN_vkVoidFunction {
    unimplemented!("vkGetInstanceProcAddr(instance, pName")
}

pub unsafe extern "C" fn vkGetFenceFdKHR(
    device: VkDevice,
    pGetFdInfo: Option<NonNull<VkFenceGetFdInfoKHR>>,
    pFd: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!("vkGetFenceFdKHR(device, pGetFdInfo, pFd")
}

pub unsafe extern "C" fn vkGetSemaphoreCounterValue(
    device: VkDevice,
    semaphore: VkSemaphore,
    pValue: Option<NonNull<u64>>,
) -> VkResult {
    unimplemented!("vkGetSemaphoreCounterValue(device, semaphore, pValue")
}

pub unsafe extern "C" fn vkCmdSetStencilTestEnable(
    commandBuffer: VkCommandBuffer,
    stencilTestEnable: VkBool32,
) {
    unimplemented!("vkCmdSetStencilTestEnable(commandBuffer, stencilTestEnable")
}

pub unsafe extern "C" fn vkCreateDebugReportCallbackEXT(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkDebugReportCallbackCreateInfoEXT>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pCallback: Option<NonNull<VkDebugReportCallbackEXT>>,
) -> VkResult {
    unimplemented!("vkCreateDebugReportCallbackEXT(instance, pCreateInfo, pAllocator, pCallback")
}

pub unsafe extern "C" fn vkBuildMicromapsEXT(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: Option<NonNull<VkMicromapBuildInfoEXT>>,
) -> VkResult {
    unimplemented!("vkBuildMicromapsEXT(device, deferredOperation, infoCount, pInfos")
}

pub unsafe extern "C" fn vkSetBufferCollectionBufferConstraintsFUCHSIA(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pBufferConstraintsInfo: Option<NonNull<VkBufferConstraintsInfoFUCHSIA>>,
) -> VkResult {
    unimplemented!(
        "vkSetBufferCollectionBufferConstraintsFUCHSIA(device, collection, pBufferConstraintsInfo"
    )
}

pub unsafe extern "C" fn vkGetPhysicalDeviceToolProperties(
    physicalDevice: VkPhysicalDevice,
    pToolCount: Option<NonNull<u32>>,
    pToolProperties: Option<NonNull<VkPhysicalDeviceToolProperties>>,
) -> VkResult {
    unimplemented!("vkGetPhysicalDeviceToolProperties(physicalDevice, pToolCount, pToolProperties")
}

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

pub unsafe extern "C" fn vkGetDeviceGroupSurfacePresentModesKHR(
    device: VkDevice,
    surface: VkSurfaceKHR,
    pModes: Option<NonNull<VkDeviceGroupPresentModeFlagsKHR>>,
) -> VkResult {
    unimplemented!("vkGetDeviceGroupSurfacePresentModesKHR(device, surface, pModes")
}

pub unsafe extern "C" fn vkGetSwapchainGrallocUsageANDROID(
    device: VkDevice,
    format: VkFormat,
    imageUsage: VkImageUsageFlags,
    grallocUsage: Option<NonNull<int>>,
) -> VkResult {
    unimplemented!("vkGetSwapchainGrallocUsageANDROID(device, format, imageUsage, grallocUsage")
}

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

pub unsafe extern "C" fn vkCmdSetPolygonModeEXT(
    commandBuffer: VkCommandBuffer,
    polygonMode: VkPolygonMode,
) {
    unimplemented!("vkCmdSetPolygonModeEXT(commandBuffer, polygonMode")
}

pub unsafe extern "C" fn vkCmdBeginRenderPass(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: Option<NonNull<VkRenderPassBeginInfo>>,
    contents: VkSubpassContents,
) {
    unimplemented!("vkCmdBeginRenderPass(commandBuffer, pRenderPassBegin, contents")
}

pub unsafe extern "C" fn vkCmdCopyAccelerationStructureKHR(
    commandBuffer: VkCommandBuffer,
    pInfo: Option<NonNull<VkCopyAccelerationStructureInfoKHR>>,
) {
    unimplemented!("vkCmdCopyAccelerationStructureKHR(commandBuffer, pInfo")
}

pub unsafe extern "C" fn vkCmdBeginConditionalRenderingEXT(
    commandBuffer: VkCommandBuffer,
    pConditionalRenderingBegin: Option<NonNull<VkConditionalRenderingBeginInfoEXT>>,
) {
    unimplemented!("vkCmdBeginConditionalRenderingEXT(commandBuffer, pConditionalRenderingBegin")
}

pub unsafe extern "C" fn vkResetQueryPool(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
) {
    unimplemented!("vkResetQueryPool(device, queryPool, firstQuery, queryCount")
}

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

pub unsafe extern "C" fn vkCreateDisplayModeKHR(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pCreateInfo: Option<NonNull<VkDisplayModeCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pMode: Option<NonNull<VkDisplayModeKHR>>,
) -> VkResult {
    unimplemented!("vkCreateDisplayModeKHR(physicalDevice, display, pCreateInfo, pAllocator, pMode")
}

pub unsafe extern "C" fn vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(
    device: VkDevice,
    pInfo: Option<NonNull<VkAccelerationStructureCaptureDescriptorDataInfoEXT>>,
    pData: Option<NonNull<std::ffi::c_void>>,
) -> VkResult {
    unimplemented!("vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(device, pInfo, pData")
}
