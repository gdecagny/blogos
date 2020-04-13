#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use blog_os::println;

use blog_os::vga_buffer;

#[cfg(test)]
use blog_os::{serial_print, serial_println, exit_qemu, QemuExitCode}; 


#[cfg(not(test))]
#[panic_handler]
fn panicc(_info: &PanicInfo) -> ! {
    println!("Panic detected! {}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panicc(_info: &PanicInfo) -> ! {
    serial_println!("Panic detected! {}", _info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}



#[no_mangle]
extern "C" fn _start() -> ! {
    
    println!("Ca roacks! ");
    
    vga_buffer::print_something();

    #[cfg(test)]
    test_main();

    println!("Ca a roacked.");
    println!("Ca a totaement rocked {}", 42);

    loop {}
}

#[test_case]
fn trivial_assertion() {
    serial_print!("Test trivial assertion... ");
    assert_eq!(0, 0);
    serial_println!("Ok!"); 
}