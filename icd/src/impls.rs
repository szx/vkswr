//! Vulkan entry point implementations.
//!
//! Hand-written implementations live here; the generated dispatch file supplies
//! stubs for dispatched commands that are not implemented yet.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use crate::buffer::*;
use crate::command_buffer::*;
use crate::descriptor::*;
use crate::image::*;
use crate::memory::*;
use crate::pipeline::*;
use crate::sampler::*;
use crate::surface::*;
use crate::swapchain::*;
use headers::vk_decls::*;
use runtime::command_buffer::CommandBuffer;
use runtime::context::VulkanObject;
use runtime::fence::Fence;
use runtime::instance::Instance;
use runtime::logical_device::LogicalDevice;
use runtime::physical_device::PhysicalDevice;
use runtime::queue::Queue;
use runtime::semaphore::Semaphore;

include!(concat!(env!("OUT_DIR"), "/dispatch_tables.rs"));

pub unsafe extern "C" fn vkCreateInstance(
    pCreateInfo: Option<NonNull<VkInstanceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pInstance: Option<NonNull<VkInstance>>,
) -> VkResult {
    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO
    );

    let _ = pAllocator;

    let pInstance = pInstance.expect("Vulkan guarantees a non-null pointer");

    *pInstance.as_ptr() = match Instance::create() {
        Ok(inner) => inner,
        Err(e) => return e,
    };

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkEnumeratePhysicalDevices(
    instance: VkInstance,
    pPhysicalDeviceCount: Option<NonNull<u32>>,
    pPhysicalDevices: Option<NonNull<VkPhysicalDevice>>,
) -> VkResult {
    let instance = Instance::from_handle(instance).expect("Vulkan handle refers to a live object");

    let pPhysicalDeviceCount = pPhysicalDeviceCount.expect("Vulkan guarantees a non-null pointer");

    pPhysicalDevices.map_or_else(
        || {
            *pPhysicalDeviceCount.as_ptr() = Instance::physical_device_count() as u32;
            VkResult::VK_SUCCESS
        },
        |pPhysicalDevices| {
            *pPhysicalDevices.as_ptr() = instance.lock().physical_device().lock().handle();
            VkResult::VK_SUCCESS
        },
    )
}

pub unsafe extern "C" fn vkGetPhysicalDeviceProperties(
    physicalDevice: VkPhysicalDevice,
    pProperties: Option<NonNull<VkPhysicalDeviceProperties>>,
) {
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pProperties = pProperties.expect("Vulkan guarantees a non-null pointer");

    // SPEC: "Returns properties of a physical device"
    *pProperties.as_ptr() = physicalDevice.lock().properties();
}

