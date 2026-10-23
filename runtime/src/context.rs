//! Global object registry.
//!
//! Vulkan hands opaque handles to the application. The runtime keeps the real
//! object behind a reference-counted mutex and uses the handle as a key, which
//! keeps the FFI boundary free of lifetimes.

use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock};

use headers::vk_decls::{
    VkDispatchableHandle, VkDispatchableHandleInner, VkLoaderData, VkNonDispatchableHandle,
};
use parking_lot::{Mutex, RwLock, RwLockWriteGuard};
use std::ptr::NonNull;

/// Every Vulkan object in the process.
#[derive(Debug, Default)]
pub struct Context {
    instances: HashMap<VkDispatchableHandle, Arc<Mutex<crate::instance::Instance>>>,
    physical_devices:
        HashMap<VkDispatchableHandle, Arc<Mutex<crate::physical_device::PhysicalDevice>>>,
    logical_devices:
        HashMap<VkDispatchableHandle, Arc<Mutex<crate::logical_device::LogicalDevice>>>,
    queues: HashMap<VkDispatchableHandle, Arc<Mutex<crate::queue::Queue>>>,
    command_buffers:
        HashMap<VkDispatchableHandle, Arc<Mutex<crate::command_buffer::CommandBuffer>>>,
    fences: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::fence::Fence>>>,
    semaphores: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::semaphore::Semaphore>>>,
    surfaces: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::surface::Surface>>>,
    swapchains: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::swapchain::Swapchain>>>,
    images: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::image::Image>>>,
    image_views: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::image::ImageView>>>,
    command_pools: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::command_buffer::CommandPool>>>,
    memory_allocations:
        HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::memory::MemoryAllocation>>>,
    samplers: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::sampler::Sampler>>>,
    buffers: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::buffer::Buffer>>>,
    buffer_views: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::buffer::BufferView>>>,
    descriptor_set_layouts:
        HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::descriptor::DescriptorSetLayout>>>,
    pipeline_layouts: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::pipeline::PipelineLayout>>>,
    render_passes: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::pipeline::RenderPass>>>,
    shader_modules: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::pipeline::ShaderModule>>>,
    pipeline_caches: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::pipeline::PipelineCache>>>,
    pipelines: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::pipeline::Pipeline>>>,
    descriptor_pools:
        HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::descriptor::DescriptorPool>>>,
    descriptor_sets: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::descriptor::DescriptorSet>>>,
    framebuffers: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::pipeline::Framebuffer>>>,
}

/// Assigns an object to its registry in [`Context`].
#[macro_export]
macro_rules! impl_vulkan_object {
    ($object:ty, $container:ident, $handle:ty) => {
        impl $crate::context::VulkanObject for $object {
            type Handle = $handle;

            fn registry(
                context: &$crate::context::Context,
            ) -> &::std::collections::HashMap<
                Self::Handle,
                ::std::sync::Arc<::parking_lot::Mutex<Self>>,
            > {
                &context.$container
            }

            fn registry_mut(
                context: &mut $crate::context::Context,
            ) -> &mut ::std::collections::HashMap<
                Self::Handle,
                ::std::sync::Arc<::parking_lot::Mutex<Self>>,
            > {
                &mut context.$container
            }

            fn set_handle(&mut self, handle: Self::Handle) {
                self.handle = handle;
            }

            fn handle(&self) -> Self::Handle {
                self.handle
            }
        }
    };
}

