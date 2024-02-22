#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static TEXT: &[u8] = b"Lollipop - Kernel";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xB8000 as *mut u8;

    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xB;
        }
    }

    loop {}
}
