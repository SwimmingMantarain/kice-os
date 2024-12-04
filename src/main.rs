#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("start.asm"));
core::arch::global_asm!(include_str!("multiboot2_header.asm"));

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[no_mangle]
fn entry_rust() {
    println!("Hello, world!");

    loop {
        hlt_loop();
    }
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {
        hlt_loop();
    }
}