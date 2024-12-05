// src/interrupts.rs
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;

use crate::println;

// Define the IDT (Interrupt Descriptor Table)
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Set up basic exception handlers
        idt.divide_error.set_handler_fn(divide_by_zero_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt // Return the initialized IDT
    };
}

// Initialize the IDT
pub fn init_idt() {
    IDT.load();
}

// Example handler for Divide by Zero
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {} // Halt the CPU to prevent further execution
}

// Example handler for Page Fault
extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    println!("EXCEPTION: PAGE FAULT");
    println!("Error Code: {:#x}\n{:#?}", error_code, stack_frame);
    loop {}
}
