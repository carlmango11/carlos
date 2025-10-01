#![no_std]
#![no_main]

static HELLO: &[u8] = b"CARL";

use core::panic::PanicInfo;

macro_rules! kpanic {
    ($msg:expr) => {{
        let vga_buffer = 0xb8000 as *mut u8;
        unsafe {
            *vga_buffer.offset(0) = b'!';
            *vga_buffer.offset(1) = 0x4f;
        }
        loop {}
    }};
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u16;

    // for (i, byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = b'C';
    //         *vga_buffer.offset((i as isize * 2) + 1) = 0x0f;
    //     }
    // }

    for i in 0..5 {
        unsafe {
            write_char(0, 'C');
            write_char(1, 'C');
            // *vga_buffer.offset(i as isize * 2) = b'C';
            // *vga_buffer.offset((i as isize * 2) + 1) = 0x0f;
        }
    }

    loop {}
}

fn write_char(loc: isize, c: char) {
    let merged = 0x0f00 | c as u16;
    
    unsafe {
        *vga_buffer.offset(loc) = merged;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
