#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

/// Kernel main entry point
///
/// This function is called from assembly after setting up long mode
///
/// # Parameters
/// - `magic`: The multiboot magic number
/// - `info_ptr`: Pointer to the multiboot information structure
#[no_mangle]
pub extern "C" fn kmain(magic: u32, info_ptr: usize) -> ! {
    // You can use the magic and info_ptr parameters here
    // to verify and parse the multiboot information

    // For now, just loop forever
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
