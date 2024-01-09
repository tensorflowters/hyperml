#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on entry.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Note that we don’t have to import the macro in the main function, because it already lives in the root namespace.
    // As expected, we now see a “Hello World!” on the screen:
    println!("Hello World{}", "!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}