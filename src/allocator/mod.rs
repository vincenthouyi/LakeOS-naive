mod linked_list;
mod init_allocator;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

use crate::mutex::Mutex;

use init_allocator::InitAllocator;

pub struct GlobalAllocator(Mutex<InitAllocator>);

impl GlobalAllocator {
    pub fn add_mempool(&self, base: *mut u8, size: usize) {
        self.0
            .lock()
            .add_mempool(base, size)
    }

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .unwrap()
            .as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .dealloc(NonNull::new_unchecked(ptr), layout)
    }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc(ptr, layout)
    }
}

#[global_allocator]
pub static INIT_ALLOC: GlobalAllocator = GlobalAllocator(Mutex::new(InitAllocator::new()));

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}