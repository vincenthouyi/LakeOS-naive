use rustyl4api::init::InitCSpaceSlot::*;

use crate::debug_printer::*;
use crate::capability::{TcbObj};
use crate::allocator::INIT_ALLOC;

extern "Rust" {
    fn main();
}

fn test_thread() -> ! {
    for i in 1..=1 {
        for _ in 0..10000000 {rustyl4api::syscall::nop()}
        println!("妈妈再爱我{}次", i);
    }
    loop {}
}

fn spawn_thread(entry: fn() -> !) {
    use core::ops::DerefMut;
    let tcb = INIT_ALLOC.object_alloc
                        .lock()
                        .deref_mut()
                        .utspace_alloc::<TcbObj>(12)
                        .unwrap();

    tcb.configure(InitL1PageTable as usize, InitCSpace as usize)
       .expect("Error Configuring TCB");
    tcb.set_registers(0b1100,entry as usize, 0x600000 - 0x500)
       .expect("Error Setting Registers");
    tcb.resume()
       .expect("Error Resuming TCB");
}

const MEMPOOL_SIZE: usize = 4096;
static mut INIT_ALLOC_MEMPOOL: [u8; MEMPOOL_SIZE] = [0u8; MEMPOOL_SIZE];

#[no_mangle]
pub fn _start() -> ! {
    use alloc::vec::Vec;

    println!("赞美太阳！");

    let brk = unsafe{ crate::_end.as_ptr() as usize };
    let brk = crate::utils::align_up(brk, rustyl4api::vspace::FRAME_SIZE);

    INIT_ALLOC.initialize(brk);
    INIT_ALLOC.add_mempool(unsafe{ INIT_ALLOC_MEMPOOL.as_mut_ptr() }, MEMPOOL_SIZE);

    spawn_thread(test_thread);

    let mut foo: Vec<u32> = Vec::new();
    for i in 0..1024 {
        foo.push(i);
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