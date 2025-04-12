#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

// External Crates
extern crate multiboot2;

// Kernel modules
pub mod kernel {
    pub mod config;

    pub mod interrupts {
        pub mod idt;
        pub mod pic;
    }

}

// Device drivers
pub mod drivers {
    pub mod input {
        pub mod keyboard;
    }

    pub mod video {
        #[macro_use]
        pub mod vga;
    }

    pub mod timer {
        pub mod pit;
    }
}

// Utility modules
pub mod utils {
    pub mod port;
}

use core::{arch::asm, panic};

// Imports
use drivers::video::vga::*;
use multiboot2::{BootInformation, BootInformationHeader};

// Kernel entry point
#[no_mangle]
pub extern "C" fn kmain(multiboot2_magic: u32, multiboot2_info_ptr: u32) -> ! {
    unsafe {
        clear_screen(Color::Black);
        kernel::config::DEBUG_OUTPUT = false; // Enable debug output
    }

    debug_println!(
        Color::Green,
        Color::Black,
        "Multiboot2 Magic:{}",
        multiboot2_magic
    );
    debug_println!(
        Color::Green,
        Color::Black,
        "Multiboot2 Info Ptr:{}",
        multiboot2_info_ptr
    );

    println!(Color::Green, Color::Black, "Print Test              ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    if multiboot2_magic == multiboot2::MAGIC {
        let boot_info = unsafe { BootInformation::load(multiboot2_info_ptr as *const BootInformationHeader).unwrap() };
        let _cmd = boot_info.command_line_tag();
    } else {
        panic!("Multiboot2 magic doesn't magic!");
    }

    kernel::interrupts::pic::remap_pic();

    println!(Color::Green, Color::Black, "Setup PIC               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    kernel::interrupts::idt::init_idt();

    println!(Color::Green, Color::Black, "Setup IDT               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    kernel::interrupts::pic::pic_enable_irq(0); // Enable Timer

    println!(Color::Green, Color::Black, "Setup PIT               ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    kernel::interrupts::pic::pic_enable_irq(1); // Enable Keyboard

    println!(Color::Green, Color::Black, "Setup Keyboard          ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    unsafe { asm!("sti") } // Enable all interrupts

    println!(Color::Green, Color::Black, "Interrupts Enabled      ");
    print!(Color::LightGreen, Color::Black, "[OK]");

    hlt_loop();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!(Color::LightGreen, Color::Black, "Panik! -> \n{:#?}", _info);
    hlt_loop()
}

/// Halt loop for debugging
fn hlt_loop() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
