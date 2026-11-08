/* Linker script for QEMU AArch64 'virt' machine (RAM starts at 0x4000_0000) */
ENTRY(_start)

SECTIONS
{
    /* QEMU virt loads the kernel near the start of RAM at 0x40080000 */
    . = 0x40080000;

    .text : {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    .rodata : ALIGN(4096) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4096) {
        *(.data .data.*)
    }

    .bss (NOLOAD) : ALIGN(4096) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(8);
        __bss_end = .;
    }

    /* 64 KiB kernel boot stack (grows downwards) */
    . = ALIGN(4096);
    __stack_bottom = .;
    . += 0x10000;
    __stack_top = .;

    /DISCARD/ : {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.eh_frame*)
    }
}
