use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::{debug_print, print, Color};
use spin::Mutex;

// Constants for heap configuration
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub struct BumpAllocator {
    heap_start: AtomicUsize,
    heap_end: AtomicUsize,
    next: AtomicUsize,
    locked: AtomicBool,
}

impl BumpAllocator {
    /// Creates a new uninitialized bump allocator
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: AtomicUsize::new(0),
            heap_end: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
            locked: AtomicBool::new(false),
        }
    }

    /// Initializes the bump allocator with the given heap bounds
    /// 
    /// # Safety
    /// This function must be called only once, and the heap bounds must be valid
    pub fn init(&self, heap_start: usize, size: usize) {
        self.heap_start.store(heap_start, Ordering::SeqCst);
        let heap_end = heap_start.checked_add(size).expect("Heap bounds overflow");
        self.heap_end.store(heap_end, Ordering::SeqCst);
        self.next.store(heap_start, Ordering::SeqCst);
        
        debug_print!(Color::Green, Color::Black, "Initialized Bump Allocator\n");
        debug_print!(Color::Green, Color::Black, "Heap Start: 0x{:x}\n", heap_start);
        debug_print!(Color::Green, Color::Black, "Heap Size: {} bytes\n", size);
    }

    /// Aligns the given address upward to the specified alignment
    fn align_up(addr: usize, align: usize) -> usize {
        let remainder = addr % align;
        if remainder == 0 {
            addr
        } else {
            addr - remainder + align
        }
    }

    /// Allocates memory from the heap
    pub fn alloc(&self, layout: Layout) -> *mut u8 {
        // Try to lock the allocator
        while self.locked.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed,
        ).is_err() {
            core::hint::spin_loop();
        }

        let alloc_start = Self::align_up(
            self.next.load(Ordering::Relaxed),
            layout.align(),
        );
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => {
                self.locked.store(false, Ordering::Release);
                return null_mut();
            }
        };

        if alloc_end <= self.heap_end.load(Ordering::Relaxed) {
            self.next.store(alloc_end, Ordering::Relaxed);
            self.locked.store(false, Ordering::Release);
            alloc_start as *mut u8
        } else {
            self.locked.store(false, Ordering::Release);
            print!(Color::Red, Color::Black, "Out of memory!\n");
            null_mut()
        }
    }

    /// Deallocates memory from the heap
    pub fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // This allocator never deallocates memory
    }
}

pub struct LockedBumpAllocator(Mutex<BumpAllocator>);

impl LockedBumpAllocator {
    pub const fn new() -> Self {
        LockedBumpAllocator(Mutex::new(BumpAllocator::new()))
    }

    /// Initializes the bump allocator with the given heap bounds
    /// 
    /// # Safety
    /// This function must be called only once, and the heap bounds must be valid
    pub fn init(&self, heap_start: usize, size: usize) {
        self.0.lock().init(heap_start, size);
    }
}

unsafe impl GlobalAlloc for LockedBumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(ptr, layout);
    }
}

#[alloc_error_handler]
pub fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}
