section .multiboot_header
align 8
dd 0xe85250d6              ; magic number (Multiboot2)
dd 0                       ; architecture (0 = i386)
dd header_end - header_start ; header length
dd -(0xe85250d6 + 0 + (header_end - header_start)) ; checksum

header_start:
; end tag
dw 0
dw 0
dd 8
header_end:
