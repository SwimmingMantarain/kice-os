
#[repr(C, packed)]
struct idt_entry {
    low_bits: u16,      // Low 16 bits (0-15) of the offset
    css: u16,           // Code Segment Selector
    zero: u8,           // Just 0
    mid_bits: u16,      // Middle 16 bits (16-31) of the offset
    top_bits: u32,      // High 32 bits (32-64) of the offset
    res: u32,           // Reserved for something at a restaurant idk?
}

