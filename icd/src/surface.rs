//! VK_KHR_surface and VK_KHR_xcb_surface extension instance commands

use headers::vk_decls::*;
use runtime::surface::*;
use runtime::*;

pub unsafe extern "C" fn vkCreateXcbSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkXcbSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    // VUID-vkCreateXcbSurfaceKHR-instance-parameter
    let Some(instance) = Instance::get_handle(instance) else {
        unreachable!()
    };

    // VUID-vkCreateXcbSurfaceKHR-pCreateInfo-parameter
    let Some(pCreateInfo) = pCreateInfo else {
        unreachable!()
    };
    let create_info = pCreateInfo.as_ref();
    // TODO: Automate valid CreateInfo structure asserts.
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR
    );

    // VUID-vkCreateXcbSurfaceKHR-pAllocator-parameter
    if let Some(pAllocator) = pAllocator {
        let pAllocator = pAllocator.as_ptr();
        // TODO: Use callbacks for memory allocation.
    }

    // VUID-vkCreateXcbSurfaceKHR-pSurface-parameter
    let Some(pSurface) = pSurface else {
        unreachable!()
    };

    Surface::set_handle(pSurface, Surface::create(create_info));

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
    // VUID-vkDestroySurfaceKHR-instance-parameter
    let Some(instance) = Instance::get_handle(instance) else {
        unreachable!()
    };

    // VUID-vkDestroySurfaceKHR-pAllocator-parameter
    if let Some(pAllocator) = pAllocator {
        let pAllocator = pAllocator.as_ptr();
        // TODO: Use callbacks for memory allocation.
    }

    Surface::drop_handle(surface);
}
