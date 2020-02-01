
use rustyl4api::ObjType;
use rustyl4api::syscall::*;

use crate::debug_printer::*;

fn alloc_obj(obj_type: ObjType, slot: usize) -> rustyl4api::error::Result<()>
{
    use rustyl4api::init::InitCSpaceSlot::UntypedStart;
    untyped_retype(UntypedStart as usize, obj_type, 12, slot, 1)
}

fn test_thread() -> ! {

    for i in 1..=1 {
        for _ in 0..10000000 {nop()}
        println!("爸爸再爱我{}次", i);
    }
    loop {}
}

extern "Rust" {
    fn main();
}

const MEMPOOL_SIZE: usize = 4096;
static mut mempool: [u8; MEMPOOL_SIZE] = [0u8; MEMPOOL_SIZE];

#[no_mangle]
pub fn _start() {
    use rustyl4api::init::InitCSpaceSlot::*;
    use alloc::boxed::Box;
    use crate::allocator::*;

    println!("赞美太阳！");

    unsafe {
        InitAlloc.set_leaky_mempool(mempool.as_mut_ptr(), MEMPOOL_SIZE);
    }

    unsafe{ main(); }
    unreachable!("Init Returns!");
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic! {:?}", _info);
    loop {
    }
}