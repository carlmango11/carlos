global long_mode_start

extern main
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

    call remap_pic

    call idt_install
    call main