#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

core::arch::global_asm!(include_str!("start.asm"));
core::arch::global_asm!(include_str!("multiboot2_header.asm"));

use core::panic::PanicInfo;
use multiboot2::BootInformation;

mod interrupts;
#[macro_use]
mod vga_buffer;
mod mem;

#[no_mangle]
fn entry_rust(multiboot2_magic: u32, multiboot2_info_ptr: u32) -> ! {
    // Initialize the memory allocator
    mem::init_heap();

    let multiboot2_info = unsafe { BootInformation::load(multiboot2_info_ptr as *const multiboot2::BootInformationHeader) };

    println!("multiboot2 magic: {:#x}", multiboot2_magic);

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
    loop {
        hlt_loop();
    }
}