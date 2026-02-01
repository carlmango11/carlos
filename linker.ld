OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

KERNEL_VMA = 0xffffffff80000000;
KERNEL_LMA = 0x00100000;

SECTIONS {
    . = KERNEL_LMA;

    .multiboot : {
        *(.multiboot_header*)
    }

    . = KERNEL_VMA;

    .text ALIGN(4K): AT (ADDR(.text) - KERNEL_VMA) {
        *(.text*)
    }

    .rodata ALIGN(4K): AT (ADDR(.rodata) - KERNEL_VMA) {
        *(.rodata*)
    }

    .data ALIGN(4K): AT (ADDR(.data) - KERNEL_VMA) {
        *(.data*)
    }

    .bss ALIGN(4K): AT (ADDR(.bss) - KERNEL_VMA) {
        *(.bss*)
        *(COMMON)
    }

    # needs to be 4k aligned for my page tables
    . = ALIGN(4K);
    PROVIDE(heap_start  = .);
    PROVIDE(heap_end  = . + 32M);

    kernel_start = KERNEL_VMA;
    kernel_end = .;
}
