//! Context

use crate::impl_dispatchable_trait;
use crate::impl_non_dispatchable_trait;
use headers::vk_decls::*;
use lazy_static::lazy_static;

use parking_lot::{Mutex, RwLock, RwLockWriteGuard};
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Context {
    // TODO: Better way to do concurrency than Arc<Mutex<_>>?
    instances: HashMap<VkDispatchableHandle, Arc<Mutex<crate::instance::Instance>>>,
    physical_devices:
        HashMap<VkDispatchableHandle, Arc<Mutex<crate::physical_device::PhysicalDevice>>>,
    logical_devices:
        HashMap<VkDispatchableHandle, Arc<Mutex<crate::logical_device::LogicalDevice>>>,
    queues: HashMap<VkDispatchableHandle, Arc<Mutex<crate::queue::Queue>>>,
    fences: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::fence::Fence>>>,
    semaphores: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::semaphore::Semaphore>>>,
    surfaces: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::surface::Surface>>>,
    swapchains: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::swapchain::Swapchain>>>,
    images: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::image::Image>>>,
    image_views: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::image::ImageView>>>,
    command_pools: HashMap<VkNonDispatchableHandle, Arc<Mutex<crate::command_buffer::CommandPool>>>,
    command_buffers:
        HashMap<VkDispatchableHandle, Arc<Mutex<crate::command_buffer::CommandBuffer>>>,
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

impl_dispatchable_trait!(crate::instance::Instance, instances);
impl_dispatchable_trait!(crate::physical_device::PhysicalDevice, physical_devices);
impl_dispatchable_trait!(crate::logical_device::LogicalDevice, logical_devices);
impl_dispatchable_trait!(crate::queue::Queue, queues);
impl_non_dispatchable_trait!(crate::fence::Fence, fences);
impl_non_dispatchable_trait!(crate::semaphore::Semaphore, semaphores);
impl_non_dispatchable_trait!(crate::surface::Surface, surfaces);
impl_non_dispatchable_trait!(crate::swapchain::Swapchain, swapchains);
impl_non_dispatchable_trait!(crate::image::Image, images);
impl_non_dispatchable_trait!(crate::image::ImageView, image_views);
impl_non_dispatchable_trait!(crate::command_buffer::CommandPool, command_pools);
impl_dispatchable_trait!(crate::command_buffer::CommandBuffer, command_buffers);
impl_non_dispatchable_trait!(crate::memory::MemoryAllocation, memory_allocations);
impl_non_dispatchable_trait!(crate::sampler::Sampler, samplers);
impl_non_dispatchable_trait!(crate::buffer::Buffer, buffers);
impl_non_dispatchable_trait!(crate::buffer::BufferView, buffer_views);
impl_non_dispatchable_trait!(
    crate::descriptor::DescriptorSetLayout,
    descriptor_set_layouts
);
impl_non_dispatchable_trait!(crate::pipeline::PipelineLayout, pipeline_layouts);
impl_non_dispatchable_trait!(crate::pipeline::RenderPass, render_passes);
impl_non_dispatchable_trait!(crate::pipeline::ShaderModule, shader_modules);
impl_non_dispatchable_trait!(crate::pipeline::PipelineCache, pipeline_caches);
impl_non_dispatchable_trait!(crate::pipeline::Pipeline, pipelines);
impl_non_dispatchable_trait!(crate::descriptor::DescriptorPool, descriptor_pools);
impl_non_dispatchable_trait!(crate::descriptor::DescriptorSet, descriptor_sets);
impl_non_dispatchable_trait!(crate::pipeline::Framebuffer, framebuffers);

#[macro_export]
macro_rules! impl_non_dispatchable_trait {
    ($object:ty, $container:ident) => {
        impl NonDispatchable for $object {
            fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
                &context.$container
            }

            fn get_hash_mut(
                context: &mut Context,
            ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
                &mut context.$container
            }

            fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
                self.handle = handle;
            }

            fn get_handle(&self) -> VkNonDispatchableHandle {
                self.handle
            }
        }
    };
}

#[macro_export]
macro_rules! impl_dispatchable_trait {
    ($object:ty, $container:ident) => {
        impl Dispatchable for $object {
            fn get_hash(context: &Context) -> &HashMap<VkDispatchableHandle, Arc<Mutex<Self>>> {
                &context.$container
            }

            fn get_hash_mut(
                context: &mut Context,
            ) -> &mut HashMap<VkDispatchableHandle, Arc<Mutex<Self>>> {
                &mut context.$container
            }

            fn set_handle(&mut self, handle: VkDispatchableHandle) {
                self.handle = handle;
            }

            fn get_handle(&self) -> VkDispatchableHandle {
                self.handle
            }
        }
    };
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }
}

lazy_static! {
    static ref CONTEXT: RwLock<Context> = RwLock::new(Context::new());
}

static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub trait Dispatchable<T = Self>
where
    Self: Sized + Send + Sync,
{
    fn get_hash(context: &Context) -> &HashMap<VkDispatchableHandle, Arc<Mutex<Self>>>;

    fn get_hash_mut(context: &mut Context) -> &mut HashMap<VkDispatchableHandle, Arc<Mutex<Self>>>;

    fn set_handle(&mut self, handle: VkDispatchableHandle);

    fn get_handle(&self) -> VkDispatchableHandle;

    fn register_object(self) -> VkDispatchableHandle {
        let mut context: RwLockWriteGuard<'_, _> = CONTEXT.write();
        let handle = VkDispatchableHandle(NonNull::new(Box::leak(Box::new(
            VkDispatchableHandleInner {
                loader_data: VkLoaderData {
                    loader_magic: VkLoaderData::LOADER_MAGIC,
                },
                key: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            },
        ))));
        let object = Arc::new(Mutex::new(self));
        Self::get_hash_mut(&mut context).insert(handle, object.clone());
        object.lock().set_handle(handle);
        handle
    }

    fn from_handle(handle: VkDispatchableHandle) -> Option<Arc<Mutex<Self>>> {
        let context = CONTEXT.read();
        Self::get_hash(&context).get(&handle).cloned()
    }

    fn drop_handle(handle: VkDispatchableHandle) {
        let mut context = CONTEXT.write();
        Self::get_hash_mut(&mut context).remove(&handle);
        let inner = unsafe { Box::from_raw(handle.0.expect("null handle").as_ptr()) };
        drop(inner);
    }
}

pub trait NonDispatchable<T = Self>
where
    Self: Sized + Send + Sync,
{
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>>;

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>>;

    fn set_handle(&mut self, handle: VkNonDispatchableHandle);
    fn get_handle(&self) -> VkNonDispatchableHandle;

    fn register_object(self) -> VkNonDispatchableHandle {
        let mut context: RwLockWriteGuard<'_, _> = CONTEXT.write();
        let handle =
            VkNonDispatchableHandle(NonZeroU64::new(ID_COUNTER.fetch_add(1, Ordering::Relaxed)));
        let object = Arc::new(Mutex::new(self));
        Self::get_hash_mut(&mut context).insert(handle, object.clone());
        object.lock().set_handle(handle);
        handle
    }

    fn from_handle(handle: VkNonDispatchableHandle) -> Option<Arc<Mutex<Self>>> {
        let context = CONTEXT.read();
        Self::get_hash(&context).get(&handle).cloned()
    }

    fn drop_handle(handle: VkNonDispatchableHandle) {
        let mut context = CONTEXT.write();
        Self::get_hash_mut(&mut context).remove(&handle);
    }
}
