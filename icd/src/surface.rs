//! VK_KHR_surface and VK_KHR_xcb_surface extension instance commands

use headers::vk_decls::*;
use runtime::context::{Dispatchable, NonDispatchable};
use runtime::instance::Instance;
use runtime::surface::*;
use runtime::*;

pub unsafe extern "C" fn vkCreateXcbSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkXcbSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    let Some(instance) = Instance::from_handle(instance) else {
        unreachable!()
    };

    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR
    );

    let _ = pAllocator;

    let Some(pSurface) = pSurface else {
        unreachable!()
    };

    *pSurface.as_ptr() = Surface::create(instance, create_info);

    VkResult::VK_SUCCESS
}

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

pub unsafe extern "C" fn vkDestroySurfaceKHR(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let Some(instance) = Instance::from_handle(instance) else {
        unreachable!()
    };

    let _ = pAllocator;

    Surface::drop_handle(surface);
}
