#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec};

use blog_os::{QemuExitCode, exit_qemu};
use blog_os::init;
use blog_os::serial_print;
use x86_64::instructions::hlt;

#[panic_handler]
fn panicc(_info: &PanicInfo) -> ! {
    serial_print!("Panic detected! {}\n", _info);
    exit_qemu(QemuExitCode::Failed);
    loop { hlt(); }
}

use bootloader::{BootInfo, entry_point};

entry_point!(main);

const HEAP_SIZE: usize = 256 * 1024;

fn main(boot_info: &'static BootInfo) -> ! {
    init();

    use blog_os::memory;
    use x86_64::VirtAddr;
    let mut mapper = unsafe { memory::init_mapper(VirtAddr::new(boot_info.physical_memory_offset)) };
    let mut frame_allocator = memory::BootInfoFrameAllocator::init(&boot_info.memory_map);
    use blog_os::allocator;
    allocator::init_heap(&mut mapper, &mut frame_allocator, HEAP_SIZE).expect("Failed to init heap");

    test_main();

    exit_qemu(QemuExitCode::Success);

    loop { hlt(); }

}

#[test_case]
fn test_big_vec() {
    serial_print!("test big vector ...");
    let n = 1000;
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i);
    }
    assert_eq!(v.iter().sum::<u32>(), n * ( n - 1 ) >> 1 );
    serial_print!("[ok]\n");
}


#[test_case]
fn test_lots_of_boxes() {
    serial_print!("test lots of boxes ...");
    let n = 100;
    for i in 0..n {
        let boxx = Box::new([0 as u32; (HEAP_SIZE / 4) - 200]);
        // each box will consume 200 kB of heap, total heap size is 256 kB
    }
    serial_print!("[ok]\n");
}