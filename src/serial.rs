use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

use core::fmt;

lazy_static! {
    static ref SERIAL1: Mutex<SerialPort> = {
        let mut sp = unsafe { SerialPort::new(0x03F8 as u16) };
        sp.init();
        Mutex::new(sp)
    };
}


#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial port failed");
    });
}