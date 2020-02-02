use core::alloc::{GlobalAlloc, Layout};
use core::cell::Cell;

pub struct InitAllocator {
    leaky_mempool_base: Cell<usize>,
    leaky_mempool_top: Cell<usize>,
}

unsafe impl core::marker::Sync for InitAllocator {}

impl InitAllocator {
    pub const fn uninitialized() -> Self {
        InitAllocator {
            leaky_mempool_base: Cell::new(0),
            leaky_mempool_top: Cell::new(0),
        }
    }

    pub fn set_leaky_mempool(&self, base: *mut u8, size: usize) {
        self.leaky_mempool_base.set(base as usize);
        self.leaky_mempool_top.set(base as usize + size);
    }
}

unsafe impl GlobalAlloc for InitAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        use core::ptr::null_mut;

        let cur_ptr = self.leaky_mempool_base.get();
        let new_ptr = align_up(cur_ptr, layout.align()).saturating_add(layout.size());
        let mempool_top = self.leaky_mempool_top.get();

        if new_ptr >= mempool_top {
            return null_mut();
        }

        self.leaky_mempool_base.set(new_ptr);
        (new_ptr - layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
pub static INIT_ALLOC: InitAllocator = InitAllocator::uninitialized();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if !align.is_power_of_two() {
        panic!("align is not power of 2");
    }

    addr & !(align - 1)
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr.saturating_add(align - 1), align)
}