impl_vulkan_object!(crate::instance::Instance, instances, VkDispatchableHandle);
impl_vulkan_object!(
    crate::physical_device::PhysicalDevice,
    physical_devices,
    VkDispatchableHandle
);
impl_vulkan_object!(
    crate::logical_device::LogicalDevice,
    logical_devices,
    VkDispatchableHandle
);
impl_vulkan_object!(crate::queue::Queue, queues, VkDispatchableHandle);
impl_vulkan_object!(
    crate::command_buffer::CommandBuffer,
    command_buffers,
    VkDispatchableHandle
);
impl_vulkan_object!(crate::fence::Fence, fences, VkNonDispatchableHandle);
impl_vulkan_object!(
    crate::semaphore::Semaphore,
    semaphores,
    VkNonDispatchableHandle
);
impl_vulkan_object!(crate::surface::Surface, surfaces, VkNonDispatchableHandle);
impl_vulkan_object!(
    crate::swapchain::Swapchain,
    swapchains,
    VkNonDispatchableHandle
);
impl_vulkan_object!(crate::image::Image, images, VkNonDispatchableHandle);
impl_vulkan_object!(
    crate::image::ImageView,
    image_views,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::command_buffer::CommandPool,
    command_pools,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::memory::MemoryAllocation,
    memory_allocations,
    VkNonDispatchableHandle
);
impl_vulkan_object!(crate::sampler::Sampler, samplers, VkNonDispatchableHandle);
impl_vulkan_object!(crate::buffer::Buffer, buffers, VkNonDispatchableHandle);
impl_vulkan_object!(
    crate::buffer::BufferView,
    buffer_views,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::descriptor::DescriptorSetLayout,
    descriptor_set_layouts,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::pipeline::PipelineLayout,
    pipeline_layouts,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::pipeline::RenderPass,
    render_passes,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::pipeline::ShaderModule,
    shader_modules,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::pipeline::PipelineCache,
    pipeline_caches,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::pipeline::Pipeline,
    pipelines,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::descriptor::DescriptorPool,
    descriptor_pools,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::descriptor::DescriptorSet,
    descriptor_sets,
    VkNonDispatchableHandle
);
impl_vulkan_object!(
    crate::pipeline::Framebuffer,
    framebuffers,
    VkNonDispatchableHandle
);

static CONTEXT: LazyLock<RwLock<Context>> = LazyLock::new(|| RwLock::new(Context::default()));

static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// A Vulkan handle that can be stored in [`Context`].
pub trait Handle: Copy + Eq + std::hash::Hash {
    /// Allocates a fresh handle value.
    fn allocate() -> Self;

    /// Releases resources owned by the handle itself.
    fn release(self);
}

impl Handle for VkDispatchableHandle {
    fn allocate() -> Self {
        let key = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let inner = Box::new(VkDispatchableHandleInner {
            loader_data: VkLoaderData {
                loader_magic: VkLoaderData::LOADER_MAGIC,
            },
            key,
        });
        // `Box::leak` keeps the loader data alive for as long as the handle.
        // `release` reclaims it.
        Self(NonNull::new(Box::leak(inner)))
    }

    fn release(self) {
        let Some(inner) = self.0 else {
            return;
        };
        // SAFETY: `allocate` leaked exactly one box, and `release` runs once
        // per handle when the object is removed from the registry.
        drop(unsafe { Box::from_raw(inner.as_ptr()) });
    }
}

impl Handle for VkNonDispatchableHandle {
    fn allocate() -> Self {
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(NonZeroU64::new(id))
    }

    fn release(self) {
        // Non-dispatchable handles carry no external allocation.
    }
}

/// An object that can be looked up by its Vulkan handle.
pub trait VulkanObject: Sized + Send + Sync {
    type Handle: Handle;

    fn registry(context: &Context) -> &HashMap<Self::Handle, Arc<Mutex<Self>>>;

    fn registry_mut(context: &mut Context) -> &mut HashMap<Self::Handle, Arc<Mutex<Self>>>;

    fn set_handle(&mut self, handle: Self::Handle);

    fn handle(&self) -> Self::Handle;

    /// Registers `self` and returns its fresh handle.
    fn register(self) -> Self::Handle {
        let mut context: RwLockWriteGuard<'_, Context> = CONTEXT.write();
        let handle = Self::Handle::allocate();
        let object = Arc::new(Mutex::new(self));
        Self::registry_mut(&mut context).insert(handle, Arc::clone(&object));
        object.lock().set_handle(handle);
        handle
    }

    /// Looks up a live object by handle.
    fn from_handle(handle: Self::Handle) -> Option<Arc<Mutex<Self>>> {
        let context = CONTEXT.read();
        Self::registry(&context).get(&handle).cloned()
    }

    /// Removes an object from the registry.
    fn drop_handle(handle: Self::Handle) {
        let object = {
            let mut context = CONTEXT.write();
            Self::registry_mut(&mut context).remove(&handle)
        };
        // Drop the object outside the registry lock so its destructor may lock
        // other objects without deadlocking.
        drop(object);
        handle.release();
    }
}
