#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]

// External Crates
use core::{arch::asm, panic::PanicInfo};
extern crate alloc;

// Internal Crates
#[macro_use]
pub mod vga;
pub mod keyboard;
pub mod port;
pub mod pic;
pub mod pit;
mod multiboot;
mod mem;
mod idt;
mod config;
mod allocator;

// Imports
use vga::*;

#[no_mangle]
pub extern "C" fn kmain(multiboot2_magic: u32, multiboot2_info_ptr: u32) -> ! {
    unsafe {
        clear_screen(Color::Black);
        config::DEBUG_OUTPUT = false;  // Set to true to enable debug output
    }

    // Multiboot Info Extraction
    if multiboot::check_magic(multiboot2_magic) {
        multiboot::parse_info(multiboot2_info_ptr);
        
        // Initialize allocator with memory from multiboot
        // TODO: Get actual memory region from multiboot info
        unsafe {
            allocator::ALLOCATOR.lock().init(0x_1000_0000, 1024 * 1024); // Example: 1MB at 16MB mark
        }
    }

    println!(Color::Green, Color::Black, "Multiboot2 Magic:{}", multiboot2_magic);
    println!(Color::Green, Color::Black, "Multiboot2 Info Ptr:{}", multiboot2_info_ptr);

    println!(Color::Green, Color::Black, "Print Test              ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    pic::remap_pic();

    println!(Color::Green, Color::Black, "Setup PIC               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    idt::init_idt();

    println!(Color::Green, Color::Black, "Setup IDT               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    pic::pic_enable_irq(0); // Enable Timer

    println!(Color::Green, Color::Black, "Setup PIT               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    pic::pic_enable_irq(1); // Enable Keyboard

    println!(Color::Green, Color::Black, "Setup Keyboard          ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    unsafe { asm!("sti") } // Enable all interrupts

    println!(Color::Green, Color::Black, "Interrupts Enabled      ");
    print!(Color::LightGreen, Color::Black, "[OK]");
    
    loop { 
        unsafe {
            asm!("hlt");
        }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!(Color::LightGreen, Color::Black, "Panik! -> \n{:#?}", _info);
    loop {}
}

/// Halt loop for debugging
fn hlt_loop() {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
