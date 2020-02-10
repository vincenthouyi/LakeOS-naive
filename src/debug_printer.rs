use core::fmt::{Write, Arguments, Result};

struct DebugPrinter {}

impl Write for DebugPrinter {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            let msg_len = 1;
            let msg_info = crate::MsgInfo::new(crate::SyscallOp::DebugPrint, msg_len);

            unsafe {
                crate::syscall(msg_info, 0, c as usize,0,0,0,0).unwrap();
            }
        }
        Ok(())
    }
}

pub fn _print(args: Arguments) {
    let mut debug_printer = DebugPrinter{};
    debug_printer.write_fmt(args).unwrap();
}

/// Like `println!`, but for kernel-space.
pub macro println {
    () => (print!("\n")),
    ($fmt:expr) => (print!(concat!($fmt, "\n"))),
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*))
}

/// Like `print!`, but for kernel-space.
pub macro print($($arg:tt)*) {
    _print(format_args!($($arg)*))
}
