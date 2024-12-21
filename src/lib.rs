#![no_std]
#![no_main]

// External Crates
use core::panic::PanicInfo;

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
    panic!("sdkjfhgfsujkfhskufd");
    loop { }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!(Color::Green, Color::Black, "Panik! -> \n{:#?}", _info);
    loop {}
}