#![no_std]
#![no_main]

// External Crates
use core::panic::PanicInfo;

// Internal Crates
pub mod vga;
mod mem;

// Imports
use vga::*;

#[no_mangle]
pub extern fn kmain() -> ! {
    unsafe {
        clear_screen(Color::Green);
        write_str("Test\nTesting\nTested", Color::Black, Color::Green);

    }
    loop { }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}