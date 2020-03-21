mod linked_list;

use core::alloc::{GlobalAlloc, Layout, AllocErr};
use core::ptr::{NonNull};
use core::cmp::max;
use crate::utils::prev_power_of_two;

use linked_list::LinkedList;

pub const MEMPOOL_MAX_BITSZ: usize = rustyl4api::vspace::FRAME_BIT_SIZE;
pub const MEMPOOL_MIN_BITSZ: usize = 3;
const MEMPOOL_ARRAY_SZ: usize = MEMPOOL_MAX_BITSZ - MEMPOOL_MIN_BITSZ + 1;
pub struct VmAllocator {
    mempool: [LinkedList; MEMPOOL_ARRAY_SZ],
}

impl VmAllocator {
    pub const fn new() -> Self {
        VmAllocator {
            mempool: [LinkedList::new(); MEMPOOL_ARRAY_SZ],
        }
    }

    pub fn add_mempool(&mut self, base: *mut u8, size: usize) {
        let mut cur_ptr = base as usize;
        let mut rem_sz = size;
        crate::debug_println!("mempool total {:p}-{:p} size {}", base, (base as usize + size) as *mut u8, size);

        while rem_sz > 0 {
            let cur_sz = (cur_ptr & (!cur_ptr + 1))
                .min(prev_power_of_two(rem_sz))
                .min(1 << MEMPOOL_MAX_BITSZ);
            let cur_bitsz = cur_sz.trailing_zeros() as usize;
            crate::debug_println!("adding mempool {:p}-{:p} size {}", cur_ptr as *mut usize, (cur_ptr + cur_sz) as *mut usize, cur_sz);

            if cur_bitsz >= MEMPOOL_MIN_BITSZ {
                unsafe {
                    self.mempool[cur_bitsz - MEMPOOL_MIN_BITSZ]
                        .push(cur_ptr as *mut usize);
                }
            }
            cur_ptr += cur_sz;
            rem_sz -= cur_sz;
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        let bit_sz = chunk_size(layout).trailing_zeros() as usize;

        (bit_sz..=MEMPOOL_MAX_BITSZ)
            .find_map(|sz|
                self.mempool[sz - MEMPOOL_MIN_BITSZ]
                    .pop()
                    .map(|ptr| (sz, ptr as *mut u8))
            )
            .map(|(chunk_sz, ptr)| unsafe {
                crate::debug_println!("getting ptr {:p} size {}", ptr, 1 << chunk_sz);
                for sz in bit_sz..chunk_sz {
                    let back_sz = 1 << sz;
                    let back_ptr = ptr.offset(back_sz);
                    crate::debug_println!("inserting back {:p} size {}", back_ptr, back_sz);
                    self.mempool[sz - MEMPOOL_MIN_BITSZ]
                        .push(back_ptr as *mut usize)
                }

                NonNull::new_unchecked(ptr as *mut u8)
            })
            .ok_or(AllocErr {})
    }

    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let mut bit_sz = chunk_size(layout).trailing_zeros() as usize;
        let mut cur_ptr = ptr.as_ptr() as usize;
//        crate::println!("dealloc ptr {:p} size {}", ptr.as_ptr(), 1 << bit_sz);

        while bit_sz <= MEMPOOL_MAX_BITSZ {
            let buddy = (cur_ptr ^ (1 << bit_sz)) as *mut usize;
            let tmp_ptr = self.mempool[bit_sz - MEMPOOL_MIN_BITSZ]
                              .iter_mut()
                              .find(|node| node.value() == buddy)
                              .map(|node| node.pop() as usize);

            if tmp_ptr.is_none() {
                break;
            } else {
                cur_ptr = cur_ptr & !(1 << bit_sz);
                bit_sz += 1;
            }
        }

        unsafe {
            self.mempool[bit_sz - MEMPOOL_MIN_BITSZ]
                .push(cur_ptr as *mut usize);
        }
    }
}

fn chunk_size(layout: Layout) -> usize {
    use core::mem::size_of;

    const SIZEOF_USIZE: usize = size_of::<usize>();

    max(layout.size().next_power_of_two(),
        max(layout.align(), SIZEOF_USIZE))
}

unsafe impl GlobalAlloc for VmAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc(ptr, layout)
    }
}

#[global_allocator]
pub static mut GLOBAL_VM_ALLOCATOR: VmAllocator = VmAllocator::new(); 

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}