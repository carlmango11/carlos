#![no_std]
#![no_main]

mod paging;
mod pic;

extern crate alloc;
extern crate linked_list_allocator;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

use core::panic::PanicInfo;
use core::slice;
use elf::endian::LittleEndian;
use elf::{ElfBytes, ParseError};
use x86::io::inb;
use x86::io::outb;
use crate::paging::PageTable;

unsafe extern "C" {
    static heap_start: u8;
    static heap_end: u8;

    static _binary_build_bin_hello_elf_start: u8;
    static _binary_build_bin_hello_elf_end: u8;
}

fn write_char(row: isize, col: isize, format: u16, c: char) {
    let vga_buffer = 0xb8000 as *mut u16;
    let merged = format | c as u16;

    unsafe {
        *vga_buffer.offset(col + (80 * row)) = merged;
    }
}

fn println(row: isize, s: String) {
    write_str(row, 0x0f00, s.as_str());
}

fn write_str(row: isize, format: u16, s: &str) {
    let mut col: isize = 0;

    for c in s.chars() {
        write_char(row, col, format, c);
        col += 1;
    }
}

unsafe fn init_heap() {
    let start = &heap_start as *const u8 as usize;
    let start_mut = start as *const u8 as *mut u8;
    let end = &heap_end as *const u8 as usize;

    let size = end - start;

    HEAP.lock().init(start_mut, size);
}

struct Process {
    page_tables: PageTable,
    elf: ElfBytes<'static, LittleEndian>,
}

#[unsafe(no_mangle)]
pub extern "C" fn main_rust(a: usize) {
    unsafe {
        init_heap();
    }

    let start_bin = unsafe { &_binary_build_bin_hello_elf_start as *const u8 };
    let end_bin = unsafe { &_binary_build_bin_hello_elf_end as *const u8 };

    let mut ps: Vec<Process> = Vec::new();

    let elf_result = load_elf(start_bin, end_bin);
    if elf_result.is_err() {
        panic!("elf parse err: {}", elf_result.unwrap_err());
    }

    println(2, format!("TRY"));
    let x: [Option<i64>; 100] = core::array::from_fn(|_| None);
    // let a = paging::PageTable::new();
    println(3, format!("unwrap {:?}", x.len()));
    return;

    let p = Process {
        page_tables: paging::PageTable::new(),
        elf: elf_result.unwrap(),
    };
    println(2, format!("unwrap ED"));

    ps.push(p);

    let pid = ps.len() - 1;
    execute(&mut ps, pid);
}

fn execute(ps: &mut Vec<Process>, pid: usize) {
    println(2, format!("exec: {}", pid));

    let p = ps.get_mut(pid).unwrap();

    for s in p.elf.segments().unwrap() {
        if s.p_type != 0x00000001 {
            continue;
        }

        p.page_tables.load_page(s.p_vaddr, s.p_offset);
    }
}

fn load_elf(
    start: *const u8,
    end: *const u8,
) -> Result<ElfBytes<'static, LittleEndian>, ParseError> {
    let raw = unsafe { slice::from_raw_parts(start, end.offset_from(start) as usize) };
    ElfBytes::<LittleEndian>::minimal_parse(raw)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let m = format!("panic: {}", _info);
    write_str(0, 0xfc00, m.as_str());
    loop {}
}
