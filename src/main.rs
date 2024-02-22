#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static TEXT: &str = "Lollipop - Kernel";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("{}", TEXT);

    loop {}
}
