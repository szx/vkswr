use headers::vk_decls::*;
use lazy_static::lazy_static;

/* Instance */

/// Contains system-level information and functionality
#[derive(Debug)]
pub struct Instance {
    driver_name: &'static str,
    physical_device: PhysicalDevice,
}

impl Instance {
    pub const fn physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }
}

impl Instance {
    const fn new() -> Self {
        Self {
            driver_name: "vulkan_software_rasterizer",
            physical_device: PhysicalDevice::new(),
        }
    }
}

lazy_static! {
    static ref INSTANCE: Instance = Instance::new();
}

pub fn create_instance(
    create_info: &VkInstanceCreateInfo,
    p_instance: NonNull<VkInstance>,
) -> VkResult {
    let _ = create_info;
    println!("Hello from runtime::create_instance()!");
    unsafe { set_dispatchable_handle(p_instance, &*INSTANCE) };

    VkResult::VK_SUCCESS
}

/* PhysicalDevice */

/// Performs rendering operations.
#[derive(Debug)]
pub struct PhysicalDevice {
    name: &'static str,
}

impl PhysicalDevice {
    /// Instance currently supports 1 `PhysicalDevice`.
    pub const fn count(_: &Instance) -> u32 {
        1
    }
}

impl PhysicalDevice {
    const fn new() -> Self {
        Self {
            name: "PhysicalDevice",
        }
    }
}

/* LogicalDevice */

/// Identifier used to associate functions with a `PhysicalDevice`.
#[derive(Debug)]
pub struct LogicalDevice {
    driver_name: &'static str,
}

impl LogicalDevice {
    const fn new() -> Self {
        Self {
            driver_name: "vulkan_software_rasterizer",
        }
    }
}