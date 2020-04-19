
use x86_64::structures::idt;
use lazy_static::lazy_static;
use crate::gdt;

use crate::{println, serial_println};

extern "x86-interrupt" fn handle_breakpoint(stack_frame: &mut idt::InterruptStackFrame) {
    serial_println!("breakpoint!\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn handle_page_fault(stack_frame: &mut idt::InterruptStackFrame, error_code: idt::PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read();
    serial_println!("page_fault!\nFault address: {:?}\nError Code : {:?}\n{:?}", addr, error_code, stack_frame);
    use x86_64::instructions::hlt;
    loop { hlt(); }
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


use pic8259_simple::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init_pic_interrupts() {
    unsafe { 
        PICS.lock().initialize(); 
    }
    x86_64::instructions::interrupts::enable();
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self as u8)
    }
}

//use crate::serial_print;

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut idt::InterruptStackFrame) {
    //serial_print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8()); 
    }
}


extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut idt::InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    use spin::Mutex;
    use crate::print;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    let mut port = Port::new(0x60);
    
    
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();
    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(chr) => print!("{}", chr),
                DecodedKey::RawKey(key) => print!("{:?}", key)
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8()); 
    }
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
        table[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        table[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
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