
use rustyl4api::ObjType;
use rustyl4api::syscall::*;

use crate::debug_printer::*;

extern "Rust" {
    fn main();
}

const MEMPOOL_SIZE: usize = 4096;
static mut INIT_ALLOC_MEMPOOL: [u8; MEMPOOL_SIZE] = [0u8; MEMPOOL_SIZE];

#[no_mangle]
pub fn _start() {
    use rustyl4api::init::InitCSpaceSlot::*;
    use alloc::boxed::Box;
    use crate::allocator::*;

    println!("赞美太阳！");

    INIT_ALLOC.set_leaky_mempool(unsafe{ INIT_ALLOC_MEMPOOL.as_mut_ptr() }, MEMPOOL_SIZE);
    

    let a = Box::new(100usize);
    println!("a {:?}", a);


    unsafe{ main(); }
    unreachable!("Init Returns!");
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic! {:?}", _info);
    loop {
    }
}