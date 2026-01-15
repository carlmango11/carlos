global long_mode_start
global disable_interrupts
global halt

extern main_rust
extern remap_pic
extern idt_install

section .text
bits 64
long_mode_start:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Remap PIC and install IDT entries
    call remap_pic
    call idt_install

    ; Small delay before enabling interrupts
    mov ecx, 1000000
.delay:
    dec ecx
    jnz .delay

    sti ; Enable interrupts globally now that IDT & PIC are set

    call main_rust

    hlt

disable_interrupts:
    cli
    ret

halt:
    hlt