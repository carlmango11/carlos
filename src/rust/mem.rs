use alloc::vec::Vec;

struct PageEntry {
    start: u64,
    free: bool,
}

// pub(crate) fn create_index(m: &MMap) -> Vec<PageEntry> {
//    return Vec::new();
// }