pub unsafe extern "C" fn vkGetPhysicalDeviceToolProperties(
    physicalDevice: VkPhysicalDevice,
    pToolCount: Option<NonNull<u32>>,
    pToolProperties: Option<NonNull<VkPhysicalDeviceToolProperties>>,
) -> VkResult {
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    if let Some(pToolProperties) = pToolProperties {
        let tool_properties = physicalDevice.lock().tool_properties();
        std::ptr::copy_nonoverlapping(
            tool_properties.as_ptr(),
            pToolProperties.as_ptr(),
            tool_properties.len(),
        );
    } else if let Some(pToolCount) = pToolCount {
        *pToolCount.as_ptr() = physicalDevice.lock().tool_properties().len() as u32;
    }

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkGetPhysicalDeviceMemoryProperties(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: Option<NonNull<VkPhysicalDeviceMemoryProperties>>,
) {
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pMemoryProperties = pMemoryProperties.expect("Vulkan guarantees a non-null pointer");

    // SPEC: "Reports memory information for the specified physical device"
    *pMemoryProperties.as_ptr() = physicalDevice.lock().memory_properties();
}

pub unsafe extern "C" fn vkGetPhysicalDeviceFeatures(
    physicalDevice: VkPhysicalDevice,
    pFeatures: Option<NonNull<VkPhysicalDeviceFeatures>>,
) {
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pFeatures = pFeatures.expect("Vulkan guarantees a non-null pointer");

    // SPEC: "Reports capabilities of a physical device"
    *pFeatures.as_ptr() = physicalDevice.lock().features();
}

pub unsafe extern "C" fn vkGetPhysicalDeviceQueueFamilyProperties(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: Option<NonNull<u32>>,
    pQueueFamilyProperties: Option<NonNull<VkQueueFamilyProperties>>,
) {
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pQueueFamilyPropertyCount =
        pQueueFamilyPropertyCount.expect("Vulkan guarantees a non-null pointer");

    // SPEC: "Reports properties of the queues of the specified physical device"
    if let Some(pQueueFamilyProperties) = pQueueFamilyProperties {
        let queue_family_properties = physicalDevice.lock().queue_family_properties();
        pQueueFamilyProperties.as_ptr().copy_from(
            queue_family_properties.as_ptr(),
            queue_family_properties.len(),
        );
    } else {
        *pQueueFamilyPropertyCount.as_ptr() =
            physicalDevice.lock().queue_family_properties().len() as u32;
    }
}

unsafe fn enumerate_extension_properties(
    available: &[VkExtensionProperties],
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult {
    let Some(pProperties) = pProperties else {
        if let Some(pPropertyCount) = pPropertyCount {
            *pPropertyCount.as_ptr() = available.len() as u32;
        }
        return VkResult::VK_SUCCESS;
    };
    let Some(pPropertyCount) = pPropertyCount else {
        return VkResult::VK_ERROR_INITIALIZATION_FAILED;
    };

    let requested = *pPropertyCount.as_ptr() as usize;
    let written = requested.min(available.len());
    std::ptr::copy_nonoverlapping(available.as_ptr(), pProperties.as_ptr(), written);
    *pPropertyCount.as_ptr() = written as u32;
    if written < available.len() {
        VkResult::VK_INCOMPLETE
    } else {
        VkResult::VK_SUCCESS
    }
}

pub unsafe extern "C" fn vkEnumerateInstanceExtensionProperties(
    pLayerName: Option<NonNull<std::ffi::c_char>>,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult {
    if pLayerName.is_some() {
        return VkResult::VK_ERROR_LAYER_NOT_PRESENT;
    }
    let properties = Instance::extension_properties();
    enumerate_extension_properties(&properties, pPropertyCount, pProperties)
}

pub unsafe extern "C" fn vkEnumerateDeviceExtensionProperties(
    physicalDevice: VkPhysicalDevice,
    pLayerName: Option<NonNull<std::ffi::c_char>>,
    pPropertyCount: Option<NonNull<u32>>,
    pProperties: Option<NonNull<VkExtensionProperties>>,
) -> VkResult {
    let _physical_device =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    if pLayerName.is_some() {
        return VkResult::VK_ERROR_LAYER_NOT_PRESENT;
    }
    let properties = PhysicalDevice::extension_properties();
    enumerate_extension_properties(&properties, pPropertyCount, pProperties)
}

pub unsafe extern "C" fn vkCreateDevice(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: Option<NonNull<VkDeviceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pDevice: Option<NonNull<VkDevice>>,
) -> VkResult {
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO
    );
    let _ = pAllocator;

    let pDevice = pDevice.expect("Vulkan guarantees a non-null pointer");

    let Some(queue_create_info) = create_info.pQueueCreateInfos else {
        return VkResult::VK_ERROR_INITIALIZATION_FAILED;
    };
    let queue_create_info = queue_create_info.as_ref();

    let enabled_features = create_info.pEnabledFeatures.map(|x| x.as_ref());

    *pDevice.as_ptr() =
        match LogicalDevice::create(physicalDevice, enabled_features, queue_create_info) {
            Ok(object) => object,
            Err(err) => return err,
        };
    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkGetDeviceProcAddr(
    device: VkDevice,
    pName: Option<NonNull<std::ffi::c_char>>,
) -> PFN_vkVoidFunction {
    let _device = LogicalDevice::from_handle(device).expect("device handle is live");
    let pName = pName?;
    // SAFETY: The loader passes a NUL-terminated command name.
    let Ok(pName) = (unsafe { std::ffi::CStr::from_ptr(pName.as_ptr()) }).to_str() else {
        return None;
    };

    // SPEC: Only device-level commands are valid here.
    device_command(pName)
}

pub unsafe extern "C" fn vkGetPhysicalDeviceFormatProperties(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: Option<NonNull<VkFormatProperties>>,
) {
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pFormatProperties = pFormatProperties.expect("Vulkan guarantees a non-null pointer");

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
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pImageFormatProperties =
        pImageFormatProperties.expect("Vulkan guarantees a non-null pointer");

    let properties = physicalDevice
        .lock()
        .image_format_properties(format, type_, tiling, usage, flags);
    properties.map_or(VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED, |properties| {
        *pImageFormatProperties.as_ptr() = properties;
        VkResult::VK_SUCCESS
    })
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
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pQueue = pQueue.expect("Vulkan guarantees a non-null pointer");

    let queue = device.lock().queue(queueFamilyIndex, queueIndex);
    *pQueue.as_ptr() = queue.lock().handle();
}

pub unsafe extern "C" fn vkCreateFence(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkFenceCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pFence: Option<NonNull<VkFence>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO
    );

    let _ = pAllocator;

    let pFence = pFence.expect("Vulkan guarantees a non-null pointer");

    *pFence.as_ptr() = Fence::create(device, create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroyFence(
    device: VkDevice,
    fence: VkFence,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    Fence::drop_handle(fence);
}

pub unsafe extern "C" fn vkCreateSemaphore(
    device: VkDevice,
    pCreateInfo: Option<NonNull<VkSemaphoreCreateInfo>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSemaphore: Option<NonNull<VkSemaphore>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO
    );

    let _ = pAllocator;

    let pSemaphore = pSemaphore.expect("Vulkan guarantees a non-null pointer");

    *pSemaphore.as_ptr() = Semaphore::create(create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroySemaphore(
    device: VkDevice,
    semaphore: VkSemaphore,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    Semaphore::drop_handle(semaphore);
}

pub unsafe extern "C" fn vkWaitForFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: Option<NonNull<VkFence>>,
    waitAll: VkBool32,
    timeout: u64,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    if fenceCount == 0 {
        return VkResult::VK_SUCCESS;
    }

    let pFences = pFences.expect("Vulkan guarantees a non-null pointer");
    let pFences = std::slice::from_raw_parts(pFences.as_ptr(), fenceCount as usize);
    let fences = pFences
        .iter()
        .flat_map(|&handle| Fence::from_handle(handle))
        .collect::<Vec<_>>();

    device.lock().wait_for_fences(fences, waitAll != 0, timeout);
    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkResetFences(
    device: VkDevice,
    fenceCount: u32,
    pFences: Option<NonNull<VkFence>>,
) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    if fenceCount == 0 {
        return VkResult::VK_SUCCESS;
    }

    let pFences = pFences.expect("Vulkan guarantees a non-null pointer");
    let pFences = std::slice::from_raw_parts(pFences.as_ptr(), fenceCount as usize);
    let fences = pFences
        .iter()
        .flat_map(|&handle| Fence::from_handle(handle))
        .collect::<Vec<_>>();

    device.lock().reset_fences(fences);
    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkQueueSubmit(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: Option<NonNull<VkSubmitInfo>>,
    fence: VkFence,
) -> VkResult {
    let queue = Queue::from_handle(queue).expect("Vulkan handle refers to a live object");

    let submits = pSubmits.map_or(&[] as &[_], |x| {
        std::slice::from_raw_parts(x.as_ptr(), submitCount as usize)
    });

    for submit in submits {
        let wait_semaphores = submit
            .pWaitSemaphores
            .map_or(&[] as &[_], |x| {
                std::slice::from_raw_parts(x.as_ptr(), submit.waitSemaphoreCount as usize)
            })
            .iter()
            .flat_map(|&handle| Semaphore::from_handle(handle));
        let wait_semaphores_stage_flags = submit.pWaitDstStageMask.map_or(&[] as &[_], |x| {
            std::slice::from_raw_parts(x.as_ptr(), submit.waitSemaphoreCount as usize)
        });
        let signal_semaphores = submit
            .pSignalSemaphores
            .map_or(&[] as &[_], |x| {
                std::slice::from_raw_parts(x.as_ptr(), submit.signalSemaphoreCount as usize)
            })
            .iter()
            .flat_map(|&handle| Semaphore::from_handle(handle));
        let command_buffers = submit
            .pCommandBuffers
            .map_or(&[] as &[_], |x| {
                std::slice::from_raw_parts(x.as_ptr(), submit.commandBufferCount as usize)
            })
            .iter()
            .flat_map(|&handle| CommandBuffer::from_handle(handle));

        queue.lock().submit(
            wait_semaphores,
            wait_semaphores_stage_flags,
            signal_semaphores,
            command_buffers,
        );
    }

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDeviceWaitIdle(device: VkDevice) -> VkResult {
    let device = LogicalDevice::from_handle(device).expect("Vulkan handle refers to a live object");

    device.lock().wait_idle()
}

pub unsafe extern "C" fn vkQueueWaitIdle(queue: VkQueue) -> VkResult {
    let queue = Queue::from_handle(queue).expect("Vulkan handle refers to a live object");

    queue.lock().wait_idle()
}

/* VK_KHR_surface extension instance commands */

pub unsafe extern "C" fn vkGetPhysicalDeviceSurfaceSupportKHR(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: Option<NonNull<VkBool32>>,
) -> VkResult {
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pSupported = pSupported.expect("Vulkan guarantees a non-null pointer");

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
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pPresentModeCount = pPresentModeCount.expect("Vulkan guarantees a non-null pointer");

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
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pSurfaceFormatCount = pSurfaceFormatCount.expect("Vulkan guarantees a non-null pointer");

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
    let physicalDevice =
        PhysicalDevice::from_handle(physicalDevice).expect("Vulkan handle refers to a live object");

    let pSurfaceCapabilities = pSurfaceCapabilities.expect("Vulkan guarantees a non-null pointer");

    *pSurfaceCapabilities.as_ptr() = physicalDevice.lock().surface_capabilities();

    VkResult::VK_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn count_ptr(count: &mut u32) -> NonNull<u32> {
        NonNull::new(count).expect("count pointer is non-null")
    }

    #[allow(clippy::as_ptr_cast_mut)]
    fn layer_name(value: &CString) -> NonNull<std::ffi::c_char> {
        NonNull::new(value.as_ptr() as *mut std::ffi::c_char).expect("layer name is non-null")
    }

    #[test]
    fn enumerates_instance_extensions() {
        let mut count = 0;
        let result = unsafe {
            vkEnumerateInstanceExtensionProperties(None, Some(count_ptr(&mut count)), None)
        };
        assert_eq!(result, VkResult::VK_SUCCESS);
        assert_eq!(count as usize, Instance::extension_count());

        let mut properties = vec![VkExtensionProperties::default(); Instance::extension_count()];
        let mut count = properties.len() as u32;
        let result = unsafe {
            vkEnumerateInstanceExtensionProperties(
                None,
                Some(count_ptr(&mut count)),
                NonNull::new(properties.as_mut_ptr()),
            )
        };
        assert_eq!(result, VkResult::VK_SUCCESS);
        assert_eq!(count as usize, properties.len());
    }

    #[test]
    fn truncates_instance_extensions() {
        let mut properties =
            vec![VkExtensionProperties::default(); Instance::extension_count() - 1];
        let mut count = properties.len() as u32;
        let result = unsafe {
            vkEnumerateInstanceExtensionProperties(
                None,
                Some(count_ptr(&mut count)),
                NonNull::new(properties.as_mut_ptr()),
            )
        };
        assert_eq!(result, VkResult::VK_INCOMPLETE);
        assert_eq!(count as usize, properties.len());
    }

    #[test]
    fn rejects_instance_layer_enumeration() {
        let layer = CString::new("VK_LAYER_test").expect("valid layer name");
        let result =
            unsafe { vkEnumerateInstanceExtensionProperties(Some(layer_name(&layer)), None, None) };
        assert_eq!(result, VkResult::VK_ERROR_LAYER_NOT_PRESENT);
    }

    #[test]
    fn enumerates_device_extensions() {
        let physical_device = PhysicalDevice::create();
        let mut count = 0;
        let result = unsafe {
            vkEnumerateDeviceExtensionProperties(
                physical_device,
                None,
                Some(count_ptr(&mut count)),
                None,
            )
        };
        assert_eq!(result, VkResult::VK_SUCCESS);
        assert_eq!(count as usize, PhysicalDevice::extension_count());

        let mut properties =
            vec![VkExtensionProperties::default(); PhysicalDevice::extension_count()];
        let mut count = properties.len() as u32;
        let result = unsafe {
            vkEnumerateDeviceExtensionProperties(
                physical_device,
                None,
                Some(count_ptr(&mut count)),
                NonNull::new(properties.as_mut_ptr()),
            )
        };
        assert_eq!(result, VkResult::VK_SUCCESS);
        assert_eq!(count as usize, properties.len());
    }

    #[test]
    fn truncates_device_extensions() {
        let physical_device = PhysicalDevice::create();
        let mut properties =
            vec![VkExtensionProperties::default(); PhysicalDevice::extension_count() - 1];
        let mut count = properties.len() as u32;
        let result = unsafe {
            vkEnumerateDeviceExtensionProperties(
                physical_device,
                None,
                Some(count_ptr(&mut count)),
                NonNull::new(properties.as_mut_ptr()),
            )
        };
        assert_eq!(result, VkResult::VK_INCOMPLETE);
        assert_eq!(count as usize, properties.len());
    }

    #[test]
    fn rejects_device_layer_enumeration() {
        let physical_device = PhysicalDevice::create();
        let layer = CString::new("VK_LAYER_test").expect("valid layer name");
        let result = unsafe {
            vkEnumerateDeviceExtensionProperties(
                physical_device,
                Some(layer_name(&layer)),
                None,
                None,
            )
        };
        assert_eq!(result, VkResult::VK_ERROR_LAYER_NOT_PRESENT);
    }
}
