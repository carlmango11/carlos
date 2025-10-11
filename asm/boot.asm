GLOBAL _start

extern _main
;extern idt_install
extern testy2
extern long_mode_start
;extern isr_handler
;extern remap_pic

SECTION .text
bits 32
_start:
    mov esp, stack_top

    call set_up_page_tables
    call enable_paging

    lgdt [gdt64.pointer]
    jmp gdt64.code_segment:long_mode_start

    hlt

set_up_page_tables:
    mov eax, page_table_l3
    or eax, 0b11 ; present + writable flags
    mov [page_table_l4], eax

    mov eax, page_table_l2
    or eax, 0b11 ; present + writable flags
    mov [page_table_l3], eax

    mov ecx, 0 ; counter
.loop:
    mov  eax, 0x200000; 2MB
    mul ecx
    or eax, 0b10000011 ; present + writable + huge page
    mov [page_table_l2 + ecx * 8], eax

    inc ecx ; inc
    cmp ecx, 512 ; check
    jne .loop

    ret

enable_paging:
    mov eax, page_table_l4
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
    or eax, 1 << 31 ; long mode flagt
    mov cr0, eax

    ret

section .rodata
gdt64:
    dq 0; zero entry
.code_segment: equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53) ; code segment: exec flag, descriptor=code+data, present flag, 64bit flag
.pointer:
    dw $ - gdt64 - 1 ; current adddress minus start of this section - 1 (i.e. length - 1)
    dq gdt64

;global load_idt
;load_idt:
;    mov eax, [esp+4]   ; address of idtp
;    lidt [eax]
;    ret

global isr1
isr1:
    cli
    push byte 0        ; error code placeholder
    push byte 1        ; interrupt number
    jmp isr_common_stub

extern isr_handler

isr_common_stub:
;    pusha               ; save all registers
;    push ds
;    push es
;    push fs
;    push gs
;
;    mov ax, 0x10        ; kernel data segment
;    mov ds, ax
;    mov es, ax
;    mov fs, ax
;    mov gs, ax

;    push esp            ; pass pointer to stack frame
    call isr_handler    ; C function
;    add esp, 4
;
;    pop gs
;    pop fs
;    pop es
;    pop ds
;    popa
;    add esp, 8          ; remove int number + err code
;    sti
;    iret

section .bss
align 4096
page_table_l4:
    resb 4096
page_table_l3:
    resb 4096
page_table_l2:
    resb 4096
stack_bottom:
    resb 1024 * 16 ; 16kb
stack_top: