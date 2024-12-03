#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("start.asm"));
core::arch::global_asm!(include_str!("multiboot2_header.asm"));

use core::panic::PanicInfo;

#[no_mangle]
fn entry_rust() {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}