//! CPU-side storage backing GPU memory allocations.

use std::collections::HashMap;
use std::ops::Range;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};

use common::graphics::MemoryBinding;
use log::trace;

/// A simple bump-allocated pool of host memory.
///
/// TODO: Replace this with a real bitmap allocator.
pub struct Memory {
    allocations: HashMap<MemoryAllocationHandle, Vec<u8>>,
    allocation_index: AtomicU64,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            allocations: HashMap::new(),
            allocation_index: AtomicU64::new(1),
        }
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Memory")
            .field("allocation_count", &self.allocation_count())
            .finish_non_exhaustive()
    }
}

impl Memory {
    pub const fn memory_size_in_bytes() -> u64 {
        10 * 1024 * 1024
    }

    /// Number of live allocations, used by the debug representation.
    pub fn allocation_count(&self) -> usize {
        self.allocations.len()
    }

    pub fn allocate_memory(&mut self, size: u64) -> MemoryAllocation {
        let handle = MemoryAllocationHandle(self.allocation_index.fetch_add(1, Ordering::Relaxed));
        let size = usize::try_from(size).expect("allocation fits in host memory");
        self.allocations.insert(handle, vec![0; size]);
        MemoryAllocation {
            handle,
            size: size as u64,
        }
    }

    pub fn free_memory(&mut self, memory_allocation: MemoryAllocation) {
        self.allocations.remove(&memory_allocation.handle);
    }

    pub fn get_memory(&self, memory: &impl MemoryHandle) -> &[u8] {
        self.allocations
            .get(&memory.memory_handle())
            .expect("memory handle refers to a live allocation")
    }

    pub fn get_memory_mut(&mut self, memory: &impl MemoryHandle) -> &mut [u8] {
        self.allocations
            .get_mut(&memory.memory_handle())
            .expect("memory handle refers to a live allocation")
    }

    /// Returns mutable references to `N` distinct allocations.
    pub fn get_memory_many_mut<const N: usize>(
        &mut self,
        memories: &[&dyn MemoryHandle; N],
    ) -> [&mut Vec<u8>; N] {
        let handles: [MemoryAllocationHandle; N] =
            std::array::from_fn(|index| memories[index].memory_handle());
        let allocations = self
            .allocations
            .get_disjoint_mut(std::array::from_fn(|index| &handles[index]));
        allocations.map(|allocation| allocation.expect("memory handle refers to a live allocation"))
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
        let src_len = src.len();
        let dst_len = dst.len();
        let src = &src[index_range(src_offset, size, src_len)];
        let dst = &mut dst[index_range(dst_offset, size, dst_len)];
        dst.copy_from_slice(src);
    }

    pub fn write_bytes(&mut self, src: &[u8], dst: &impl MemoryHandle, dst_offset: u64) {
        let dst_len = self.get_memory(dst).len();
        let Some(range) = checked_range(dst_offset, src.len() as u64, dst_len) else {
            trace!(
                "attempt to write bytes into incorrect range {dst_offset}..({dst_offset}+{}) \
                 (dst_len={dst_len})",
                src.len()
            );
            return;
        };
        self.get_memory_mut(dst)[range].copy_from_slice(src);
    }

    pub fn read_bytes(&self, src: &impl MemoryHandle, offset: u64, size: u64) -> &[u8] {
        let src = self.get_memory(src);
        &src[index_range(offset, size, src.len())]
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
            .expect("memory handle refers to a live allocation");
        let range = index_range(offset, size, memory.len());
        NonNull::new(memory[range].as_mut_ptr().cast())
    }

    pub const fn unmap_host(&self, _memory_allocation: MemoryAllocation) {
        // No-op: host memory stays resident.
    }
}

fn index_range(offset: u64, size: u64, len: usize) -> Range<usize> {
    checked_range(offset, size, len).expect("memory range is inside the allocation")
}

fn checked_range(offset: u64, size: u64, len: usize) -> Option<Range<usize>> {
    let offset = usize::try_from(offset).ok()?;
    let end = offset.checked_add(usize::try_from(size).ok()?)?;
    (end <= len).then_some(offset..end)
}

/// A live allocation inside [`Memory`].
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

impl MemoryHandle for MemoryBinding {
    fn memory_handle(&self) -> MemoryAllocationHandle {
        MemoryAllocationHandle(self.memory_handle.load(Ordering::Relaxed))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MemoryAllocationHandle(pub u64);

pub trait MemoryHandle {
    fn memory_handle(&self) -> MemoryAllocationHandle;
}

pub trait MemoryHandleStore {
    fn store(&mut self, memory_allocation: MemoryAllocation, offset: u64, size: u64);
}

impl MemoryHandleStore for MemoryBinding {
    fn store(&mut self, memory_allocation: MemoryAllocation, offset: u64, size: u64) {
        self.memory_handle
            .store(memory_allocation.handle.0, Ordering::Relaxed);
        self.offset = offset;
        self.size = size;
    }
}
