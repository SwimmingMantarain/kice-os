#[repr(C)]
#[derive(Debug)]
pub struct MultibootTag {
    typ: u32,
    size: u32,
}

#[repr(C)]
pub struct MultibootMemoryMap {
    typ: u32,
    size : u32,
    entry_size: u32,
    entry_version: u32,
    entries: [MemoryMapEntry; 0]
}

#[repr(C)]
pub struct MemoryMapEntry {
    base_addr: u64,
    length: u64,
    typ: u32, // 1 = Usable, other = reserved/special memory
}