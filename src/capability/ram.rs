use rustyl4api::object::ObjType;
use rustyl4api::error::Result;
use rustyl4api::vspace::Permission;

use super::{Capability, KernelObject};

pub struct RamObj {}

impl KernelObject for RamObj {
    fn obj_type() -> ObjType { ObjType::Ram }
}

impl Capability<RamObj> {
    pub fn map(&self, vspace: usize, vaddr: usize, rights: Permission) -> Result<()> {
        rustyl4api::syscall::ram_map(self.slot, vspace, vaddr, rights.into())
    }

    pub fn unmap(&self) -> Result<()> {
        unimplemented!()
    }
}
