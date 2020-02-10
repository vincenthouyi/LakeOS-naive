use core::marker::PhantomData;

use rustyl4api::init::INIT_CSPACE_SIZE;

use crate::capability::{Capability, KernelObject};

#[derive(Clone)]
pub struct ObjectAllocator {
//    untyped: Vec<Vec<KernelObject>>,
    empty_slot_start: usize,
    cnode_sz: usize,
}

impl ObjectAllocator {
    pub const fn new() -> Self {
        Self {
//            untyped: vec![Vec::new(); 12],
            empty_slot_start: 100,
            cnode_sz: INIT_CSPACE_SIZE,
        }
    }

    pub fn cspace_alloc(&mut self) -> Option<usize> {
        if self.empty_slot_start < self.cnode_sz {
            let ret = self.empty_slot_start;
            self.empty_slot_start += 1;
            Some(ret)
        } else {
            None
        }
    }

    pub fn cspace_free(&mut self, cptr: usize) {
        // unimplemented!()
    }

    pub fn utspace_alloc<T: KernelObject>(&mut self, size_bits: usize) -> Option<Capability<T>> {
        use rustyl4api::init::InitCSpaceSlot::UntypedStart;
        use rustyl4api::syscall::untyped_retype;

        let slot = self.cspace_alloc()?;
        untyped_retype(UntypedStart as usize, T::obj_type(), size_bits, slot, 1).ok()?;
//        Some(Capability::<T>::new(slot))
        Some(Capability::<T> {slot: slot, obj_type: PhantomData})
    }

    pub fn utspace_free(&mut self) {
        // unimplemented!()
    }
}
