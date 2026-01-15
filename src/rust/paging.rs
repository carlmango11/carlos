use alloc::boxed::Box;
use core::fmt::{Display, Formatter};

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
    name: u64,
    entries: [u64; 512],
    // tables: [Option<Box<PageTable>>; 512],
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            name: 3,
            entries: [0; 512],
            // tables: core::array::from_fn(|_| None),
        }
    }

    // | 63 … 48 | 47 … 39 | 38 … 30 | 29 … 21 | 20 … 12 | 11 … 0 |
    // | sign   |   PML4   |   PDPT  |    PD   |    PT   | offset|
    // pub fn load_page(&mut self, vaddr: u64, paddr: u64) {
    //     let v1 = (vaddr >> 12) & 0x1FF;
    //     let v2 = (vaddr >> 21) & 0x1FF;
    //     let v3 = (vaddr >> 30) & 0x1FF;
    //     let v4 = (vaddr >> 39) & 0x1FF;
    //
    //     let entries = &mut self.entries;
    //     let t3 = self.tables[v4 as usize].get_or_insert_with(|| {
    //         let pt = Box::new(PageTable::new());
    //         entries[v4 as usize] = (pt.entries.as_ptr() as u64) | 0x11;
    //         pt
    //     });
    //
    //     let entries = &mut t3.entries;
    //     let t2 = t3.tables[v3 as usize].get_or_insert_with(|| {
    //         let pt = Box::new(PageTable::new());
    //         entries[v3 as usize] = (pt.entries.as_ptr() as u64) | 0x11;
    //         pt
    //     });
    //
    //     let entries = &mut t2.entries;
    //     let t1 = t2.tables[v2 as usize].get_or_insert_with(|| {
    //         let pt = Box::new(PageTable::new());
    //         entries[v2 as usize] = (pt.entries.as_ptr() as u64) | 0x11;
    //         pt
    //     });
    //
    //     t1.entries[v1 as usize] = paddr | 0x11;
    // }
}
