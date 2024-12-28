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

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;
static mut CURRENT_FG: Color = Color::White;
static mut CURRENT_BG: Color = Color::Black;

/// Clears the screen by writing spaces with a default background color.
pub unsafe fn clear_screen(bg: Color) {
    let space: u16 = (bg as u16) << 12 | b' ' as u16; // Combine space character and background color
    let buffer_size = BUF_WIDTH * BUF_HEIGHT;

    // Fill the VGA buffer with spaces and the specified background color
    let buffer_ptr = VGA_BUFFER as *mut u16; // Cast to u16 pointer for 2 bytes per character
    for i in 0..buffer_size {
        ptr::write(buffer_ptr.add(i), space);
    }

    CURSOR_X = 0;
    CURSOR_Y = 0;
    CURRENT_BG = bg;
}

/// Scrolls the screen up by one line.
unsafe fn scroll_screen(bg: Color) {
    let buffer_ptr = VGA_BUFFER as *mut u16;
    let blank: u16 = (bg as u16) << 12 | b' ' as u16; // Blank character with background color

    // Move each line up by one
    for y in 1..BUF_HEIGHT {
        for x in 0..BUF_WIDTH {
            let src_index = y * BUF_WIDTH + x;
            let dest_index = (y - 1) * BUF_WIDTH + x;
            ptr::write(
                buffer_ptr.add(dest_index),
                ptr::read(buffer_ptr.add(src_index)),
            );
        }
    }

    // Clear the last line
    for x in 0..BUF_WIDTH {
        ptr::write(buffer_ptr.add((BUF_HEIGHT - 1) * BUF_WIDTH + x), blank);
    }

    CURSOR_Y -= 1;
}

/// Writes a character to the VGA buffer at the current cursor position.
pub unsafe fn write_char(c: u8, fg: Color, bg: Color) {
    if c == b'\n' {
        // Handle newline
        CURSOR_X = 0;
        CURSOR_Y += 1;
        if CURSOR_Y >= BUF_HEIGHT {
            scroll_screen(bg);
        }
        return;
    }

    let index = 2 * (CURSOR_Y * BUF_WIDTH + CURSOR_X);
    *VGA_BUFFER.add(index) = c; // Character
    *VGA_BUFFER.add(index + 1) = (bg as u8) << 4 | (fg as u8); // Color attributes

    CURSOR_X += 1;
    if CURSOR_X >= BUF_WIDTH {
        CURSOR_X = 0;
        CURSOR_Y += 1;
    }

    if CURSOR_Y >= BUF_HEIGHT {
        scroll_screen(bg);
    }
}

/// Writes a string starting at the current cursor position.
pub unsafe fn write_str(string: &str, fg: Color, bg: Color) {
    for c in string.chars() {
        write_char(c as u8, fg, bg);
    }
}

#[macro_export]
macro_rules! print {
    ($fg:expr, $bg:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = crate::VgaWriter::new($fg, $bg);
        write!(writer, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    ($fg:expr, $bg:expr) => (print!($fg, $bg, "\n"));
    ($fg:expr, $bg:expr, $($arg:tt)*) => {{
        crate::print!($fg, $bg, "\n{}", format_args!($($arg)*));
    }};
}

pub struct VgaWriter {
    fg: Color,
    bg: Color,
}

impl VgaWriter {
    pub fn new(fg: Color, bg: Color) -> Self {
        VgaWriter { fg, bg }
    }
}

impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            crate::write_str(s, self.fg, self.bg);
        }
        Ok(())
    }
}
