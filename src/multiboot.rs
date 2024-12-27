use crate::{Color, debug_println};

pub const MULTIBOOT2_MAGIC: u32 = 0x36d76289;

#[repr(C)]
pub struct MultibootTagHeader {
    total_size: u32,
    reserved: u32,
}

#[repr(C)]
pub struct MultibootTag {
    typ: u32,
    size: u32,
}

#[repr(C)]
pub struct MemoryMapTag {
    typ: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
}

#[repr(C)]
pub struct MemoryMapEntry {
    base_addr: u64,
    length: u64,
    typ: u32,
    reserved: u32,
}

pub fn check_magic(magic: u32) -> bool {
    if magic != MULTIBOOT2_MAGIC {
        debug_println!(Color::Red, Color::Black, "Invalid multiboot2 magic number: 0x{:x}", magic);
        debug_println!(Color::Red, Color::Black, "Expected: 0x{:x}", MULTIBOOT2_MAGIC);
        false
    } else {
        debug_println!(Color::Green, Color::Black, "Valid multiboot2 magic number: 0x{:x}", magic);
        true
    }
}

pub fn parse_info(info_ptr: u32) {
    let header = unsafe { &*(info_ptr as *const MultibootTagHeader) };
    debug_println!(Color::Green, Color::Black, "Multiboot2 Info Total Size: {} bytes", header.total_size);

    let mut current = (info_ptr as usize + core::mem::size_of::<MultibootTagHeader>()) as *const MultibootTag;
    
    while unsafe { (*current).typ } != 0 {
        let tag = unsafe { &*current };
        
        match tag.typ {
            6 => parse_memory_map(current as *const MemoryMapTag),
            _ => debug_println!(Color::Yellow, Color::Black, "Skipping tag type: {}", tag.typ),
        }

        // Move to the next tag, aligned to 8 bytes
        current = ((current as usize + tag.size as usize + 7) & !7) as *const MultibootTag;
    }
}

fn parse_memory_map(tag_ptr: *const MemoryMapTag) {
    let tag = unsafe { &*tag_ptr };
    debug_println!(Color::Green, Color::Black, "\nMemory Map:");
    
    let entries_ptr = (tag_ptr as usize + core::mem::size_of::<MemoryMapTag>()) as *const MemoryMapEntry;
    let entry_count = (tag.size - core::mem::size_of::<MemoryMapTag>() as u32) / tag.entry_size;
    
    for i in 0..entry_count {
        let entry = unsafe { &*entries_ptr.add(i as usize) };
        let type_str = match entry.typ {
            1 => "Available",
            2 => "Reserved",
            3 => "ACPI",
            4 => "NVS",
            5 => "Defective",
            _ => "Unknown",
        };
        
        debug_println!(Color::Green, Color::Black, 
            "  Region: base=0x{:016x}, length=0x{:016x}, type={}", 
            entry.base_addr, 
            entry.length,
            type_str
        );
    }
}