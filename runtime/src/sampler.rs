//! Sampler

use crate::{Context, LogicalDevice, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Debug)]
pub struct Sampler {
    handle: VkNonDispatchableHandle,
    logical_device: Arc<Mutex<LogicalDevice>>,
    flags: VkSamplerCreateFlags,
}

impl Sampler {
    pub fn create(
        logical_device: Arc<Mutex<LogicalDevice>>,
        flags: VkSamplerCreateFlags,
    ) -> VkNonDispatchableHandle {
        info!("new Sampler");
        let handle = VK_NULL_HANDLE;

        let object = Self {
            handle,
            logical_device,
            flags,
        };
        object.register_object()
    }
}

impl NonDispatchable for Sampler {
    fn get_hash(context: &Context) -> &HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.samplers
    }

    fn get_hash_mut(
        context: &mut Context,
    ) -> &mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.samplers
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}
