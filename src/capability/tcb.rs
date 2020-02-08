use rustyl4api::object::ObjType;
use rustyl4api::error::Result;

use super::{Capability, KernelObject};

pub struct TcbObj {}

impl KernelObject for TcbObj {
    fn obj_type() -> ObjType { ObjType::Tcb }
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