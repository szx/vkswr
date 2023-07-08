//! XCB surface

use crate::{Context, Instance, NonDispatchable};
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::sync::Arc;
use xcb;

pub struct Surface {
    handle: VkNonDispatchableHandle,
    instance: Arc<Instance>,
    flags: VkXcbSurfaceCreateFlagsKHR,
    connection: ManuallyDrop<xcb::Connection>,
    window: ManuallyDrop<xcb::x::Window>,
}

impl Surface {
    pub fn create(
        instance: Arc<Instance>,
        create_info: &VkXcbSurfaceCreateInfoKHR,
    ) -> VkNonDispatchableHandle {
        info!("new Surface");
        let handle = VK_NULL_HANDLE;
        let instance = instance.clone();
        let flags = create_info.flags;
        let Some(connection) = create_info.connection else {
            unreachable!()
        };
        let connection =
            unsafe { ManuallyDrop::new(xcb::Connection::from_raw_conn(connection.as_ptr())) };
        let window = unsafe { ManuallyDrop::new(xcb::XidNew::new(create_info.window)) };

        let surface = Self {
            handle,
            instance,
            flags,
            connection,
            window,
        };
        surface.register_object()
    }
}

impl NonDispatchable for Surface {
    fn get_hash<'a>(
        context: &'a Context,
    ) -> &'a HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &context.surfaces
    }

    fn get_hash_mut<'a>(
        context: &'a mut Context,
    ) -> &'a mut HashMap<VkNonDispatchableHandle, Arc<Mutex<Self>>> {
        &mut context.surfaces
    }

    fn set_handle(&mut self, handle: VkNonDispatchableHandle) {
        self.handle = handle;
    }

    fn get_handle(&self) -> VkNonDispatchableHandle {
        self.handle
    }
}

impl Debug for Surface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("instance", &self.instance)
            .field("flags", &self.flags)
            .field("connection", &self.connection.get_raw_conn())
            .field("window", &self.window)
            .finish()
    }
}
