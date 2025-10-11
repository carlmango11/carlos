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
pub extern "C" fn _main() -> ! {
    remap_pic();

    for _i in 0..5 {
        write_char(0, 'C');
        write_char(1, 'C');
    }

    loop {}
}

#[repr(C, packed)]
struct idt_ptr {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
struct idt_entry {
    offset_lo: u16,
    seg: u16,
    ist: u8,
    type_field: u8,
    off2: u16,
    off3: u32,
    reserved: u32,
}

fn set_up_interrupts() {
    // idt_set(0x21)
}

#[unsafe(no_mangle)]
fn isr_handler() {
    // if (r->int_no == 0x21) {
    // uint8_t scancode = inb(0x60);
    // // process key...
    // }

    // write_char(4, 'X');
    // Send End of Interrupt (EOI) to PIC
    // unsafe {
    //     outb(0x20, 0x20);
    // }
}

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

#[unsafe(no_mangle)]
fn testy() {
    write_char(9, 'Y');
}

#[unsafe(no_mangle)]
fn testy2() {
    write_char(10, 'Z');
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
