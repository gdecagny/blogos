#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use blog_os;
use blog_os::gdt;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    blog_os::serial_println!("C'est parti!");
    gdt::load_gdt();
    load_test_idt();
    test_stack_overflow();
    blog_os::exit_qemu(blog_os::QemuExitCode::Success);
    loop {}
}

#[panic_handler]
fn panicccc(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
} 

use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn test_handle_double_fault(_sf: &mut InterruptStackFrame, _ec: u64) -> ! {
    blog_os::serial_println!("Ok!");
    blog_os::exit_qemu(blog_os::QemuExitCode::Success);
    loop {}
}

use lazy_static::lazy_static;
use x86_64::structures::idt;

lazy_static! {
    static ref TEST_IDT: idt::InterruptDescriptorTable = {
        let mut table = idt::InterruptDescriptorTable::new();
        unsafe { 
            table.double_fault.set_handler_fn(test_handle_double_fault)
                .set_stack_index(gdt::DOUBLE_FAULT_TSS_INDEX);
        }
        table
    };
}

fn load_test_idt() {
    TEST_IDT.load();
}


fn test_stack_overflow()
{
    use blog_os::{serial_print, serial_println};
    serial_print!("Testing stack overflow... ");
    
    #[allow(unconditional_recursion)]
    fn stack_overflow(a: i64) {
        stack_overflow(a + 1);
    }
    stack_overflow(0);    
    serial_println!("Ok!");
}
