//! Software implementation of the GPU-facing half of the Vulkan API.

pub mod gpu;
pub mod graphics_pipeline;
pub mod memory;
pub mod rasterization;

pub use gpu::{Command, CommandBuffer, Gpu, RegionCopyBufferBuffer, RegionCopyBufferImage};
pub use graphics_pipeline::{
    GraphicsPipeline, InputAssemblyState, PrimitiveTopology, RasterizationState, RenderArea,
    RenderTarget, RenderTargetIndex, Scissor, Viewport, ViewportIndex, ViewportState,
};
pub use memory::{
    Memory, MemoryAllocation, MemoryAllocationHandle, MemoryHandle, MemoryHandleStore,
};
pub use rasterization::{draw_line_bresenham, draw_points, draw_triangle};
