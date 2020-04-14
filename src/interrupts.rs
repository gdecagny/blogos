
use x86_64::structures::idt;
use lazy_static::lazy_static;
use crate::gdt;

use crate::{println, serial_println};

extern "x86-interrupt" fn handle_breakpoint(stack_frame: &mut idt::InterruptStackFrame) {
    serial_println!("breakpoint!\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn handle_page_fault(stack_frame: &mut idt::InterruptStackFrame, error_code: idt::PageFaultErrorCode) {
    serial_println!("page_fault!\nError Code : {:?}\n{:?}", error_code, stack_frame);
}


extern "x86-interrupt" fn handle_general_protection_fault(stack_frame: &mut idt::InterruptStackFrame, error_code: u64) {
    println!("general_protection_fault!\nError Code : {}\n{:#?}", error_code, stack_frame);
    serial_println!("general_protection_fault!\nError Code : {}\n{:#?}", error_code, stack_frame);
}


extern "x86-interrupt" fn handle_divide_error(stack_frame: &mut idt::InterruptStackFrame) {
    serial_println!("divide error!\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn handle_double_fault(stack_frame: &mut idt::InterruptStackFrame, error_code: u64) -> ! {
    
    serial_println!("Double fault!!\nError Code : {}\nStack frame:\n{:#?}", error_code, stack_frame);
    loop {}
}


lazy_static! {
    static ref IDT: idt::InterruptDescriptorTable = {
        let mut table = idt::InterruptDescriptorTable::new();
        table.breakpoint.set_handler_fn(handle_breakpoint);
        table.divide_error.set_handler_fn(handle_divide_error);
        table.page_fault.set_handler_fn(handle_page_fault);
        table.general_protection_fault.set_handler_fn(handle_general_protection_fault);
        unsafe { 
            table.double_fault.set_handler_fn(handle_double_fault)
                .set_stack_index(gdt::DOUBLE_FAULT_TSS_INDEX);
        }
        table
    };
}


pub fn register_idt() {
    IDT.load();
}



#[test_case]
fn test_breakpoint_interrupt()
{
    use crate::{serial_print, serial_println};
    serial_print!("Testing int3 interrupt... ");
    use x86_64::instructions;
    instructions::interrupts::int3();
    serial_println!("Ok!");
}