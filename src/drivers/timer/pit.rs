use utils::port::outb;

use crate::utils;

const PIT_COMMAND_PORT: u16 = 0x43;
const PIT_DATA_PORT: u16 = 0x40;
const PIT_FREQ: u32 = 1193182;

pub fn pit_init(freq: u32) {
    let divisor = PIT_FREQ / freq;
    unsafe {
        outb(PIT_COMMAND_PORT, 0x36); // Command: Mode 3, binary, channel 0
        outb(PIT_DATA_PORT, (divisor & 0xFF) as u8); // Low byte
        outb(PIT_DATA_PORT, (divisor >> 8) as u8); // High byte
    }
}
