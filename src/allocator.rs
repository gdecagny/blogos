

use alloc::alloc::Layout;


#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    use crate::serial_println;

    serial_println!("Alloc failed miserably ! layout {:?}", layout);

    panic!("Failed to allocate");
}

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();


use x86_64::VirtAddr;
use x86_64::structures::paging::Page;
use x86_64::structures::paging::{mapper::MapToError, Mapper, FrameAllocator, Size4KiB, PhysFrame, UnusedPhysFrame, PageTableFlags};

const HEAP_START_VIRT: u64 = 0x4444_0000_0000;

pub fn init_heap(mapper: &mut impl Mapper<Size4KiB>, 
                 frame_allocator: &mut impl FrameAllocator<Size4KiB>,
                 heap_size: usize) -> Result<(), MapToError<Size4KiB>> {

    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START_VIRT);
        let heap_end = heap_start + heap_size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT;
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }

    unsafe { ALLOCATOR.lock().init(HEAP_START_VIRT as usize, heap_size); }

    Ok(())

}