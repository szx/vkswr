use headers::vk_decls::*;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Instance {
    driver_name: &'static str,
}

impl Instance {
    const fn new() -> Self {
        Self {
            driver_name: "vulkan_software_rasterizer",
        }
    }
}

lazy_static! {
    static ref INSTANCE: Instance = Instance::new();
}

pub fn create_instance(
    create_info: &VkInstanceCreateInfo,
    p_instance: std::ptr::NonNull<VkInstance>,
) -> VkResult {
    let _ = create_info;
    println!("Hello from runtime::create_instance()!");
    unsafe { set_dispatchable_handle(p_instance, &*INSTANCE) };

    VkResult::VK_SUCCESS
}

pub fn enumerate_physical_devices(
    instance: VkInstance,
) -> VkResult {
    println!("Hello from runtime::enumerate_physical_devices()!");
    unsafe {
        let instance : &Instance = get_dispatchable_handle_ref(instance);

        println!("instance: {:?}", instance);
        println!("driver_name: {:?}", instance.driver_name);
        assert_eq!(instance.driver_name, (*INSTANCE).driver_name);
    }

    VkResult::VK_SUCCESS
}

