use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use spin::Mutex;

/// Represents a block of memory in our free list
#[repr(C)]
struct Block {
    size: usize,
    next: Option<NonNull<Block>>,
}

/// A basic free-list allocator
pub struct Allocator {
    head: Option<NonNull<Block>>,
    start: usize,
    size: usize,
}

/// Wrapper type for our global allocator
pub struct LockedAllocator(Mutex<Allocator>);

impl LockedAllocator {
    /// Get access to the inner allocator
    pub fn lock(&self) -> spin::MutexGuard<Allocator> {
        self.0.lock()
    }
}

/// Thread-safe global allocator instance
#[global_allocator]
pub static ALLOCATOR: LockedAllocator = LockedAllocator(Mutex::new(Allocator::new()));

unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().deallocate(ptr, layout)
    }
}

impl Allocator {
    /// Creates a new empty allocator
    const fn new() -> Self {
        Self {
            head: None,
            start: 0,
            size: 0,
        }
    }

    /// Initializes the allocator with a memory region
    /// 
    /// # Safety
    /// The caller must ensure that:
    /// - The memory region is valid and unused
    /// - The memory region won't be used by anything else
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        // TODO: Initialize the allocator with the given memory region
        // 1. Store start and size
        // 2. Create initial free block
        // 3. Set up free list

        self.start = start;
        self.size = size;

        
    }

    /// Allocates memory according to the given layout
    /// 
    /// # Safety
    /// This function is unsafe because:
    /// - It may return a null pointer if allocation fails
    /// - The returned memory is uninitialized
    unsafe fn allocate(&mut self, layout: Layout) -> *mut u8 {
        // TODO: Implement allocation logic
        // 1. Find suitable block in free list
        // 2. Split block if necessary
        // 3. Remove from free list
        // 4. Return pointer to usable memory
        core::ptr::null_mut()
    }

    /// Deallocates previously allocated memory
    /// 
    /// # Safety
    /// The caller must ensure that:
    /// - `ptr` was allocated by this allocator
    /// - `layout` matches the layout used for allocation
    unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        // TODO: Implement deallocation logic
        // 1. Convert ptr back to Block
        // 2. Add block to free list
        // 3. Merge with adjacent free blocks if possible
    }

    /// Merges adjacent free blocks to reduce fragmentation
    fn merge(&mut self) {
        // TODO: Implement coalescing logic
        // 1. Walk through free list
        // 2. Merge adjacent blocks
        // 3. Update next pointers
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    panic!("Allocation error: {:?}", _layout)
}
