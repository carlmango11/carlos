#![no_std]
#![no_main]

use core::panic::PanicInfo;
use x86::io::inb;
use x86::io::outb;

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

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

#[unsafe(no_mangle)]
fn remap_pic() {
    unsafe {
        let a1 = inb(PIC1_DATA);
        let a2 = inb(PIC2_DATA);

        outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
        outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);

        outb(PIC1_DATA, 0x20);
        outb(PIC2_DATA, 0x28);

        outb(PIC1_DATA, 4);
        outb(PIC2_DATA, 2);

        outb(PIC1_DATA, ICW4_8086);
        outb(PIC2_DATA, ICW4_8086);

        outb(PIC1_DATA, a1);
        outb(PIC2_DATA, a2);
    }
}

fn write_char(loc: isize, c: char) {
    let vga_buffer = 0xb8000 as *mut u16;
    let merged = 0x0f00 | c as u16;

    unsafe {
        *vga_buffer.offset(loc) = merged;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
