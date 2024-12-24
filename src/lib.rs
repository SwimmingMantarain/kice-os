#![no_std]
#![no_main]
#![feature(naked_functions)]

// External Crates
use core::{arch::asm, panic::PanicInfo};

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

// Imports
use vga::*;
use multiboot::*;

#[no_mangle]
pub extern "C" fn kmain(multiboot_info_addr: *const u8) -> ! {
    unsafe {
        clear_screen(Color::Black);
    }

    print!(Color::Green, Color::Black, "Print Test              ");
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

    // Multiboot Info Extraction
    let multiboot_info = check(multiboot_info_addr);
    //multiboot::parse_memory_map(multiboot_info);
    
    loop { }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!(Color::LightGreen, Color::Black, "Panik! -> \n{:#?}", _info);
    loop {}
}