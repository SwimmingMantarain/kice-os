global start
extern kmain

section .text
bits 32
start:
    ; Set stack pointer
    mov esp, stack_top

    ; save the multiboot info passed by grub
    mov dword [mbi_ptr], ebx
    mov dword [magic], eax  ; Save multiboot magic value

    ; Point the first entry of the level 4 page table to the first entry in the
    ; p3 table
    mov eax, p3_table
    or eax, 0b11
    mov dword [p4_table + 0], eax

    ; Point the first entry of the level 3 page table to the first entry in the
    ; p2 table
    mov eax, p2_table
    or eax, 0b11
    mov dword [p3_table + 0], eax

    ; point each page table level two entry to a page
    mov ecx, 0         ; counter variable
.map_p2_table:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ; move page table address to cr3 register
    mov eax, p4_table
    mov cr3, eax

    ; enable physical address extension
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set long mode bit
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 0x80000001
    mov cr0, eax

    ; load the GDT
    lgdt [gdt64.pointer]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax

    ; jump, actually no: LEAP gracefully and magestically to kmane!
    jmp supa_jmp

supa_jmp:
    jmp gdt64.code:long_mode_start

section .text
bits 64
long_mode_start:
    ; Pass parameters according to System V AMD64 ABI
    mov edi, [magic]      ; First parameter in rdi (magic numba)
    mov rsi, [mbi_ptr]    ; Second parameter in rsi (info pointer)
    jmp kmain

section .bss

align 4096

p4_table:
    resb 4096

p3_table:
    resb 4096

p2_table:
    resb 4096

align 16
stack_bottom:
    resb 4096 ; Reserve 4KB for the stack
align 16
stack_top:

mbi_ptr:
    resq 1
magic:
    resq 1

section .rodata
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1 << 44) | (1 << 47) | (1 << 41) | (1 << 43) | (1 << 53)
.data: equ $ - gdt64
    dq (1 << 44) | (1 << 47) | (1 << 41)
.pointer:
    dw .pointer - gdt64 - 1
    dq gdt64
