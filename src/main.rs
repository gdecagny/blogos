#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panicc(_info: &PanicInfo) -> ! {
    println!("Panic detected! {}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panicc(_info: &PanicInfo) -> ! {
    console_log!("Panic detected! {}", _info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
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
    console_log!("Ca roaaackksss {}", 422);
    console_log!("Ca roaaackksss {}", 422);
    console_log!("Ca roaaackksss {}", 422);
    console_log!("Ca roaaackksss {}", 422);
    console_log!("Ca roaaackksss {}", 422);
    console_log!("Ca roaaackksss {}", 422);
    assert_eq!(0, 0);
    println!("Ol goud!");
}