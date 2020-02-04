use alloc::vec::{Vec};
use alloc::boxed::Box;
use core::marker::PhantomData;

use crate::mutex::Mutex;
use rustyl4api::ObjType;
use rustyl4api::init::INIT_CSPACE_SIZE;
use rustyl4api::error::Result;

//#[derive(Copy, Clone)]
//struct KernelObject {
////    obj_type: ObjType,
//}

pub trait KernelObject {
    fn obj_type() -> ObjType;
}

pub struct TcbObj {}

impl KernelObject for TcbObj {
    fn obj_type() -> ObjType { ObjType::Tcb }
}

#[derive(Copy, Clone)]
pub struct Capability<T: KernelObject> {
    slot: usize,
    obj_type: PhantomData<T>,
}

impl Capability<TcbObj> {
    pub fn configure(&self, vspace_cap: usize, cspace_cap: usize) -> Result<()> {
        rustyl4api::syscall::tcb_configure(self.slot, vspace_cap, cspace_cap)
    }

    pub fn set_registers(&self, flags: usize, elr: usize, sp: usize) -> Result<()> {
        rustyl4api::syscall::tcb_set_registers(self.slot, flags, elr, sp)
    }

    pub fn resume(&self) -> Result<()> {
        rustyl4api::syscall::tcb_resume(self.slot)
    }
}

#[derive(Clone)]
pub struct ObjectManager {
//    untyped: Vec<Vec<KernelObject>>,
    empty_slot_start: usize,
    cnode_sz: usize,
}

impl ObjectManager {
    pub fn new() -> Self {
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
        untyped_retype(UntypedStart as usize, T::obj_type(), 12, slot, 1).ok()?;
        Some(Capability::<T> {slot: slot, obj_type: PhantomData})
//        unimplemented!()
    }

    pub fn utspace_free(&mut self) {
        // unimplemented!()
    }
}

pub static OBJECT_MANAGER: Mutex<Option<ObjectManager>> = Mutex::new(None);