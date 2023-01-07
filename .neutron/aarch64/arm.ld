ENTRY(_start)

SECTIONS {
    . = 0x40000000;
    .text : { *(.text) }
    .data : { *(.data) }
    __bss_start = .;
    .bss : { *(.bss COMMON) }
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    . = ALIGN(8);
    . += 0x1000;
    stack_top = .;
}