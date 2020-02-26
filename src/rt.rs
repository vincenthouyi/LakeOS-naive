use crate::debug_printer::*;
use crate::allocator::INIT_ALLOC;

extern "Rust" {
    fn main();
}

const MEMPOOL_SIZE: usize = 4096;
static mut INIT_ALLOC_MEMPOOL: [u8; MEMPOOL_SIZE] = [0u8; MEMPOOL_SIZE];

#[no_mangle]
pub fn _start() -> ! {
    println!("赞美太阳！");

    let brk = unsafe{ crate::_end.as_ptr() as usize };
    let brk = crate::utils::align_up(brk, rustyl4api::vspace::FRAME_SIZE);

    unsafe {
        INIT_ALLOC.add_mempool(INIT_ALLOC_MEMPOOL.as_ptr() as *mut u8, INIT_ALLOC_MEMPOOL.len());
        INIT_ALLOC.initialize(brk);
    }

    unsafe { main(); }
    unreachable!("Init Returns!");
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic! {:?}", _info);
    loop {
    }
}