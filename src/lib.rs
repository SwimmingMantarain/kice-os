#![no_std]
#![no_main]

// External Crates
use core::panic::PanicInfo;

// Internal Crates
#[macro_use]
pub mod vga;
mod mem;

// Imports
use vga::*;

#[no_mangle]
pub extern fn kmain() -> ! {
    unsafe {
        clear_screen(Color::Green);
        println!(Color::Green, Color::Black, "Test!");
    }
    loop { }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { 
       println!(Color::Red, Color::DarkGray, "Panik! -> \n{:#?}", _info);
    }
    loop {}
}