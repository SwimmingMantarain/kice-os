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
mod mem;
mod idt;

// Imports
use vga::*;

#[no_mangle]
pub extern fn kmain() -> ! {
    unsafe {
        clear_screen(Color::Black);
    }
    println!(Color::LightGreen, Color::Black, "Test!");

    pic::remap_pic();

    idt::init_idt();

    pic::pic_enable_irq(0); // Enable Timer
    pic::pic_enable_irq(1); // Enable Keyboard

    unsafe { asm!("sti") }
    
    loop { }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!(Color::Green, Color::Black, "Panik! -> \n{:#?}", _info);
    loop {}
}