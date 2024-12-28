#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

// External Crates
extern crate alloc;

// Kernel modules
pub mod kernel {
    pub mod config;
    pub mod multiboot;
    pub mod memory;

    pub mod interrupts {
        pub mod idt;
        pub mod pic;
    }
}

// Device drivers
pub mod drivers {
    pub mod input {
        pub mod keyboard;
    }

    pub mod video {
        #[macro_use]
        pub mod vga;
    }

    pub mod timer {
        pub mod pit;
    }
}

// Utility modules
pub mod utils {
    pub mod port;
}

use core::arch::asm;

// Imports
use drivers::video::vga::*;
use kernel::memory::bump_allocator::{LockedBumpAllocator, HEAP_START, HEAP_SIZE};

use alloc::boxed::Box;

#[global_allocator]
static ALLOCATOR: LockedBumpAllocator = LockedBumpAllocator::new();

// Kernel entry point
#[no_mangle]
pub extern "C" fn kmain(multiboot2_magic: u32, multiboot2_info_ptr: u32) -> ! {
    unsafe {
        clear_screen(Color::Black);
        kernel::config::DEBUG_OUTPUT = false; // Set to true to enable debug output
    }

    // Multiboot Info Extraction
    if kernel::multiboot::check_magic(multiboot2_magic) {
        kernel::multiboot::parse_info(multiboot2_info_ptr);
    }

    println!(
        Color::Green,
        Color::Black,
        "Multiboot2 Magic:{}",
        multiboot2_magic
    );
    println!(
        Color::Green,
        Color::Black,
        "Multiboot2 Info Ptr:{}",
        multiboot2_info_ptr
    );

    println!(Color::Green, Color::Black, "Print Test              ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    kernel::interrupts::pic::remap_pic();

    println!(Color::Green, Color::Black, "Setup PIC               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    // Initialize the heap allocator
    ALLOCATOR.init(HEAP_START, HEAP_SIZE);
    println!(Color::Green, Color::Black, "Setup Heap Allocator    ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    kernel::interrupts::idt::init_idt();

    println!(Color::Green, Color::Black, "Setup IDT               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    kernel::interrupts::pic::pic_enable_irq(0); // Enable Timer

    println!(Color::Green, Color::Black, "Setup PIT               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    kernel::interrupts::pic::pic_enable_irq(1); // Enable Keyboard

    println!(Color::Green, Color::Black, "Setup Keyboard          ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    unsafe { asm!("sti") } // Enable all interrupts

    println!(Color::Green, Color::Black, "Interrupts Enabled      ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    // Test Heap Allocation with a Box
    let heap_box = Box::new(42);
    println!(Color::Green, Color::Black, "Heap Box Allocation   {}", heap_box);

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!(Color::LightGreen, Color::Black, "Panik! -> \n{:#?}", _info);
    hlt_loop()
}

/// Halt loop for debugging
fn hlt_loop() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
