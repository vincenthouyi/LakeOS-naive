#![feature(asm)]
#![feature(decl_macro)]

#![no_std]

extern crate rustyl4api;

mod debug_printer;
mod syscall;

pub use rustyl4api::*;
pub use debug_printer::{print, println};
pub use syscall::*;