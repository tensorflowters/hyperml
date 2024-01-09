use volatile::Volatile;

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
pub struct Writer {
    // The column_position field keeps track of the current position in the last row.
    column_position: usize,
    // The current foreground and background colors are specified by color_code and a reference to the VGA buffer is stored in buffer.
    color_code: ColorCode,
    // The 'static lifetime specifies that the reference is valid for the whole program run time (which is true for the VGA text buffer).
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

    fn new_line(&mut self) {/* TODO */}
}

// To write some characters to the screen, you can create a temporary function:
pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}