#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panicc(_info: &PanicInfo) -> ! {
    println!("Panic detected! {}", _info);
    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
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
    println!("Test begin");
    assert_eq!(1, 0);
    println!("Ol goud!");
}