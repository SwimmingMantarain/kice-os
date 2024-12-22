global timer_interrupt_stub

extern timer_handler

section .text
timer_interrupt_stub:
    push rbx                   ; Save rbx
    push r12                   ; Save r12
    push r13                   ; Save r13
    push r14                   ; Save r14
    push r15                   ; Save r15

    call timer_handler         ; Call the Rust handler

    pop r15                    ; Restore r15
    pop r14                    ; Restore r14
    pop r13                    ; Restore r13
    pop r12                    ; Restore r12
    pop rbx                    ; Restore rbx

    iret                       ; Return from interrupt
