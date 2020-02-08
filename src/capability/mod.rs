use core::marker::PhantomData;

use rustyl4api::object::ObjType;

mod tcb;
mod ram;

pub use tcb::TcbObj;
pub use ram::RamObj;

#[derive(Copy, Clone)]
pub struct Capability<T: KernelObject> {
    pub slot: usize,
    pub obj_type: PhantomData<T>,
}

impl<T: KernelObject> Capability<T> {
    pub fn new(slot: usize) -> Self {
        Self {
            slot: slot,
            obj_type: PhantomData,
        }
    }
}

pub trait KernelObject {
    fn obj_type() -> ObjType;
}