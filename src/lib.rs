#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn kmain() -> ! {
    unsafe {
        let vga = 0xb8000 as *mut u64;

        *vga = 0x2f592f412f4b2f4f;
    }
    loop { }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}