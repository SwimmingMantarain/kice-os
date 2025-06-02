[org 0x7c00]

KERNEL_OFFSET equ 0x1000

mov [BOOT_DRIVE], dl ; BIOS sets the boot drive in dl on boot
mov bp, 0x9000
mov sp, bp ; Setup 16-bit stack

mov bx, MSG_REAL_MODE ; print first message to the world
call print
call print_nl

call load_kernel ; does what it says :D (hopefully)
call switch_to_pm ; disables interrupts, loads GDT, switches to pm

jmp $ ; If this runs I will ... >:(

%include "boot/print.asm"

BOOT_DRIVE db, 0 ; dl may get overidden
MSG_REAL_MODE db "Real Mode has begun!", 0
MSG_PROT_MODE db "We have touched down on Protected Mode!", 0
MSG_LONG_MODE db "A small step for man, a giant leap for man kind to Long Mode!", 0

; padding
times 510 - ($-$$) db 0
dw 0xaa55
