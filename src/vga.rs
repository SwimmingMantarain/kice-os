// Constants
const VGA_BUFFER_ADDR: usize = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer([
    [Volatile<u16>; BUFFER_WIDTH]; BUFFER_HEIGHT
]);

pub struct VGA {
    buffer: &'static mut Buffer,
    current_row: usize,
    current_col: usize,
    color_code: u8,
}

impl VGA {
    pub fn new(foreground: Color, background: Color) -> Self {
        let color_code = (background as u8) << 4 | (foreground as u8);
        Self {
            buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut &mut Buffer) },
            current_row: 0,
            current_col: 0,
            color_code,
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.0[row][col].write(Self::encode_char(' ', self.color_code));
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.current_col >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.0[self.current_row][self.current_col].write(Self::encode_char(byte as char, self.color_code));
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of ASCII char set
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn encode_char(c: char, color_code: u8) -> u16 {
        (color_code as u16) << 8 | (c as u16)
    }

    fn new_line(&mut self) {
        self.current_row += 1;
        self.current_col = 0;

        if self.current_row >= BUFFER_HEIGHT {
            self.scroll_up();
            self.current_row = BUFFER_HEIGHT - 1;
        }
    }

    fn scroll_up(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.0[row][col].read();
                self.buffer.0[row - 1][col].write(char);
            }
        }

        for col in 0..BUFFER_WIDTH {
            self.buffer.0[BUFFER_HEIGHT - 1][col].write(Self::encode_char(' ', self.color_code));
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

struct Volatile<T> {
    value: T,
}

impl<T> Volatile<T> {
    fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(&self.value) }
    }

    fn write(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(&mut self.value, value); }
    }
}