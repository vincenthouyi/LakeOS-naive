use core::alloc::Layout;
use alloc::collections::linked_list::LinkedList;
use crate::utils::align_up;

#[derive(Debug)]
struct MemRange {
    start: usize,
    size: usize,
}

#[derive(Debug)]
pub struct VspaceAllocator {
    base_brk: usize,
    brk: usize,
    memlist: LinkedList<MemRange>,
}

impl VspaceAllocator {
    pub const fn uninitialized() -> Self {
        Self {
            base_brk: 0,
            brk: 0,
            memlist: LinkedList::new(),
        }
    }

    pub fn initialize(&mut self, brk: usize) {
        self.base_brk = align_up(brk, 4096);
        self.brk = self.base_brk;
    }

    pub fn allocate(&mut self, layout: Layout) -> usize {
        let start = align_up(self.brk, layout.align());
        let size = layout.size();
        self.brk = start + size;
        self.memlist.push_back(MemRange{start: start, size: size});
        start
    }
}