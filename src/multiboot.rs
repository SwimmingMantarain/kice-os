use crate::Color;

#[repr(C, packed)]
pub struct MemMapEntry {
    base_addr: u32,
    size: u32,
    typ_e: u16,
}

#[repr(C, packed)]
pub struct MultibootTag {
    typ_e: u32,
    size: u32,
}

#[repr(C, packed)]
pub struct MemMapTag {
    typ_e: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
    entries: [MemMapEntry; 0], // Flexible
}

pub fn parse_memory_map(multiboot_info: *const MultibootTag) {
    let mut current_tag = multiboot_info;
    while !current_tag.is_null() {
        let tag = unsafe { &*current_tag };
        if tag.typ_e == 6 { // Usable Memory
            let mem_map_tag = unsafe { &*(current_tag as *const MemMapTag) };
            let mut offset = 0;
            while offset < mem_map_tag.size as usize - core::mem::size_of::<MemMapTag>() {
                let entry = unsafe {
                    &*((mem_map_tag.entries.as_ptr() as usize + offset) as *const MemMapEntry)
                };
                if entry.typ_e == 1 {
                    let base_addr = entry.base_addr;
                    let size = entry.size;
                    println!(Color::Green, Color::Black,
                        "Usable Memory: start=0x{:x}, length=0x{:x}",
                        base_addr, size
                    );
                }
                offset += mem_map_tag.entry_size as usize;
            }
        }
        current_tag = (current_tag as usize + tag.size as usize + 7 & !7) as *const MultibootTag;
    }
}

pub fn check(multiboot_info_addr: *const u8) {
    let tot_size = unsafe { *(multiboot_info_addr as *const u32) };
    let magic = unsafe { *((multiboot_info_addr as usize + 4) as *const u32) };

    println!(Color::Green, Color::Black, "Multiboot Total Size: {:#x}", tot_size);
    println!(Color::Green, Color::Black, "Multiboot Magic: {:#x}", magic);
}