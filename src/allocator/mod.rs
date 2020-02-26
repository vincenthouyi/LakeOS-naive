mod vm_allocator;
mod object_allocator;

use core::alloc::{GlobalAlloc, Layout, AllocErr};
use core::ptr::NonNull;

use rustyl4api::object::RamObj;

use crate::mutex::Mutex;

use vm_allocator::VmAllocator;
use object_allocator::ObjectAllocator;

pub struct GlobalAllocator {
    pub vm_alloc: Mutex<VmAllocator>,
    pub object_alloc: Mutex<ObjectAllocator>,
    pub sys_brk: Mutex<Option<usize>>,
}

impl GlobalAllocator {
    pub const fn uninitialized() -> Self {
        Self {
            vm_alloc: Mutex::new(VmAllocator::new()),
            object_alloc: Mutex::new(ObjectAllocator::new()),
            sys_brk: Mutex::new(None),
        }
    }

    pub fn initialize(&self, brk: usize) {
        use core::ops::DerefMut;
        *self.sys_brk.lock().deref_mut() = Some(brk);
    }

    pub fn add_mempool(&self, base: *mut u8, size: usize) {
        self.vm_alloc
            .lock()
            .add_mempool(base, size)
    }

    pub fn try_alloc(&self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        self.vm_alloc
            .lock()
            .alloc(layout)
    }

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        use core::ops::DerefMut;
        use core::ptr::null_mut;
        use rustyl4api::init::InitCSpaceSlot::InitL1PageTable;
        use rustyl4api::vspace::{FRAME_BIT_SIZE, FRAME_SIZE, Permission};

        if layout.size() > FRAME_SIZE {
            return null_mut();
        }

        let mut ptr = self.try_alloc(layout);

        if ptr.is_err() {
            let ram_cap = self.object_alloc.lock().utspace_alloc::<RamObj>(FRAME_BIT_SIZE).unwrap();
            let mut brk_lock = self.sys_brk.lock();
            let brk = brk_lock.deref_mut().as_mut().unwrap();
            ram_cap.map(InitL1PageTable as usize, *brk, Permission::new(true, true, false)).unwrap();
            self.add_mempool(*brk as *mut u8, FRAME_SIZE);
            ptr = self.try_alloc(layout);
            *brk += FRAME_SIZE;
        }

        ptr.map(|p| p.as_ptr())
           .unwrap_or(null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.vm_alloc
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
pub static INIT_ALLOC: GlobalAllocator = GlobalAllocator::uninitialized();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}