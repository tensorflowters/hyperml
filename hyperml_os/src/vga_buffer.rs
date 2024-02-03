use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// With lazy_static, we can define our static WRITER without problems:
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Normally the compiler would issue a warning for each unused variant. By using the #[allow(dead_code)] attribute, we disable these warnings for the Color enum.
#[allow(dead_code)]
// By deriving the Copy, Clone, Debug, PartialEq, and Eq traits, we enable copy semantics for the type and make it printable and comparable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// We use a C-like enum here to explicitly specify the number for each color. Because of the repr(u8) attribute, each enum variant is stored as a u8. Actually 4 bits would be sufficient, but Rust doesn’t have a u4 type.
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


// To represent a full color code that specifies foreground and background color, we create a newtype on top of u8

// The ColorCode struct contains the full color byte, containing foreground and background color. Like before, we derive the Copy and Debug traits for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// To ensure that the ColorCode has the exact same data layout as a u8, we use the repr(transparent) attribute.
#[repr(transparent)]
// To represent a full color code that specifies foreground and background color, we create a newtype on top of u8
struct ColorCode(u8);
// The ColorCode struct contains the full color byte, containing foreground and background color.
// Like before, we derive the Copy and Debug traits for it.
// To ensure that the ColorCode has the exact same data layout as a u8, we use the repr(transparent) attribute.
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


//Now we can add structures to represent a screen character and the text buffer:

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//Since the field ordering in default structs is undefined in Rust, we need the repr(C) attribute.
// It guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering.
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}


const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// For the Buffer struct, we use repr(transparent) again to ensure that it has the same memory layout as its single field.
#[repr(transparent)]
struct Buffer {
    //Instead of a ScreenChar, we’re now using a Volatile<ScreenChar>. (The Volatile type is generic and can wrap (almost) any type). This ensures that we can’t accidentally write to it “normally”. 
    // Instead, we have to use the write method now.
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


// To actually write to screen, we now create a writer type

// The writer will always write to the last line and shift lines up when a line is full (or on \n).
// Note that we need an explicit lifetime here to tell the compiler how long the reference is valid.
// pub struct Writer {
//     // The column_position field keeps track of the current position in the last row.
//     column_position: usize,
//     // The current foreground and background colors are specified by color_code and a reference to the VGA buffer is stored in buffer.
//     color_code: ColorCode,
//     // The 'static lifetime specifies that the reference is valid for the whole program run time (which is true for the VGA text buffer).
//     buffer: &'static mut Buffer,
// }
// NB: To understand what’s happening here, we need to know that statics are initialized at compile time, in contrast to normal variables that are initialized at run time.
// The component of the Rust compiler that evaluates such initialization expressions is called the “const evaluator”.
// Its functionality is still limited, but there is ongoing work to expand it, for example in the “Allow panicking in constants” RFC.
// The issue with ColorCode::new would be solvable by using const functions,
// but the fundamental problem here is that Rust’s const evaluator is not able to convert raw pointers to references at compile time. Maybe it will work someday, but until then, we have to find another solution.

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// Now we can use the Writer to modify the buffer’s characters.
// First we create a method to write a single ASCII byte:
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // If the byte is the newline byte \n, the writer does not print anything. Instead, it calls a new_line method, which we’ll implement later.
            b'\n' => self.new_line(),
            // Other bytes get printed to the screen in the second match case
            byte => {
                // When printing a byte, the writer checks if the current line is full.
                if self.column_position >= BUFFER_WIDTH {
                    // In that case, a new_line call is used to wrap the line
                    self.new_line();
                }

                // Then it writes a new ScreenChar to the buffer at the current position.
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;

                // Instead of a typical assignment using =, we’re now using the write method. 
                // Now we can guarantee that the compiler will never optimize away this write.
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                // Finally, the current column position is advanced
                self.column_position += 1;
            }
        }
    }

    // To print whole strings, we can convert them to bytes and print them one-by-one
    pub fn write_string(&mut self, s: &str) {
        //The VGA text buffer only supports ASCII and the additional bytes of code page 437.
        // Rust strings are UTF-8 by default, so they might contain bytes that are not supported by the VGA text buffer.
        for byte in s.bytes() {
            // We use a match to differentiate printable ASCII bytes (a newline or anything in between a space character and a ~ character) and unprintable bytes
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }

    // ight now, we just ignore newlines and characters that don’t fit into the line anymore
    // Instead, we want to move every character one line up (the top line gets deleted) and start at the beginning of the last line again.
    // To do this, we add an implementation for the new_line method of Writer:
    fn new_line(&mut self) {
        // We iterate over all the screen characters and move each character one row up.
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                // Note that the upper bound of the range notation (..) is exclusive.
                // We also omit the 0th row (the first range starts at 1) because it’s the row that is shifted off screen.
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // This method clears a row by overwriting all of its characters with a space character.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// To write some characters to the screen, you can create a temporary function:
// pub fn print_something() {
//     use core::fmt::Write;
//     let mut writer = Writer {
//         column_position: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Black),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };

//     writer.write_byte(b'H');
//     writer.write_string("ello! ");
//     write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
// }
// After adding Mutex, we can delete the print_something function and print directly from our _start function:

// That way, we can easily print different types, like integers or floats.
// To support them, we need to implement the core::fmt::Write trait.
// The only required method of this trait is write_str, which looks quite similar to our write_string method, just with a fmt::Result return type:
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
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

// One thing that we changed from the original println definition is that we prefixed the invocations of the print! macro with $crate too.

// This ensures that we don’t need to import the print! macro too if we only want to use println.

//Like in the standard library, we add the #[macro_export] attribute to both macros to make them available everywhere in our crate. Note that this places the macros in the root namespace of the crate, so importing them via use crate::vga_buffer::println does not work. Instead, we have to do use crate::println.

// The _print function locks our static WRITER and calls the write_fmt method on it. This method is from the Write trait, which we need to import. The additional unwrap() at the end panics if printing isn’t successful. But since we always return Ok in write_str, that should not happen.

// Since the macros need to be able to call _print from outside of the module, the function needs to be public. However, since we consider this a private implementation detail, we add the doc(hidden) attribute to hide it from the generated documentation.

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}