#![no_std]
#![no_main]
#![feature(naked_functions)]

// External Crates
use core::{arch::asm, panic::PanicInfo};

// Internal Crates
#[macro_use]
pub mod vga;
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

    idt::init_idt();

    unsafe {
        asm!("int3");
    }
    
    loop { }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!(Color::Green, Color::Black, "Panik! -> \n{:#?}", _info);
    loop {}
}