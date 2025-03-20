use multiboot2::MemoryArea;

pub struct FrameAllocator {
    next_frame: usize,
    max_frame: usize,
}

impl FrameAllocator {
    pub fn new(memory_map: &[MemoryArea]) -> Self {
        let max = memory_map.iter()
            .filter(|area| area.typ() == MemoryType::AVAILABLE)
            .map(|area| area.end_address() / 0x200000)
            .max()
            .unwrap();

        Self {
            next_frame: 0,
            max_frame: max,
        }
    }

    pub fn allocate(&mut self) -> Option<usize> {
        (self.next_frame < self.max_frame).then(|| {
            let frame = self.next_frame;
            self.next_frame += 1;
            frame * 0x200000  // Return physical address
        })
    }
}
