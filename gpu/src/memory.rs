use hashbrown::HashMap;
use log::trace;
use std::ops::Range;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Default)]
pub struct Memory {
    // TODO: Usa bitmap allocator.
    allocations: HashMap<MemoryAllocationHandle, Vec<u8>>,
    allocation_index: AtomicU64,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::default(),
            allocation_index: AtomicU64::new(1),
        }
    }

    pub const fn memory_size_in_bytes() -> u64 {
        10 * 1024 * 1024
    }

    pub fn allocate_memory(&mut self, size: u64) -> MemoryAllocation {
        let handle = MemoryAllocationHandle(self.allocation_index.fetch_add(1, Ordering::Relaxed));
        self.allocations.insert(handle, vec![0; size as usize]);
        MemoryAllocation { handle, size }
    }

    pub fn free_memory(&mut self, memory_allocation: MemoryAllocation) {
        self.allocations.remove(&memory_allocation.handle);
    }

    pub fn get_memory(&self, memory: &impl MemoryHandle) -> &[u8] {
        self.allocations
            .get(&memory.memory_handle())
            .unwrap_or_else(|| unreachable!())
            .as_slice()
    }

    pub fn get_memory_mut(&mut self, memory: &impl MemoryHandle) -> &mut [u8] {
        self.allocations
            .get_mut(&memory.memory_handle())
            .unwrap_or_else(|| unreachable!())
            .as_mut_slice()
    }

    pub fn get_memory_many_mut<const N: usize>(
        &mut self,
        memories: &[&dyn MemoryHandle; N],
    ) -> [&mut Vec<u8>; N] {
        let ks = memories
            .iter()
            .map(|x| x.memory_handle())
            .collect::<Vec<_>>();
        let ks = ks.iter().collect::<Vec<_>>();
        self.allocations
            .get_many_mut::<MemoryAllocationHandle, N>(
                ks.as_slice().try_into().unwrap_or_else(|_| unreachable!()),
            )
            .unwrap_or_else(|| unreachable!())
    }

    pub fn copy_bytes(
        &mut self,
        src: &impl MemoryHandle,
        dst: &impl MemoryHandle,
        src_offset: u64,
        dst_offset: u64,
        size: u64,
    ) {
        let [src, dst] = self.get_memory_many_mut(&[src, dst]);
        let src = &src[src_offset as usize..(src_offset + size) as usize];
        let dst = &mut dst[dst_offset as usize..(dst_offset + size) as usize];
        dst.copy_from_slice(src);
    }

    pub fn write_bytes(&mut self, src: &[u8], dst: &impl MemoryHandle, dst_offset: u64) {
        let dst = self.get_memory_mut(dst);
        let size = src.len();

        if dst_offset as usize >= dst.len()
            || (dst_offset as usize).checked_add(size).is_none()
            || dst_offset as usize + size > dst.len()
        {
            trace!(
                "attempt to write bytes into incorrect range {}..({}+{}) (dst_len={})",
                dst_offset,
                dst_offset,
                size,
                dst.len()
            );
            return;
        }

        let dst_range = Range {
            start: dst_offset as usize,
            end: dst_offset as usize + size,
        };
        let dst = &mut dst[dst_range];
        dst.copy_from_slice(src);
    }

    pub fn read_bytes(&self, src: &impl MemoryHandle, offset: u64, size: u64) -> &[u8] {
        let src = self.get_memory(src);
        let offset = offset as usize;
        let size = size as usize;
        &src[offset..offset + size]
    }

    pub fn map_host(
        &mut self,
        memory_allocation: MemoryAllocation,
        offset: u64,
        size: u64,
    ) -> Option<NonNull<std::ffi::c_void>> {
        let memory = self
            .allocations
            .get_mut(&memory_allocation.handle)
            .unwrap_or_else(|| unreachable!());
        let ptr = memory[offset as usize..(offset + size) as usize].as_mut_ptr();
        NonNull::new(ptr as *mut std::ffi::c_void)
    }

    pub const fn unmap_host(&self, _memory_allocation: MemoryAllocation) {
        // No-op.
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryAllocation {
    handle: MemoryAllocationHandle,
    pub size: u64,
}

impl MemoryHandle for MemoryAllocation {
    fn memory_handle(&self) -> MemoryAllocationHandle {
        self.handle
    }
}

#[derive(Debug, Clone, Default)]
pub struct MemoryBinding {
    /// Thanks to Arc cloned resource binding points to the same MemoryAllocation
    memory_handle: Arc<AtomicU64>,
    pub offset: u64,
    pub size: u64,
}

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

impl MemoryHandle for MemoryBinding {
    fn memory_handle(&self) -> MemoryAllocationHandle {
        MemoryAllocationHandle(self.memory_handle.load(Ordering::Relaxed))
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct MemoryAllocationHandle(u64);

pub trait MemoryHandle {
    fn memory_handle(&self) -> MemoryAllocationHandle;
}
