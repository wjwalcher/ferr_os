/** Utility for writing text to the VGA buffer */

// CONSTANTS
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// DATA TYPE DEFINITIONS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
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
    White = 15
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScreenChar {
    ascii_char: u8, 
    color_code: ColorCode
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct ScreenWriter {
    color_code: ColorCode, 
    buffer: &'static mut Buffer
}

static mut COL : usize = 0;
static mut ROW : usize = 0;

impl ScreenWriter {
    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            match byte {
                b'\n' => self.new_line(),
                byte => {
                    if COL >= BUFFER_WIDTH {
                        self.new_line();
                    }

                    let color_code = self.color_code;

                    self.buffer.chars[ROW][COL] = ScreenChar {
                        ascii_char: byte, 
                        color_code
                    };

                    COL += 1;
                }
            }
        }
    }

    fn new_line(&mut self) { 
        unsafe {
            COL = 0;
            ROW += 1;
        } 
    }

    // Print string to the VGA buffer
    pub fn print_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // is valid ASCII byte?
                0x20..=0x7e | b'\n' => self.write_byte(byte), 
                // if not valid...
                _ => self.write_byte(0xfe)
            }
        }
    }
}

pub fn kprintln(string: &str) {
    unsafe {
        let mut writer = ScreenWriter {
            color_code: ColorCode::new(Color::Blue, Color::Black), 
            buffer: &mut *(0xb8000 as *mut Buffer) 
        }; 

        writer.print_string(string);
        writer.new_line();
    }
}