//! XCB surface and presentation to an X11 window.

use crate::context::VulkanObject;
use crate::instance::Instance;
use crate::memory::MemoryAllocation;
use common::math::Extent3;
use headers::vk_decls::*;
use log::{info, warn};
use parking_lot::Mutex;
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;
use std::sync::Arc;
use xcb::XidNew;
use xcb::x;

pub struct Surface {
    pub(crate) handle: VkNonDispatchableHandle,
    #[allow(dead_code)]
    instance: Arc<Mutex<Instance>>,
    #[allow(dead_code)]
    flags: VkXcbSurfaceCreateFlagsKHR,
    /// Borrowed from the application: dropping it would close their connection.
    connection: ManuallyDrop<xcb::Connection>,
    window: x::Window,
    gc: Option<x::Gcontext>,
    depth: Option<u8>,
}

impl Surface {
    pub fn create(
        instance: Arc<Mutex<Instance>>,
        create_info: &VkXcbSurfaceCreateInfoKHR,
    ) -> VkNonDispatchableHandle {
        info!("new Surface");
        let connection = create_info.connection.expect("xcb connection is not null");
        // SAFETY: Vulkan takes ownership of the surface, not of the XCB
        // connection. The pointer is valid for the lifetime of the application
        // connection, and `ManuallyDrop` prevents closing it.
        let connection = unsafe { xcb::Connection::from_raw_conn(connection.as_ptr()) };
        let connection = ManuallyDrop::new(connection);
        let window = x::Window::new(create_info.window);

        Self {
            handle: VK_NULL_HANDLE,
            instance,
            flags: create_info.flags,
            connection,
            window,
            gc: None,
            depth: None,
        }
        .register()
    }
}

impl Debug for Surface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Surface")
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
        extent: Extent3<u32>,
    ) -> Result<VkResult, VkResult> {
        let (gc, depth) = match (self.gc, self.depth) {
            (Some(gc), Some(depth)) => (gc, depth),
            _ => self.create_graphics_context()?,
        };

        let mut memory_allocation = memory_allocation.lock();
        let size = memory_allocation.gpu_memory_allocation.size;
        assert!(size < self.connection.get_maximum_request_length() as u64 * 4);
        let data = memory_allocation
            .map_host(0, size)
            .map_err(|_| VkResult::VK_ERROR_OUT_OF_DATE_KHR)?;
        let data_length = usize::try_from(size).expect("framebuffer fits in host memory");
        // SAFETY: `map_host` points at `size` bytes that stay mapped until
        // `unmap_host` is called below.
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr().cast::<u8>(), data_length) };

        // TODO: VK_ERROR_OUT_OF_DATE_KHR
        // TODO: Use the X Present extension.
        self.connection
            .send_and_check_request(&x::PutImage {
                format: x::ImageFormat::ZPixmap,
                drawable: x::Drawable::Window(self.window),
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

    fn create_graphics_context(&mut self) -> Result<(x::Gcontext, u8), VkResult> {
        self.connection
            .send_and_check_request(&x::ChangeWindowAttributes {
                window: self.window,
                value_list: &[x::Cw::EventMask(x::EventMask::EXPOSURE)],
            })
            .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;
        let gc = self.connection.generate_id();
        self.connection
            .send_and_check_request(&x::CreateGc {
                cid: gc,
                drawable: x::Drawable::Window(self.window),
                value_list: &[x::Gc::GraphicsExposures(true)],
            })
            .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;
        self.connection
            .send_and_check_request(&x::MapWindow {
                window: self.window,
            })
            .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;
        self.connection
            .flush()
            .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;

        let cookie = self.connection.send_request(&x::GetGeometry {
            drawable: x::Drawable::Window(self.window),
        });
        let reply = self
            .connection
            .wait_for_reply(cookie)
            .map_err(|_| VkResult::VK_ERROR_SURFACE_LOST_KHR)?;
        let depth = reply.depth();

        self.gc = Some(gc);
        self.depth = Some(depth);
        warn!("TODO: Handle window resize events");
        Ok((gc, depth))
    }
}
