#![feature(asm)]
#![feature(decl_macro)]
#![feature(alloc_error_handler)]
#![feature(const_in_array_repeat_expressions)]
#![feature(optin_builtin_traits)]
#![feature(const_fn)]
#![feature(allocator_api)]

#![no_std]

extern crate alloc;
extern crate rustyl4api;

#[macro_use] mod debug_printer;
pub mod syscall;
mod init;
pub mod allocator;
mod mutex;
pub mod capability;
mod utils;

pub use rustyl4api::*;
pub use debug_printer::{print, println};
pub use syscall::*;

extern "C" {
    pub static _end: [u8; 0];
}