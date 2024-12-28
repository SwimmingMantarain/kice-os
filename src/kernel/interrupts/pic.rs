use crate::utils::port::*;

/// I/O ports for the PIC
const MASTER_PIC_CMD: u16 = 0x20;
const MASTER_PIC_DATA: u16 = 0x21;
const SLAVE_PIC_CMD: u16 = 0xA0;
const SLAVE_PIC_DATA: u16 = 0xA1;

const PIC_EOI: u8 = 0x20; // End-of-Interrupt command

/// Remap the PIC to avoid conflicts with CPU exceptions
pub fn remap_pic() {
    unsafe {
        // Start initialization
        outb(MASTER_PIC_CMD, 0x11); // ICW1: Start initialization sequence
        outb(SLAVE_PIC_CMD, 0x11);

        // Set offsets
        outb(MASTER_PIC_DATA, 0x20); // ICW2: Master PIC vector offset (32)
        outb(SLAVE_PIC_DATA, 0x28); // ICW2: Slave PIC vector offset (40)

        // Configure cascading
        outb(MASTER_PIC_DATA, 0x04); // ICW3: Master PIC has a slave at IRQ2
        outb(SLAVE_PIC_DATA, 0x02); // ICW3: Slave PIC identity

        // Set mode
        outb(MASTER_PIC_DATA, 0x01); // ICW4: 8086/88 mode
        outb(SLAVE_PIC_DATA, 0x01);

        // Unmask all IRQs (optional; better to mask unused ones)
        outb(MASTER_PIC_DATA, 0x00);
        outb(SLAVE_PIC_DATA, 0x00);
    }
}

/// Sends an EOI to the PIC.
pub unsafe fn pic_send_eoi(irq: u8) {
    if irq >= 8 {
        outb(0xA0, 0x20); // Acknowledge slave PIC
    }
    outb(0x20, 0x20); // Acknowledge master PIC
}

/// Enable certain IRQ number
pub fn pic_enable_irq(irq_num: u8) {
    let mut mask: u8 = unsafe { inb(0x21) };
    mask &= !(1 << irq_num);
    unsafe { outb(0x21, mask) }
}
