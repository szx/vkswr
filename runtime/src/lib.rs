use headers::vk::*;

pub fn create_instance(p_create_info: *const VkInstanceCreateInfo,
                       p_allocator: *const VkAllocationCallbacks,
                       p_instance: *mut VkInstance) -> VkResult {
    println!("Hello from runtime::vkCreateInstance()!");
    unsafe {
        *p_instance = std::ptr::null_mut();
    }
    // TODO: Create and register internal VkInstance.
    VkResult::VK_SUCCESS
}
