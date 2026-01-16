use crate::{multiboot_info, println};
use alloc::format;
use alloc::vec::Vec;

struct PageEntry {
    start: u64,
    free: bool,
}

#[derive(Debug)]
#[repr(C)]
pub struct MultibootMemEntry {
    addr: u64,  // physical base
    len: u64,   // length
    etype: u32, // 1 = usable RAM
    reserved: u32,
}

#[repr(C)]
struct MultibootInfo {
    total_size: u32,
    reserved: u32,
}

#[repr(C)]
struct MultibootMemTag {
    ttype: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
}

fn create_page_directory() -> Vec<PageEntry>{
    let mb_entries = unsafe {read_mb_entries()};

    let mut pes: Vec<PageEntry> = Vec::new();

    for (i, e) in mb_entries.iter().enumerate() {
        let pe = PageEntry{
            start: e.addr + (i as u64 * 4096),
            free: true,
        };

        pes.push(pe);
    }

    pes
}

pub unsafe fn read_mb_entries() -> Vec<&'static MultibootMemEntry> {
    let mbi = &*(multiboot_info as *const MultibootInfo);

    let mut current = multiboot_info + core::mem::size_of::<MultibootInfo>();
    let end = current + mbi.total_size as usize;

    while current < end {
        let tag = &*(current as *const MultibootMemTag);

        if tag.ttype == 0 && tag.size == 8 {
            break;
        }

        if tag.ttype == 6 {
            let x = to_entry(tag);
            println(1, format!("t6 = {}", x.len()));
            return x;
        }

        current += align_up(tag.size as usize, 8);
    }

    panic!("no multiboot tag found");
}

fn align_up(v: usize, align: usize) -> usize {
    (v + align - 1) & !(align - 1)
}

unsafe fn to_entry(tag: &MultibootMemTag) -> Vec<&MultibootMemEntry> {
    let mut current = core::mem::size_of::<MultibootMemTag>();
    let end = tag.size as usize;

    let mut entries: Vec<&MultibootMemEntry> = Vec::new();

    let mut i = 4;
    while current < end {
        let entry = &*((tag as *const _ as *const u8).add(current) as *const MultibootMemEntry);

        // println(
        //     2 + i,
        //     format!(
        //         "entry = type:{} len:{} MiB addr:{:X}",
        //         entry.etype,
        //         entry.len / 1024 / 1024,
        //         entry.addr
        //     ),
        // );

        if entry.etype == 1 {
            entries.push(entry);
            i += 1;
        }

        current = current + tag.entry_size as usize;
    }

    entries
}

// pub(crate) fn create_index(m: &MMap) -> Vec<PageEntry> {
//    return Vec::new();
// }
