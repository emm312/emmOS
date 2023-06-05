use core::{alloc::GlobalAlloc, ptr::null_mut};

use spin::Mutex;
use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator, mapper::MapToError, Page, PageTableFlags}, VirtAddr};

#[global_allocator]
static ALLOCATOR: good_memory_allocator::SpinLockedAllocator = good_memory_allocator::SpinLockedAllocator::empty();

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_page_start = Page::containing_address(heap_start);
        let heap_page_end = Page::containing_address(heap_end);
        Page::range_inclusive(heap_page_start, heap_page_end)
    };
    for page in page_range {
        let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        };
    }
    unsafe {
        ALLOCATOR.init(HEAP_START, HEAP_SIZE);
    }
    Ok(())
}
