use alloc::boxed::Box;

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [u64; 512],
    tables: [Option<Box<PageTable>>; 512],
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: [0; 512],
            tables: core::array::from_fn(|_| None),
        }
    }

    // | 63 … 48 | 47 … 39 | 38 … 30 | 29 … 21 | 20 … 12 | 11 … 0 |
    // | sign   |   PML4   |   PDPT  |    PD   |    PT   | offset|
    pub fn load_page(&mut self, vaddr: u64, paddr: u64) {
        let v1 = (vaddr >> 12) & 0x1FF;
        let v2 = (vaddr >> 21) & 0x1FF;
        let v3 = (vaddr >> 30) & 0x1FF;
        let v4 = (vaddr >> 39) & 0x1FF;

        let t3 = self.tables[v4 as usize].get_or_insert_with(|| {
            let pt = PageTable::new();
            let pte = pt.entries.as_ptr();
            self.entries[v4 as usize] = unsafe {*pte} | 0x11;

            Box::new(pt)
        });

        let t2 = t3.tables[v3 as usize].get_or_insert_with(|| {
            let pt = PageTable::new();
            let pt_ptr = pt.entries.as_ptr();
            self.entries[v3 as usize] = unsafe {*pt_ptr } | 0x11;

            Box::new(pt)
        });

        let t1 = t2.tables[v2 as usize].get_or_insert_with(|| {
            let pt = PageTable::new();
            let pt_ptr = pt.entries.as_ptr();
            self.entries[v2 as usize] = unsafe {*pt_ptr } | 0x11;

            Box::new(pt)
        });

        t1.entries[v1 as usize] = paddr | 0x11;
    }
}
