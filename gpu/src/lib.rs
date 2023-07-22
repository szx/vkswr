use hashbrown::HashMap;
use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Default)]
pub struct Gpu {
    // TODO: Usa bitmap allocator.
    memory_allocations: HashMap<MemoryBindingHandle, Vec<u8>>,
    memory_allocation_index: AtomicU64,
    render_targets: HashMap<RenderTargetIndex, RenderTarget>,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            memory_allocations: HashMap::default(),
            memory_allocation_index: AtomicU64::new(1),
            render_targets: HashMap::default(),
        }
    }

    pub const fn memory_size_in_bytes() -> u64 {
        10 * 1024 * 1024
    }

    pub fn allocate_memory(&mut self, size: u64) -> MemoryAllocation {
        let handle =
            MemoryBindingHandle(self.memory_allocation_index.fetch_add(1, Ordering::Relaxed));
        self.memory_allocations
            .insert(handle, vec![0; size as usize]);
        MemoryAllocation { handle, size }
    }

    pub fn free_memory(&mut self, memory_allocation: MemoryAllocation) {
        self.memory_allocations.remove(&memory_allocation.handle);
    }

    pub fn map_host(
        &mut self,
        memory_allocation: MemoryAllocation,
        offset: u64,
        size: u64,
    ) -> NonNull<std::ffi::c_void> {
        let memory = self
            .memory_allocations
            .get_mut(&memory_allocation.handle)
            .unwrap_or_else(|| unreachable!());
        let ptr = memory[offset as usize..(offset + size) as usize].as_mut_ptr();
        unsafe { NonNull::new_unchecked(ptr as *mut std::ffi::c_void) }
    }

    pub const fn unmap_host(&self, _memory_allocation: MemoryAllocation) {
        // No-op.
    }

    pub fn submit(&mut self, command_buffer: CommandBuffer) {
        // TODO: Just submit, mpsc event loop on other thread.
        for command in command_buffer.commands {
            match command {
                Command::CopyBufferToImage {
                    src_buffer,
                    dst_image,
                    region,
                } => {
                    self.copy_buffer_to_image(src_buffer, dst_image, region);
                }
                Command::CopyImageToBuffer {
                    src_image,
                    dst_buffer,
                    region,
                } => {
                    self.copy_image_to_buffer(src_image, dst_buffer, region);
                }
                Command::CopyBufferToBuffer {
                    src_buffer,
                    dst_buffer,
                    region,
                } => {
                    self.copy_buffer_to_buffer(src_buffer, dst_buffer, region);
                }
                Command::ExecuteCommands { command_buffer } => {
                    // TODO: Avoid submit recursion.
                    self.submit(command_buffer);
                }
                Command::BindRenderTarget { render_target } => {
                    self.bind_render_target(render_target);
                }
                Command::UnbindRenderTarget { index } => {
                    self.unbind_render_target(index);
                }
                Command::ClearRenderTarget {
                    index,
                    render_area,
                    color,
                } => {
                    self.clear_render_target(index, render_area, color);
                }
            }
        }
    }

    fn copy_bytes(
        &mut self,
        src_binding: &MemoryBinding,
        dst_binding: &MemoryBinding,
        src_offset: u64,
        dst_offset: u64,
        size: u64,
    ) {
        let [src, dst] = self
            .memory_allocations
            .get_many_mut([
                &MemoryBindingHandle(src_binding.memory_handle.load(Ordering::Relaxed)),
                &MemoryBindingHandle(dst_binding.memory_handle.load(Ordering::Relaxed)),
            ])
            .unwrap_or_else(|| unreachable!());
        let src = src[src_offset as usize..(src_offset + size) as usize].as_ptr();
        let dst = dst[dst_offset as usize..(dst_offset + size) as usize].as_mut_ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, size as usize);
        }
    }

    fn copy_buffer_to_image(
        &mut self,
        src_buffer: Buffer,
        dst_image: Image,
        region: RegionCopyBufferImage,
    ) {
        // TODO: Complete buffer to image copy algorithm.
        assert_eq!(region.buffer_offset, 0);
        assert_eq!(region.buffer_image_height, 0);
        assert_eq!(region.image_offset.x, 0);
        assert_eq!(region.image_offset.y, 0);
        assert_eq!(region.image_offset.z, 0);
        assert_eq!(region.image_mip_level, 0);
        assert_eq!(region.image_base_array_level, 0);
        assert_eq!(region.image_array_level_count, 1);
        assert_eq!(region.image_extent.depth, 1);
        let size =
            region.image_extent.width * region.image_extent.height * region.bytes_per_pixel as u32;
        let src_offset = region.buffer_offset;
        let dst_offset = 0;
        self.copy_bytes(
            &src_buffer.binding,
            &dst_image.binding,
            src_offset,
            dst_offset,
            size as u64,
        );
    }

    fn copy_image_to_buffer(
        &mut self,
        src_image: Image,
        dst_buffer: Buffer,
        region: RegionCopyBufferImage,
    ) {
        // TODO: Complete buffer to image copy algorithm.
        assert_eq!(region.buffer_offset, 0);
        assert_eq!(region.buffer_image_height, 0);
        assert_eq!(region.image_offset.x, 0);
        assert_eq!(region.image_offset.y, 0);
        assert_eq!(region.image_offset.z, 0);
        assert_eq!(region.image_mip_level, 0);
        assert_eq!(region.image_base_array_level, 0);
        assert_eq!(region.image_array_level_count, 1);
        assert_eq!(region.image_extent.depth, 1);
        let size =
            region.image_extent.width * region.image_extent.height * region.bytes_per_pixel as u32;
        let src_offset = 0;
        let dst_offset = region.buffer_offset;
        self.copy_bytes(
            &src_image.binding,
            &dst_buffer.binding,
            src_offset,
            dst_offset,
            size as u64,
        );
    }

    fn copy_buffer_to_buffer(
        &mut self,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region: RegionCopyBufferBuffer,
    ) {
        self.copy_bytes(
            &src_buffer.binding,
            &dst_buffer.binding,
            region.src_offset,
            region.dst_offset,
            region.size,
        );
    }

    fn bind_render_target(&mut self, rt: RenderTarget) {
        self.render_targets.insert(rt.index, rt);
    }

    fn unbind_render_target(&mut self, index: RenderTargetIndex) {
        self.render_targets.remove(&index);
    }

    fn clear_render_target(&mut self, index: RenderTargetIndex, area: RenderArea, color: Color) {
        let rt = self
            .render_targets
            .get(&index)
            .unwrap_or_else(|| unreachable!());
        assert_eq!(rt.samples, 1);
        assert!(area.offset.x >= 0);
        assert!(area.offset.y >= 0);

        let bytes_per_pixel = rt.format.bytes_per_pixel();
        let dst_offset = rt.image.extent.width * area.offset.y as u32 * bytes_per_pixel as u32;
        let dst = self
            .memory_allocations
            .get_mut(&MemoryBindingHandle(
                rt.image.binding.memory_handle.load(Ordering::Relaxed),
            ))
            .unwrap_or_else(|| unreachable!());
        let mut dst = dst[dst_offset as usize..].as_mut_ptr();
        let src = color.to_bytes(rt.format);

        unsafe {
            let src = src.as_ptr();
            for _y in 0..area.extent.height {
                dst = dst.offset(area.offset.x as isize * bytes_per_pixel as isize);
                for _x in 0..area.extent.width {
                    std::ptr::copy_nonoverlapping(src, dst, bytes_per_pixel as usize);
                    dst = dst.offset(bytes_per_pixel as isize);
                }
            }
        }
    }
}

