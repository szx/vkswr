use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct Gpu {
    // TODO: Usa bitmap allocator.
    memory_allocations: HashMap<u64, Vec<u8>>,
    memory_allocation_index: AtomicU64,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            memory_allocations: HashMap::default(),
            memory_allocation_index: AtomicU64::new(1),
        }
    }

    pub const fn memory_size_in_bytes() -> u64 {
        10 * 1024 * 1024
    }

    pub fn allocate_memory(&mut self, size: u64) -> MemoryAllocation {
        let handle = self.memory_allocation_index.fetch_add(1, Ordering::Relaxed);
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
            .unwrap();
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
                    let size = region.image_extent.width
                        * region.image_extent.height
                        * region.bytes_per_pixel as u32;
                    let src_offset = region.buffer_offset;
                    let dst_offset = 0;
                    self.copy_bytes(
                        src_buffer.memory_allocation,
                        dst_image.memory_allocation,
                        src_offset,
                        dst_offset,
                        size as u64,
                    );
                }
                Command::CopyImageToBuffer {
                    src_image,
                    dst_buffer,
                    region,
                } => {
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
                    let size = region.image_extent.width
                        * region.image_extent.height
                        * region.bytes_per_pixel as u32;
                    let src_offset = 0;
                    let dst_offset = region.buffer_offset;
                    self.copy_bytes(
                        src_image.memory_allocation,
                        dst_buffer.memory_allocation,
                        src_offset,
                        dst_offset,
                        size as u64,
                    );
                }
                Command::CopyBufferToBuffer {
                    src_buffer,
                    dst_buffer,
                    region,
                } => {
                    self.copy_bytes(
                        src_buffer.memory_allocation,
                        dst_buffer.memory_allocation,
                        region.src_offset,
                        region.dst_offset,
                        region.size,
                    );
                }
                Command::ExecuteCommands { command_buffer } => {
                    // TODO: Avoid submit recursion.
                    self.submit(command_buffer);
                }
            }
        }
    }

    fn copy_bytes(
        &mut self,
        src: MemoryAllocation,
        dst: MemoryAllocation,
        src_offset: u64,
        dst_offset: u64,
        size: u64,
    ) {
        let src = self.memory_allocations.get_mut(&src.handle).unwrap();
        let src = src[src_offset as usize..(src_offset + size) as usize].as_mut_ptr();
        let dst = self.memory_allocations.get_mut(&dst.handle).unwrap();
        let dst = dst[dst_offset as usize..(dst_offset + size) as usize].as_mut_ptr();
        unsafe {
            std::ptr::copy(src, dst, size as usize);
        }
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

    pub bytes_per_pixel: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct RegionCopyBufferBuffer {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct Buffer {
    pub memory_allocation: MemoryAllocation,
}

#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub memory_allocation: MemoryAllocation,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryAllocation {
    handle: u64,
    pub size: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct Offset3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[test]
pub fn foo() {
    println!("foo")
}
