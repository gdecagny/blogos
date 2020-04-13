#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panicccc(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
} 


#[test_case]
fn test_integration_tests() {
    blog_os::serial_print!("testing integration test...");
    assert_eq!(0, 0);
    blog_os::serial_println!("[ok]");
}