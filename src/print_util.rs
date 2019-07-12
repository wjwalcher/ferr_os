/** Utility for writing text to the VGA buffer */

extern crate cpuio;

use volatile::Volatile;
use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

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
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct ScreenWriter {
    column_position: usize, 
    row_position: usize, 
    color_code: ColorCode, 
    buffer: &'static mut Buffer
}

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
        column_position: 0, 
        row_position: 0, 
        color_code: ColorCode::new(Color::Blue, Color::Black), 
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
    });
}

impl ScreenWriter {
    pub fn write_byte(&mut self, byte: u8) {
       
        match byte {
            b'\n' => self.new_line(),
            byte => {
                let col = self.column_position;
                let row = self.row_position;

                if col >= BUFFER_WIDTH {
                    self.new_line();
                }

                if row >= BUFFER_HEIGHT {
                    self.shift_lines_up();
                    self.row_position = BUFFER_HEIGHT - 1;
                }

                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte, 
                    color_code
                });

                self.column_position += 1;
            }
            
        }

    }

    fn new_line(&mut self) { 
        self.column_position = 0;
        self.row_position += 1;
    }

    // Can I get an O(r * h) react 
    fn shift_lines_up(&mut self) {
        for col in 0..BUFFER_WIDTH {
            for row in 1..BUFFER_HEIGHT {
                self.buffer.chars[row-1][col].write(
                    self.buffer.chars[row][col].read()
                );
            }
        }
    }

    // Print string to the VGA buffer
    #[allow(exceeding_bitshifts)]
    pub fn print_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // is valid ASCII byte?
                0x20..=0x7e | b'\n' => self.write_byte(byte), 
                // if not valid...
                _ => self.write_byte(0xfe)
            }
        }

        // TODO: Get cursor to update upon writing a line
        // let column_offset = self.column_position * 80;
        // let row_offset = self.row_position;
        // let cursor_position : u16 = (column_offset + row_offset) as u16;
        // unsafe {
        //     cpuio::outb(15, 0x3D4);
        //     cpuio::outb((cursor_position & 0xFF) as u8, 0x3D5);
        //     cpuio::outb(14, 0x3D4);
        //     cpuio::outb(((cursor_position >> 8) & 0xFF) as u8, 0x3D5);
        // }

        // For now I'll just turn off the cursor blink 
        unsafe {
            cpuio::outb(14, 0x3D4);
            cpuio::outb(0xFF, 0x3D5);
        }

    }
}

pub fn kprintln(string: &str) {
    WRITER.lock().print_string(string);
    WRITER.lock().new_line();
}