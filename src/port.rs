use core::arch::asm;


/// Write byte to a specified port
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nostack, nomem, preserves_flags)
    );
}

/// Read byte from a specified port
pub unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nostack, nomem, preserves_flags)
    );
    value
}