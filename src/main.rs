#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(not(test))]
use blog_os::{println, serial_print, serial_println};

use blog_os::init;
use x86_64::instructions::hlt;

#[cfg(test)]
use blog_os::{println, serial_print, serial_println, exit_qemu, QemuExitCode}; 


#[cfg(not(test))]
#[panic_handler]
fn panicc(info: &PanicInfo) -> ! {
    println!("Panic detected! {}", info);
    serial_println!("Panic detected! {}", info);
    loop { hlt(); }
}

#[cfg(test)]
#[panic_handler]
fn panicc(_info: &PanicInfo) -> ! {
    serial_println!("Panic detected! {}", _info);
    exit_qemu(QemuExitCode::Failed);
    loop { hlt(); }
}



#[no_mangle]
extern "C" fn _start() -> ! {
    
    println!("Ca roacks! ");
    
    init();

    use x86_64::instructions;
    instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("Ca a roacked.");
    loop { 
        hlt();
    }
}

#[test_case]
fn trivial_assertion() {
    serial_print!("Test trivial assertion... ");
    assert_eq!(0, 0);
    serial_println!("Ok!"); 
}