bits 32 ; GRUB abandons us here, in this wasteland

section .multiboot_header
align 8
header_start:
    dd 0xE85250D6 ; Super secret backdoor for grub
    dd 0x0        ; architecture (0 = 32-bit/protected mode)
    dd header_end - header_start ; length of the forhead (we are balding)
    dd -(0xE85250D6 + 0 + (header_end - header_start))  ; checkcum (cum=0)

    ; end tag
    dd 0 ; type = 0 = end
    dd 8 ; size 8
    dq 0 ; 2 0 bytes to pad
header_end:

section .text
global _start
_start:
    cli     ; disable interrupts

    mov ebp, 0
    mov ebp, stack_top ; setup a known stack

    ; load our GDT
    lgdt [gdt_descriptor]

    ; enable PAE
    mov eax, cr4
    or eax, 1 << 5 ; set CR4.PAE = 1
    mov cr4, eax

    ; load cr3 with address of pml4 table
    lea eax, [pml4_table]
    mov cr3, eax

    ; enable long mode
    mov ecx, 0xc000080 ; IA32_EFER MSR
    rdmsr ; EDX:EAX <- MSR[IA32_EFER]
    or eax, 1 << 8 ; set EFER.LME
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31 ; CR0.PG = 1 (paging enabled)
    or eax, 1 << 0
    mov cr0, eax

    ; jamp to lung mud

    jmp 0x08:lung_mud_stort


bits 64
lung_mud_stort:
    ; Setup a new stack, we hate the 32 bit one
    mov rsp, stack_top_64

    ; Call our main C funky
    extern kmain
    call kmain

    ; If kmain returns, f*ck
.hang:
    hlt
    jmp .hang


; GDT
section .data
align 16

gdt_start:
    dq 0x0000000000000000        ; [0] Null descriptor

    ; [1] Code64:  base=0, limit=0xFFFFF, type=0xA (execute/read),
    ; L=1 → 64‑bit code, G=1 → 4KiB granularity
    ; Flags (64‑bit): 0xA09B or 0x00AF9B?
    ;   0x00AF9B → P=1, DPL=0, S=1, Type=1010 (exec/read),
    ;              G=1, L=1, D/B=0, AVL=0
    dq 0x00AF9B000000FFFF

    ; [2] Data64:  base=0, limit=0xFFFFF, type=0x2 (read/write),
    ; D/B=1 → 32‑bit / expand up, G=1 → 4KiB granularity,
    ; L=0 since data segments ignore L in long mode
    ; 0x00AF93 → P=1, DPL=0, S=1, Type=0010 (read/write),
    ;              G=1, L=0, D/B=1, AVL=0
    dq 0x00AF93000000FFFF

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1    ; size (limit)
    dq gdt_start                  ; 64‑bit base

; Page tables
section .bss
align 4096

pml4_table:
    resq 512 ; 512 * 8b = 4096b

pdpt_table:
    resq 512 ; 512 * 8b = 4096b

pd_table:
    resq 512


; Define some stacks (both for 32 and 64 bits)
section .bss
    align 16
    stack_space_32:
        resb 16*1024        ; 16 Kib for 32-bit

    stack_space_64:
        resb 16*1024        ; 16 Kib for 64-bit

stack_top equ (stack_space_32 + 16*1024)
stack_top_64 equ (stack_space_64 + 16*1024)
