#![no_std]
#![no_main]

mod paging;

extern crate alloc;
extern crate linked_list_allocator;

use alloc::vec::Vec;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

use core::panic::PanicInfo;
use core::slice;
use x86::io::inb;
use x86::io::outb;
use elf::ElfBytes;
use elf::endian::LittleEndian;
use elf::segment::ProgramHeader;


unsafe extern "C" {
    static heap_start: u8;
    static heap_end: u8;

    static _binary_build_bin_hello_elf_start: u8;
    static _binary_build_bin_hello_elf_size: usize;

    static mut user_l4: u8;
    static mut user_l3: u8;
    static mut user_l2: u8;
    static mut user_l1: u8;
}

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
        // Save current masks
        let a1 = inb(PIC1_DATA);
        let a2 = inb(PIC2_DATA);

        // Start init sequence (cascade mode, expect ICW4)
        outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
        outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);

        // Set new vector offsets
        outb(PIC1_DATA, 0x20); // IRQ0..7 -> 0x20..0x27
        outb(PIC2_DATA, 0x28); // IRQ8..15 -> 0x28..0x2F

        // Tell Master PIC there is a slave at IRQ2, and tell Slave its cascade identity
        outb(PIC1_DATA, 4);
        outb(PIC2_DATA, 2);

        // Set x86 mode
        outb(PIC1_DATA, ICW4_8086);
        outb(PIC2_DATA, ICW4_8086);

        // Restore masks but UNMASK keyboard (IRQ1) by clearing bit 1 on master
        // let new_a1 = a1 & !(1 << 1);
        // outb(PIC1_DATA, new_a1);
        outb(PIC1_DATA, 0xFD);

        outb(PIC2_DATA, a2);
    }
}

fn write_char(row: isize, col: isize, format: u16, c: char) {
    let vga_buffer = 0xb8000 as *mut u16;
    let merged = format | c as u16;

    unsafe {
        *vga_buffer.offset(col + (80 * row)) = merged;
    }
}

fn write_str(row: isize, format: u16, s: &str) {
    let mut col: isize = 0;

    for c in s.chars() {
        write_char(row, col, format, c);
        col += 1;
    }
}

pub fn int_to_str<'a>(mut n: i32, buf: &'a mut [u8]) -> &'a str {
    // Index to write from the end backwards
    let len = buf.len();
    let mut i = len;

    // Special case zero
    if n == 0 {
        buf[len - 1] = b'0';
        return core::str::from_utf8(&buf[len - 1..len]).unwrap();
    }

    // Handle sign
    let negative = n < 0;
    let mut v = if negative {
        // Use i32::MIN safely by converting to i64
        -(n as i64)
    } else {
        n as i64
    };

    // Write digits backwards
    while v > 0 {
        let digit = (v % 10) as u8;
        i -= 1;
        buf[i] = b'0' + digit;
        v /= 10;
    }

    // Add sign if needed
    if negative {
        i -= 1;
        buf[i] = b'-';
    }

    // Convert to &str
    core::str::from_utf8(&buf[i..len]).unwrap()
}

unsafe fn init_heap() {
    let start = &heap_start as *const u8 as usize;
    let start_mut = start as *const u8 as *mut u8;
    let end = &heap_end as *const u8 as usize;

    let size = end - start;

    HEAP.lock().init(start_mut, size);
}

struct Process {
    page_tables: paging::PageTable,
    elf: ElfBytes<'static, LittleEndian>,
    // bin_start: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn main_rust(a: usize) {
    unsafe {
        init_heap();
    }

    let start_bin = unsafe { &_binary_build_bin_hello_elf_start as *const u8 };
    let size = unsafe { _binary_build_bin_hello_elf_size };

    let mut ps: Vec<Process> = Vec::new();

    let p = Process{
        page_tables: paging::PageTable::new(),
        elf: unsafe { load_elf(start_bin, size) },
    };

    ps.push(p);

    let pid = ps.len() - 1;
    execute(&mut ps, pid);
}

fn execute(ps: &mut Vec<Process>, pid: usize) {
    let p = ps.get_mut(pid).unwrap();

    for s in p.elf.segments().unwrap() {
        if s.p_type != 0x00000001 {
            continue;
        }

        p.page_tables.load_page(s.p_vaddr, s.p_offset);
    }
}

unsafe fn load_elf(start: *const u8, len: usize) -> ElfBytes<'static, LittleEndian> {
    let raw = slice::from_raw_parts(start, len);
     ElfBytes::<LittleEndian>::minimal_parse(raw).unwrap()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
