use core::alloc::{GlobalAlloc, Alloc, Layout, AllocErr};
use core::ptr::{null_mut, NonNull};
use core::cmp::max;
use super::linked_list::LinkedList;

const MEMPOOL_MAX_BITSZ: usize = 11;
const MEMPOOL_MIN_BITSZ: usize = 3;
const MEMPOOL_ARRAY_SZ: usize = MEMPOOL_MAX_BITSZ - MEMPOOL_MIN_BITSZ + 1;
pub struct InitAllocator {
    mempool: [LinkedList; MEMPOOL_ARRAY_SZ],
}

impl InitAllocator {
    pub const fn new() -> Self {
        InitAllocator {
            mempool: [LinkedList::new(); MEMPOOL_ARRAY_SZ],
        }
    }

    fn chunk_size(layout: Layout) -> usize {
        use core::mem::size_of;

        const SIZEOF_USIZE: usize = size_of::<usize>();

        max(layout.size().next_power_of_two(),
            max(layout.align(), SIZEOF_USIZE))
    }

    pub fn add_mempool(&mut self, base: *mut u8, size: usize) {
        let mut cur_ptr = base as usize;
        let mempool_top = cur_ptr + size;

        while (cur_ptr < mempool_top) {
            let cur_sz = cur_ptr.trailing_zeros()
                                .min(MEMPOOL_MAX_BITSZ as u32) as usize;

            unsafe {
                self.mempool[cur_sz - MEMPOOL_MIN_BITSZ]
                    .push(cur_ptr as *mut usize);
            }
            cur_ptr += 1 << cur_sz;
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        let bit_sz = Self::chunk_size(layout).trailing_zeros() as usize;

        let ptr = (bit_sz..MEMPOOL_MAX_BITSZ)
            .position(|sz| self.mempool[sz - MEMPOOL_MIN_BITSZ].peek().is_some())
            .map(|idx| bit_sz + idx)
            .map(|sz| (sz, self.mempool[sz - MEMPOOL_MIN_BITSZ]
                               .pop()
                               .unwrap() as usize ))
            .map_or(null_mut(), |(chunk_sz, ptr)| {
                for sz in (bit_sz..chunk_sz) {
                    unsafe {
                        self.mempool[sz - MEMPOOL_MIN_BITSZ]
                            .push((ptr + 1 << sz) as *mut usize)
                    }
                }

                ptr as *mut u8
            });
        NonNull::new(ptr).ok_or(AllocErr {})
    }

    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let mut bit_sz = Self::chunk_size(layout).trailing_zeros() as usize;
        let mut cur_ptr = ptr.as_ptr() as usize;

        while (bit_sz < MEMPOOL_MAX_BITSZ) {
            let buddy = cur_ptr ^ (1 << bit_sz);
            let tmp_ptr = self.mempool[bit_sz]
                              .iter_mut()
                              .find(|node| node.value() == buddy as *mut usize)
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