mod vm_allocator;
mod object_allocator;
mod vspace_allocator;

use core::alloc::{GlobalAlloc, Layout, AllocErr};
use core::ptr::NonNull;

use rustyl4api::object::RamObj;

use crate::mutex::Mutex;
use crate::utils::align_up;

use vm_allocator::VmAllocator;
use object_allocator::ObjectAllocator;
use vspace_allocator::VspaceAllocator;

pub struct GlobalAllocator {
    pub vm_alloc: Mutex<VmAllocator>,
    pub object_alloc: Mutex<ObjectAllocator>,
    pub vspace_alloc: Mutex<VspaceAllocator>,
}

impl GlobalAllocator {
    pub const fn uninitialized() -> Self {
        Self {
            vm_alloc: Mutex::new(VmAllocator::new()),
            object_alloc: Mutex::new(ObjectAllocator::new()),
            vspace_alloc: Mutex::new(VspaceAllocator::uninitialized()),
        }
    }

    pub fn initialize(&self, brk: usize) {
        self.vspace_alloc.lock().initialize(brk);
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
        use core::ptr::null_mut;
        use rustyl4api::init::InitCSpaceSlot::InitL1PageTable;
        use rustyl4api::vspace::{FRAME_BIT_SIZE, FRAME_SIZE, Permission};

        if layout.size() > FRAME_SIZE {
            return null_mut();
        }

        let mut ptr = self.try_alloc(layout);

        if ptr.is_err() {
            let mempool_size = align_up(layout.size(), 4096);
            let mempool_layout = Layout::from_size_align(mempool_size, 4096).unwrap();
            let ram_cap = self.object_alloc.lock().utspace_alloc::<RamObj>(FRAME_BIT_SIZE).unwrap();
            let addr = self.vspace_alloc.lock().allocate(mempool_layout);
//            let mut brk_lock = self.sys_brk.lock();
//            let brk = brk_lock.deref_mut().as_mut().unwrap();
            ram_cap.map(InitL1PageTable as usize, addr, Permission::new(true, true, false)).unwrap();
            self.add_mempool(addr as *mut u8, FRAME_SIZE);
            ptr = self.try_alloc(layout);
//            *brk += FRAME_SIZE;
        }

        ptr.map(|p| p.as_ptr())
           .unwrap_or(null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.vm_alloc
            .lock()
            .dealloc(NonNull::new_unchecked(ptr), layout)
    }

    pub fn cspace_alloc(&self) -> Option<usize> {
        self.object_alloc
            .lock()
            .cspace_alloc()
    }

    pub fn cspace_free(&self, slot: usize) {
        self.object_alloc
            .lock()
            .cspace_free(slot)
    }

    pub fn vspace_alloc(&self, layout: Layout) -> Option<usize> {
        Some(self.vspace_alloc
            .lock()
            .allocate(layout))
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