//! XCB surface

use crate::{Context, Instance, NonDispatchableHandle};
use headers::vk_decls::*;
use log::*;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::sync::Arc;
use xcb;

pub struct Surface {
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
        let instance = instance.clone();
        let flags = create_info.flags;
        let Some(connection) = create_info.connection else {
            unreachable!()
        };
        let connection =
            unsafe { ManuallyDrop::new(xcb::Connection::from_raw_conn(connection.as_ptr())) };
        let window = unsafe { ManuallyDrop::new(xcb::XidNew::new(create_info.window)) };

        let surface = Self {
            instance,
            flags,
            connection,
            window,
        };
        surface.register_handle()
    }
}

impl NonDispatchableHandle for Surface {
    fn get_hash<'a>(context: &'a Context) -> &'a HashMap<VkNonDispatchableHandle, Self> {
        &context.surfaces
    }

    fn get_hash_mut<'a>(
        context: &'a mut Context,
    ) -> &'a mut HashMap<VkNonDispatchableHandle, Self> {
        &mut context.surfaces
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
