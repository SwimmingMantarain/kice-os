use core::alloc::Layout;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

extern "C" {
    static _heap_start: u8;
    static _heap_end: u8;
}

pub const HEAP_START: *mut u8 = unsafe { &_heap_start as *const u8 as *mut u8 };
pub const HEAP_END: *mut u8 = unsafe { &_heap_end as *const u8 as *mut u8 };


pub fn init_heap() {
    // Calculate the size of the heap
    let heap_size =  HEAP_END as usize - HEAP_START as usize;

    // Initialize the allocator
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, heap_size);
    }
}