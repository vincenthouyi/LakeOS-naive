use rustyl4api::object::ObjType;
use rustyl4api::error::Result;
use rustyl4api::syscall::{MsgInfo, SyscallOp, syscall};

use super::{Capability, KernelObject};

pub struct EndpointObj {}

impl KernelObject for EndpointObj {
    fn obj_type() -> ObjType { ObjType::Endpoint }
}

impl Capability<EndpointObj> {
    pub fn send(&self, message: &[usize]) -> Result<()> {
        let info = MsgInfo::new(SyscallOp::EndpointSend, message.len());
        let ret = unsafe { syscall(info, self.slot, message[0], 0, 0, 0, 0) };
        return ret;
    }

    pub fn receive(&self, buf: &mut [usize]) -> Result<usize> {
        let info = MsgInfo::new(SyscallOp::EndpointRecv, 1);
        let ret = unsafe { syscall(info, 0, 0, 0, 0, 0, 0) };
        Ok(0)
    }
}
