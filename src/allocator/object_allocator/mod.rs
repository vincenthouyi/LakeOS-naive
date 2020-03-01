use rustyl4api::init::INIT_CSPACE_SIZE;
use rustyl4api::object::{Capability, KernelObject};

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
            empty_slot_start: 150,
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

    pub fn cspace_free(&mut self, _cptr: usize) {
        // unimplemented!()
    }

    pub fn utspace_alloc<T: KernelObject>(&mut self, size_bits: usize) -> Option<Capability<T>> {
        use rustyl4api::init::InitCSpaceSlot::UntypedStart;
        use rustyl4api::object::UntypedObj;

        let untyped_cap = Capability::<UntypedObj>::new(UntypedStart as usize);
        let slot = self.cspace_alloc()?;
        untyped_cap.retype(T::obj_type(), size_bits, slot, 1).ok()?;
        Some(Capability::<T>::new(slot))
    }

    pub fn utspace_free(&mut self) {
        // unimplemented!()
    }
}
