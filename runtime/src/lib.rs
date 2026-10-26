//! Safe Vulkan object model used by the VkSWR installable client driver.
//!
//! Raw Vulkan handles are registered in [`context::Context`] and resolved back
//! to reference-counted objects; each module mirrors one Vulkan object type.

pub mod buffer;
pub mod command_buffer;
pub mod context;
pub mod descriptor;
pub mod extensions;
pub mod fence;
pub mod image;
pub mod instance;
pub mod logical_device;
pub mod memory;
pub mod physical_device;
pub mod pipeline;
pub mod queue;
pub mod sampler;
pub mod semaphore;
pub mod surface;
pub mod swapchain;
