global start
extern kmain

section .text
bits 32
start:
    ; Set stack pointer
    mov esp, stack_top

    ; Save the multiboot info passed by GRUB
    mov dword [mbi_ptr], ebx
    mov dword [magic], eax  ; Save multiboot magic value

    ; Clear page tables first
    mov edi, p4_table
    xor eax, eax
    mov ecx, 4096*3  ; Size of all page tables combined
    rep stosb        ; Clear memory

    ; Point the first entry of the level 4 page table to the first entry in the p3 table
    mov eax, p3_table
    or eax, 0b11     ; Present + Writable
    mov dword [p4_table], eax

    ; Point the first entry of the level 3 page table to the first entry in the p2 table
    mov eax, p2_table
    or eax, 0b11     ; Present + Writable
    mov dword [p3_table], eax

    ; Map the first 1GB using 2MB pages
    mov ecx, 0       ; Counter variable
.map_p2_table:
    mov eax, 0x200000  ; 2MB
    mul ecx
    or eax, 0b10000011 ; Present + Writable + Huge
    mov [p2_table + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ; Move page table address to cr3 register
    mov eax, p4_table
    mov cr3, eax

    ; Enable PAE (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5   ; Set PAE flag (bit 5)
    mov cr4, eax

    ; Set long mode bit in EFER MSR
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8   ; Set LM flag (bit 8)
    wrmsr

    ; Enable paging and protection
    mov eax, cr0
    or eax, 0x80000001  ; Set PG (bit 31) and PE (bit 0)
    mov cr0, eax

    ; Load the GDT
    lgdt [gdt64.pointer]

    ; Update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Jump to 64-bit code
    jmp gdt64.code:long_mode_start

section .text
bits 64
long_mode_start:
    ; Clear all the segment registers except CS
    xor ax, ax
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Use canonical parameter passing for System V AMD64 ABI
    mov edi, dword [magic]     ; First parameter in rdi (magic number)
    mov rsi, qword [mbi_ptr]   ; Second parameter in rsi (info pointer)
    call kmain                 ; Call the kernel main function

    ; If kmain returns (it shouldn't), halt the CPU
.halt:
    hlt
    jmp .halt

section .bss
align 4096

; Page tables - 4KB each
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096

; Stack - 16KB
align 16
stack_bottom:
    resb 16384  ; Reserve 16KB for the stack
stack_top:

; Multiboot info storage
align 8
mbi_ptr:
    resq 1
magic:
    resq 1

section .rodata
align 8
gdt64:
    dq 0  ; Null descriptor
.code: equ $ - gdt64
    ; Code segment: executable, 64-bit, readable
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)  ; 43: executable, 44: descriptor type, 47: present, 53: long mode
.data: equ $ - gdt64
    ; Data segment: writable, readable
    dq (1 << 44) | (1 << 47) | (1 << 41)  ; 41: writable, 44: descriptor type, 47: present
.pointer:
    dw $ - gdt64 - 1  ; Size of the GDT - 1
    dq gdt64          ; Address of the GDT
