use headers::vk_decls::*;

/// # Safety
///
/// TODO: This function should be rewritten - move all unsafe stuff to impls.rs.
pub unsafe fn create_instance(
    p_create_info: *const VkInstanceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_instance: *mut VkInstance,
) -> VkResult {
    let _ = p_create_info;
    let _ = p_allocator;
    println!("Hello from runtime::vkCreateInstance()!");
    *p_instance = std::ptr::null_mut();
    // TODO: Create and register internal VkInstance.
    VkResult::VK_SUCCESS
}
