#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec};


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

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_start);

fn kernel_start(boot_info: &'static BootInfo) -> ! {
      
    init();

    use x86_64::instructions;
    instructions::interrupts::int3();

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();

    let physical_memory_offset = boot_info.physical_memory_offset;
    serial_println!("physical memory offset : 0x{:x}", physical_memory_offset);

    let startaddr = level_4_page_table.start_address().as_u64();

    let level_4_page_table = (startaddr + physical_memory_offset) as *const [u64; 12];

    serial_println!("lv4 ptr 0x{:x} : {:#?}", level_4_page_table as u64, unsafe { *level_4_page_table });
    use blog_os::memory;


    
    use x86_64::VirtAddr;
    use x86_64::structures::paging::{MapperAllSizes, Page};

    let mut mapper = unsafe { memory::init_mapper(VirtAddr::new(physical_memory_offset)) };
    
    
    let addr_to_test = [ 0x1000, 0x201232, physical_memory_offset, physical_memory_offset+0x34252, 0xb8000, 0xdeadbeef ];
    for &addr in addr_to_test.iter() {
        println!("Translated virt addr {:x} to {:?}", addr, mapper.translate_addr(VirtAddr::new(addr)));
    }

    let memory_map = &boot_info.memory_map;

    serial_println!("{:#?}", memory_map);

    let mut frame_allocator = memory::BootInfoFrameAllocator::init(memory_map);

    use blog_os::allocator;
    allocator::init_heap(&mut mapper, &mut frame_allocator, 256 * 1024).expect("Failed to init heap");
    
    unsafe { memory::describe_page_table(0, startaddr, 4, physical_memory_offset); }

    let mut vec = Vec::new();
    for i in 0..10 {
        vec.push(i);
    }
    println!("vec at {:p} : {:?}", vec.as_slice(), vec);
    
    #[cfg(test)]
    test_main();

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