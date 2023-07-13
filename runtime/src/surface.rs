//! XCB surface

use crate::context::NonDispatchable;
use crate::instance::Instance;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::sync::Arc;
use xcb;

pub struct Surface {
    pub(crate) handle: VkNonDispatchableHandle,
    instance: Arc<Mutex<Instance>>,
    flags: VkXcbSurfaceCreateFlagsKHR,
    connection: ManuallyDrop<xcb::Connection>,
    window: ManuallyDrop<xcb::x::Window>,
}

impl Surface {
    pub fn create(
        instance: Arc<Mutex<Instance>>,
        create_info: &VkXcbSurfaceCreateInfoKHR,
    ) -> VkNonDispatchableHandle {
        info!("new Surface");
        let handle = VK_NULL_HANDLE;
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
