//! VK_KHR_surface and VK_KHR_xcb_surface extension instance commands

use headers::vk_decls::*;
use runtime::context::VulkanObject;
use runtime::instance::Instance;
use runtime::surface::*;

pub unsafe extern "C" fn vkCreateXcbSurfaceKHR(
    instance: VkInstance,
    pCreateInfo: Option<NonNull<VkXcbSurfaceCreateInfoKHR>>,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
    pSurface: Option<NonNull<VkSurfaceKHR>>,
) -> VkResult {
    let instance = Instance::from_handle(instance).expect("Vulkan handle refers to a live object");

    let pCreateInfo = pCreateInfo.expect("Vulkan guarantees a non-null pointer");
    let create_info = pCreateInfo.as_ref();
    assert_eq!(
        create_info.sType,
        VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR
    );

    let _ = pAllocator;

    let pSurface = pSurface.expect("Vulkan guarantees a non-null pointer");

    *pSurface.as_ptr() = Surface::create(instance, create_info);

    VkResult::VK_SUCCESS
}

pub unsafe extern "C" fn vkDestroySurfaceKHR(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: Option<NonNull<VkAllocationCallbacks>>,
) {
    let _instance = Instance::from_handle(instance).expect("Vulkan handle refers to a live object");

    let _ = pAllocator;

    Surface::drop_handle(surface);
}
