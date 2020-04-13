
#[allow(dead_code)]
#[repr(u8)]
enum Color {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
struct VgaChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_ROWS: usize = 25;
const BUFFER_COLS: usize = 80;

use volatile::Volatile;

#[repr(transparent)]
struct VgaBufferLine {
    chars: [Volatile<VgaChar>; BUFFER_COLS],
}

#[repr(transparent)]
struct VgaBuffer {
    rows: [VgaBufferLine; BUFFER_ROWS],
}

struct Writer {
    current_column: usize,
    current_color: ColorCode,
    buffer: &'static mut VgaBuffer,
}

impl Writer {

    fn shift_rows_up(&mut self) {

        for i in 1..BUFFER_ROWS {
            for j in 0..BUFFER_COLS {
                self.buffer.rows[i-1].chars[j].write(
                    self.buffer.rows[i].chars[j].read()
                );
            }
        }

        self.clear_last_row();
        self.current_column = 0;
    }

    fn clear_last_row(&mut self) {
        let blank = VgaChar {
            ascii_char: b' ',
            color_code: ColorCode::new(Color::Black, Color::Black)
        };
        for j in 0..BUFFER_COLS {
            self.buffer.rows[BUFFER_ROWS-1].chars[j].write(blank);
        }
    }

    fn write_byte(&mut self, byte: u8) {

        if self.current_column >= BUFFER_COLS {
            self.shift_rows_up()
        }

        self.buffer.rows[BUFFER_ROWS-1].chars[self.current_column].write(VgaChar {
            ascii_char: byte,
            color_code: self.current_color
        });
        self.current_column += 1;
    }

    fn set_color(&mut self, color_code: ColorCode) {
        self.current_color = color_code;
    }

    fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                b'\n' => { self.shift_rows_up() },
                b'\r' => { self.current_column = 0 },
                0x20..=0x7e => { self.write_byte(byte) },
                _ => { self.write_byte(b'?') }
            }
        }
    }
}
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;
 
lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new( Writer {
        current_column: 0,
        current_color: ColorCode::new(Color::Cyan, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) }
    });
}


pub fn print_something() {
    use core::fmt::Write;
    let mut writer = WRITER.lock();
    writer.write_string("Hello world de ouf!?!Hello world de ouf!?!");
    writer.set_color(ColorCode::new(Color::Yellow, Color::Pink));
    write!(writer, "\nTHIS WILL NEVER BE DISPLAYED\rHello world de ouf!?! {} Hello world de ouf!?!Hello world de ouf!?!Hello world de ouf!?!Hello world de ouf!?!", 42).unwrap();

    let color = ColorCode::new(Color::Black, Color::Pink);  
    writer.set_color(color);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn simple_println() {
    serial_print!("Test println doesn't panic... ");
    println!("Ca devrait marcher au taquet ca... {} !", 42);
    serial_println!("Ok!"); 
}

#[test_case]
fn simple_println() {
    serial_print!("Test massive println doesn't panic... ");
    for _ in 0..=400 {
        println!("Ca devrait marcher au taquet ca... {} !", 42);
    }
    serial_println!("Ok!"); 
}