impl Debug for Gpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gpu")
            .field("memory_allocation_index", &self.memory_allocation_index)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct CommandBuffer {
    commands: Vec<Command>,
}

impl CommandBuffer {
    pub const fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn record(&mut self, command: Command) {
        self.commands.push(command);
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    CopyBufferToImage {
        src_buffer: Buffer,
        dst_image: Image,
        region: RegionCopyBufferImage,
    },
    CopyImageToBuffer {
        src_image: Image,
        dst_buffer: Buffer,
        region: RegionCopyBufferImage,
    },
    CopyBufferToBuffer {
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region: RegionCopyBufferBuffer,
    },
    ExecuteCommands {
        command_buffer: CommandBuffer,
    },
    BindRenderTarget {
        render_target: RenderTarget,
    },
    UnbindRenderTarget {
        index: RenderTargetIndex,
    },
    ClearRenderTarget {
        index: RenderTargetIndex,
        render_area: RenderArea,
        color: Color,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct RegionCopyBufferImage {
    pub buffer_offset: u64,
    pub buffer_row_len: u32,
    pub buffer_image_height: u32,

    pub image_mip_level: u32,
    pub image_base_array_level: u32,
    pub image_array_level_count: u32,
    pub image_offset: Offset3d,
    pub image_extent: Extent3d,

    pub bytes_per_pixel: u8, // TODO: Replace with image_format
}

#[derive(Debug, Copy, Clone)]
pub struct RegionCopyBufferBuffer {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

#[derive(Debug, Copy, Clone)]
pub enum Format {
    R8Unorm,
    R8g8b8a8Unorm,
    A2b10g10r10UnormPack32,
    D16Unorm,
}

impl Format {
    pub const fn bytes_per_pixel(&self) -> u8 {
        match *self {
            Format::R8Unorm => 1,
            Format::R8g8b8a8Unorm => 4,
            Format::A2b10g10r10UnormPack32 => 4,
            Format::D16Unorm => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderTarget {
    pub index: RenderTargetIndex,
    pub format: Format,
    pub samples: u32,
    pub image: Image,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct RenderTargetIndex(pub usize);

// TODO: struct RenderInput for Vulkan input attachments.

#[derive(Debug, Copy, Clone)]
pub struct RenderArea {
    pub extent: Extent2d,
    pub offset: Offset2d,
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub binding: MemoryBinding,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub binding: MemoryBinding,
    pub extent: Extent3d,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryBinding {
    /// Thanks to Arc cloned resource binding points to the same MemoryAllocation
    memory_handle: Arc<AtomicU64>,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryAllocation {
    handle: MemoryBindingHandle,
    pub size: u64,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct MemoryBindingHandle(pub u64);

impl MemoryBinding {
    pub fn new() -> Self {
        Self {
            memory_handle: Arc::new(Default::default()),
            offset: 0,
            size: 0,
        }
    }

    pub fn store(&mut self, memory_allocation: MemoryAllocation, offset: u64, size: u64) {
        self.memory_handle
            .store(memory_allocation.handle.0, Ordering::Relaxed);
        self.offset = offset;
        self.size = size;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Offset3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Offset2d {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Extent2d {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl Color {
    fn to_bytes(&self, format: Format) -> Vec<u8> {
        let mut result = vec![0u8; format.bytes_per_pixel() as usize];
        let unorm_8 = |val: u32| -> u8 {
            let val = f32::from_bits(val);
            (val * 255.0f32).round() as u8
        };
        let unorm_16 = |result: &mut [u8], val: u32| {
            let val = f32::from_bits(val);
            let val = (val * 65535.0f32).round() as u16;
            result.copy_from_slice(&val.to_ne_bytes());
        };
        match format {
            Format::R8Unorm => {
                // TODO: Clearer way to put numbers into Vec during swizzling.
                result[0] = self.r as u8;
            }
            Format::R8g8b8a8Unorm => {
                result[0] = unorm_8(self.r);
                result[1] = unorm_8(self.g);
                result[2] = unorm_8(self.b);
                result[3] = unorm_8(self.a);
            }
            Format::A2b10g10r10UnormPack32 => {
                unimplemented!()
            }
            Format::D16Unorm => {
                unorm_16(&mut result[0..2], self.r);
            }
        }
        result
    }
}

#[test]
pub fn foo() {
    println!("foo")
}
