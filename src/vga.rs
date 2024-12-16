// Imports

use core::ptr;

// Constants

const BUF_WIDTH: usize = 80;
const BUF_HEIGHT: usize = 25;
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

/// Clears the screen by writing spaces with a default background color.
pub unsafe fn clear_screen(bg: Color) {
    let space: u16 = (bg as u16) << 12 | b' ' as u16; // Combine space character and background color
    let buffer_size = BUF_WIDTH * BUF_HEIGHT;

    // Fill the VGA buffer with spaces and the specified background color
    let buffer_ptr = VGA_BUFFER as *mut u16; // Cast to u16 pointer for 2 bytes per character
    for i in 0..buffer_size {
        ptr::write(buffer_ptr.add(i), space);
    }
}

/// Writes a character to the VGA buffer at the given position with the specified foreground and background colors.
pub unsafe fn write_char(x: usize, y: usize, c: u8, fg: Color, bg: Color) {
    let index = 2 * (y * BUF_WIDTH + x);
    *VGA_BUFFER.add(index) = c; // Character
    *VGA_BUFFER.add(index + 1) = (bg as u8) << 4 | (fg as u8); // Color attributes
}