
use core::cell::{RefCell, Cell};
use alloc::sync::Arc;

use rustyl4api::ObjType;
use rustyl4api::syscall::*;
use rustyl4api::init::InitCSpaceSlot::*;

use crate::debug_printer::*;
use crate::object_manager::{OBJECT_MANAGER, ObjectManager, TcbObj};

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
    let tcb = OBJECT_MANAGER.lock()
                            .as_mut()
                            .unwrap()
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
    use alloc::boxed::Box;
    use crate::allocator::*;

    println!("赞美太阳！");

    INIT_ALLOC.add_mempool(unsafe{ INIT_ALLOC_MEMPOOL.as_mut_ptr() }, MEMPOOL_SIZE);
    rustyl4api::syscall::nop(); // make sure INIT_ALLOC is initialized before other code
    *OBJECT_MANAGER.lock() = Some(ObjectManager::new());

    spawn_thread(test_thread);

    let foo = vec![1,2,3,4,5];

    for i in foo {
        println!("i {}", i);
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