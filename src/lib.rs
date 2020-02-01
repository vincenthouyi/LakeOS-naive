#![feature(asm)]
#![feature(decl_macro)]
#![feature(alloc_error_handler)]

#![no_std]

extern crate alloc;
extern crate rustyl4api;

#[macro_use] mod debug_printer;
mod syscall;
mod init;
mod allocator;

pub use rustyl4api::*;
pub use debug_printer::{print, println};
pub use syscall::*;