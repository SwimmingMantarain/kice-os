#![no_std]
#![no_main]

// External Crates
use core::panic::PanicInfo;

// Internal Crates
pub mod vga;
mod mem;

// Imports
use vga::{clear_screen, Color};

#[no_mangle]
pub extern fn kmain() -> ! {
    unsafe {
        let vgaa = 0xb8000 as *mut u64;

        *vgaa = 0x2f592f412f4b2f4f;
        clear_screen(Color::Green);
    }
    loop { }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}