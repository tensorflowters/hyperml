#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on entry.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}