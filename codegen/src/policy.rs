//! Implementation choices that are not described by the Vulkan registry.
//!
//! Registry facts such as extension spec versions, format layouts, and command
//! signatures are derived from `vk.xml`. This module is the single hand-written
//! input for choices the registry cannot make for us.

/// Extensions advertised by instances created through this ICD.
pub const INSTANCE_EXTENSIONS: &[&str] = &["VK_KHR_surface", "VK_KHR_xcb_surface"];

/// Extensions advertised by physical devices exposed through this ICD.
pub const DEVICE_EXTENSIONS: &[&str] = &[
    "VK_KHR_swapchain",
    "VK_EXT_tooling_info",
    "VK_EXT_debug_marker",
];

/// Commands implemented by the Vulkan loader instead of the ICD.
pub const LOADER_COMMANDS: &[&str] = &[
    "vkGetInstanceProcAddr",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateDeviceLayerProperties",
];

/// Commands resolved through the instance dispatch table despite a device-level
/// registry signature.
pub const INSTANCE_COMMANDS: &[&str] = &["vkGetDeviceProcAddr"];
