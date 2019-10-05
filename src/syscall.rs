use crate::{MsgInfo, SyscallOp, ObjType};

pub unsafe fn syscall(msg_info: MsgInfo, arg1: usize, arg2: usize, arg3: usize, 
                      arg4: usize, arg5: usize, arg6: usize) {
    
    asm! {"
        mov x0, $0
        mov x1, $1
        mov x2, $2
        mov x3, $3
        mov x4, $4
        mov x5, $5
        mov x6, $6

        svc 1
        "
        :
        : "r"(arg1), "r"(arg2), "r"(arg3),
          "r"(arg4), "r"(arg5), "r"(arg6), "r"(msg_info.0)
        : "x0", "x1", "x2", "x3", "x4", "x5", "x6"
        : "volatile"
    }
}

pub fn untyped_retype(untyped: usize, objtype: ObjType, bit_size: usize, slot_start: usize, slot_len: usize)
{
    let info = MsgInfo::new(SyscallOp::Retype, 4);
    unsafe { syscall(info, untyped, objtype as usize, bit_size, slot_start, slot_len, 0); }
}

pub fn nop() {
    unsafe { asm!{"nop"} }
}