//! XCB surface

use crate::{Context, NonDispatchableHandle};
use headers::vk_decls::*;
use log::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Surface {
    flags: VkXcbSurfaceCreateFlagsKHR,
    // TODO: Get xcb types from create info.
}

impl Surface {
    pub fn create(create_info: &VkXcbSurfaceCreateInfoKHR) -> VkNonDispatchableHandle {
        info!("new Surface");
        let flags = create_info.flags;

        let surface = Self { flags };
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
