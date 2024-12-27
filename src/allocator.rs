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
        self.start = start;
        self.size = size;

        // Create initial free block at the start of our memory region
        let initial_block = start as *mut Block;
        (*initial_block).size = size - core::mem::size_of::<Block>();
        (*initial_block).next = None;

        // Add it to our free list
        self.head = Some(NonNull::new_unchecked(initial_block));
    }

    /// Allocates memory according to the given layout
    /// 
    /// # Safety
    /// This function is unsafe because:
    /// - It may return a null pointer if allocation fails
    /// - The returned memory is uninitialized
    unsafe fn allocate(&mut self, layout: Layout) -> *mut u8 {
        // Calculate required size with alignment and header
        let size = layout.size();
        let align = layout.align();
        let header_size = core::mem::size_of::<Block>();
        
        // Total allocation size needs to include header and maintain alignment
        let total_size = size + header_size;
        let _aligned_size = (total_size + align - 1) & !(align - 1);

        let mut current: Option<NonNull<Block>> = self.head;
        let mut prev: Option<NonNull<Block>> = None;

        while let Some(block) = current {
            let block_ptr = block.as_ptr();
            let block_addr = block_ptr as usize;
            
            // Calculate aligned address for user data
            let data_addr = (block_addr + header_size + align - 1) & !(align - 1);
            let padding = data_addr - (block_addr + header_size);
            let needed_size = padding + size;

            if (*block_ptr).size >= needed_size {
                // Remove block from free list
                match prev {
                    Some(p) => (*p.as_ptr()).next = (*block_ptr).next,
                    None => self.head = (*block_ptr).next,
                }

                let remaining_size = (*block_ptr).size - needed_size;
                
                // If we have enough space left, create a new free block
                if remaining_size >= header_size {
                    let new_block = (block_addr + header_size + needed_size) as *mut Block;
                    (*new_block).size = remaining_size - header_size;
                    (*new_block).next = (*block_ptr).next;
                    
                    // Add new block to free list
                    let new_block = NonNull::new_unchecked(new_block);
                    match prev {
                        Some(p) => (*p.as_ptr()).next = Some(new_block),
                        None => self.head = Some(new_block),
                    }
                }

                // Set up the allocated block
                (*block_ptr).size = needed_size;
                (*block_ptr).next = None;

                // Return aligned address for user data
                return (block_addr + header_size + padding) as *mut u8;
            }

            prev = current;
            current = (*block_ptr).next;
        }

        core::ptr::null_mut()
    }

    /// Deallocates previously allocated memory
    /// 
    /// # Safety
    /// The caller must ensure that:
    /// - `ptr` was allocated by this allocator
    /// - `layout` matches the layout used for allocation
    unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        // Calculate header size and total allocation size
        let _size = layout.size();
        let header_size = core::mem::size_of::<Block>();
        
        // Calculate the block pointer by moving back to the header
        let mut block_ptr = (ptr as usize - header_size) as *mut Block;
        
        // Sanity check: ensure ptr is not null
        if block_ptr.is_null() {
            return;
        }
        
        // Restore block metadata
        let mut current_block = Block {
            size: (*block_ptr).size,
            next: None,
        };
        
        // Find the correct insertion point in the free list
        let mut prev: Option<NonNull<Block>> = None;
        let mut current = self.head;
        
        while let Some(block) = current {
            // If current block is after our new free block, insert before it
            if (block.as_ptr() as usize) > (block_ptr as usize) {
                break;
            }
            
            prev = current;
            current = unsafe { (*block.as_ptr()).next };
        }
        
        // Connect the block to the free list
        current_block.next = current;
        let block_non_null = NonNull::new_unchecked(block_ptr);
        
        match prev {
            Some(p) => unsafe { (*p.as_ptr()).next = Some(block_non_null) },
            None => self.head = Some(block_non_null),
        }
        
        // Attempt to merge with previous block if possible
        if let Some(p) = prev {
            let prev_block_end = p.as_ptr() as usize + unsafe { (*p.as_ptr()).size };
            if prev_block_end == block_ptr as usize {
                // Merge blocks
                unsafe {
                    (*p.as_ptr()).size += header_size + current_block.size;
                    current_block.size = (*p.as_ptr()).size;
                    block_ptr = p.as_ptr(); // Update block_ptr to merged block
                }
            }
        }
        
        // Attempt to merge with next block if possible
        if let Some(next_block) = current {
            let current_block_end = block_ptr as usize + header_size + current_block.size;
            if current_block_end == next_block.as_ptr() as usize {
                // Merge blocks
                unsafe {
                    current_block.size += header_size + (*next_block.as_ptr()).size;
                    current_block.next = (*next_block.as_ptr()).next;
                }
            }
        }
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    panic!("Allocation error: {:?}", _layout)
}