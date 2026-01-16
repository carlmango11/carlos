OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

SECTIONS {
    . = 1M;

    .multiboot : {
        *(.multiboot_header*)
    }

    .text : {
        *(.text*)
    }

    .bss : {
        *(.bss*)
        *(COMMON)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    # needs to be 4k aligned for my page tables
    . = ALIGN(4K);
    PROVIDE(heap_start  = .);
    PROVIDE(heap_end  = . + 32M);

    kernel_end = .;
}
