#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    blog_os::serial_println!("C'est parti!");
    blog_os::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panicccc(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
} 

#[test_case]
fn test_breakpoint_interrupt()
{
    use blog_os::{serial_print, serial_println};
    serial_print!("Testing int3 interrupt... ");
    use x86_64::instructions;
    instructions::interrupts::int3();
    serial_println!("Ok!");
}
