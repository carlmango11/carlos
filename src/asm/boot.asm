GLOBAL _start

extern _main
extern long_mode_trampoline

SECTION .boot_text
bits 32
_start:
    cmp eax, 0x36D76289
    jne failed_multiboot

    mov [multiboot_info], ebx
    lea ecx, [multiboot_info]

    mov esp, stack_top

    call set_up_page_tables
    call enable_paging

    lgdt [gdt64.pointer]
    jmp gdt64.code_segment:long_mode_trampoline

    hlt

failed_multiboot:
    hlt

set_up_page_tables:
    mov eax, kernel_l3
    or eax, 0b11 ; present + writable flags
    mov [kernel_l4], eax

    mov eax, kernel_l2
    or eax, 0b11 ; present + writable flags
    mov [kernel_l3], eax

    mov ecx, 0 ; counter
.loop:
    mov eax, 0x200000; 2MB
    mul ecx
    or eax, 0b10000011 ; present + writable + huge page
    mov [kernel_l2 + ecx * 8], eax

    inc ecx ; inc
    cmp ecx, 512 ; check
    jne .loop

    ret

enable_paging:
    mov eax, kernel_l4
    mov cr3, eax

    ; enable PAE
    mov eax, cr4
    or eax, 1 <<5 ; set PAE flag
    mov cr4, eax

    ; enable long mode
    mov ecx, 0xC0000080 ; magic val
    rdmsr
    or eax, 1 << 8 ; long mode flag
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31 ; long mode flag
    mov cr0, eax

    ret

section .boot_rodata
gdt64:
    dq 0; zero entry
.code_segment: equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53) ; code segment: exec flag, descriptor=code+data, present flag, 64bit flag
.pointer:
    dw $ - gdt64 - 1 ; current address minus start of this section - 1 (i.e. length - 1)
    dq gdt64

bits 64
global load_idt

extern keyboard_irt_handler
global isr1

extern page_fault_handler
global page_fault_routine

load_idt:
    lidt [rdi]
    ret

isr1:
    push rax
    push rcx
    push rdx
    push rbx
    push rbp
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    call keyboard_irt_handler

    ; Send End Of Interrupt to PIC
    mov al, 0x20
    out 0x20, al

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rbp
    pop rbx
    pop rdx
    pop rcx
    pop rax

    iretq

page_fault_routine:
    push rax
    push rcx
    push rdx
    push rbx
    push rbp
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    mov rdi, cr2
    call page_fault_handler

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rbp
    pop rbx
    pop rdx
    pop rcx
    pop rax

    iretq

global kernel_l4
global kernel_l3

section .boot_bss
align 4096
kernel_l4:
    resb 4096
kernel_l3:
    resb 4096
kernel_l2:
    resb 4096
stack_bottom:
    resb 1024 * 16 ; 16kb
stack_top:
align 8
multiboot_info:
    resq 1

global multiboot_info

;global user_l4
;global user_l3
;global user_l2
;global user_l1
;
;user_l4:
;    resb 4096
;user_l3:
;    resb 4096
;user_l2:
;    resb 4096
;user_l1:
;    resb 4096
