//! XCB surface

use crate::context::NonDispatchable;
use crate::instance::Instance;
use crate::memory::MemoryAllocation;
use headers::vk_decls::*;
use log::*;
use parking_lot::Mutex;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::sync::Arc;
use xcb;
use xcb::x;

pub struct Surface {
    pub(crate) handle: VkNonDispatchableHandle,
    instance: Arc<Mutex<Instance>>,
    flags: VkXcbSurfaceCreateFlagsKHR,
    connection: ManuallyDrop<xcb::Connection>,
    window: ManuallyDrop<x::Window>,

    gc: Option<x::Gcontext>,
    depth: Option<u8>,
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
            gc: None,
            depth: None,
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

impl Surface {
    pub fn present(
        &mut self,
        memory_allocation: Arc<Mutex<MemoryAllocation>>,
        extent: gpu::Extent3<u32>,
    ) -> Result<VkResult, VkResult> {
        let (gc, depth) = if let (Some(gc), Some(depth)) = (self.gc, self.depth) {
            (gc, depth)
        } else {
            self.connection
                .send_and_check_request(&x::ChangeWindowAttributes {
                    window: *self.window,
                    value_list: &[x::Cw::EventMask(x::EventMask::EXPOSURE)],
                })
                .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;
            let gc = self.connection.generate_id();
            self.connection
                .send_and_check_request(&x::CreateGc {
                    cid: gc,
                    drawable: x::Drawable::Window(*self.window),
                    value_list: &[x::Gc::GraphicsExposures(true)],
                })
                .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;
            self.connection
                .send_and_check_request(&x::MapWindow {
                    window: *self.window,
                })
                .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;
            self.connection
                .flush()
                .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;

            let cookie = self.connection.send_request(&x::GetGeometry {
                drawable: x::Drawable::Window(*self.window),
            });
            let reply = self
                .connection
                .wait_for_reply(cookie)
                .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;
            let depth = reply.depth();

            (gc, depth)
        };

        let mut memory_allocation = memory_allocation.lock();
        let size = memory_allocation.gpu_memory_allocation.size;
        assert!(size < self.connection.get_maximum_request_length() as u64 * 4);
        let data = memory_allocation
            .map_host(0, size)
            .map_err(|_| VkResult::VK_ERROR_OUT_OF_DATE_KHR)?;
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *mut u8, size as usize) };

        // TODO: VK_ERROR_OUT_OF_DATE_KHR
        // TODO: Use X Present Extension.
        self.connection
            .send_and_check_request(&x::PutImage {
                format: x::ImageFormat::ZPixmap,
                drawable: x::Drawable::Window(*self.window),
                gc,
                width: extent.width as u16,
                height: extent.height as u16,
                dst_x: 0,
                dst_y: 0,
                left_pad: 0,
                depth,
                data,
            })
            .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;

        self.connection
            .flush()
            .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;

        memory_allocation.unmap_host();
        drop(memory_allocation);
        Ok(VkResult::VK_SUCCESS)
    }
}
