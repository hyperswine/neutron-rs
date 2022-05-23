INCLUDE kernel_virt_addr_space_size.ld;

PAGE_SIZE = 4K;
PAGE_MASK = PAGE_SIZE - 1;

__kernel_virt_start_addr = ((0xffffffffffffffff - __kernel_virt_addr_space_size) + 1);

__rpi_phys_dram_start_addr = 0;
/* Identity mapped */
__rpi_phys_binary_load_addr = 0x80000;

ENTRY(__rpi_phys_binary_load_addr)

PHDRS
{
    segment_code            PT_LOAD FLAGS(5);
    segment_data            PT_LOAD FLAGS(6);
    segment_boot_core_stack PT_LOAD FLAGS(6);
}

SECTIONS
{
    . =  __kernel_virt_start_addr;

    __code_start = .;
    .text : AT(__rpi_phys_binary_load_addr)
    {
        KEEP(*(.text._start))
        *(.text._start_arguments)
        *(.text._start_rust)
        *(.text*)
    } :segment_code

    .rodata : ALIGN(8) { *(.rodata*) } :segment_code
    .got    : ALIGN(8) { *(.got)     } :segment_code

    . = ALIGN(PAGE_SIZE);
    __code_end_exclusive = .;
    
    __data_start = .;
    .data : { *(.data*) } :segment_data

    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } :segment_data

    . = ALIGN(PAGE_SIZE);
    __data_end_exclusive = .;

    __mmio_remap_start = .;
    . += 8 * 1024 * 1024;
    __mmio_remap_end_exclusive = .;

    . += PAGE_SIZE;

    .boot_core_stack (NOLOAD) : AT(__rpi_phys_dram_start_addr)
    {
        __boot_core_stack_start = .;
        . += __rpi_phys_binary_load_addr;
        __boot_core_stack_end_exclusive = .;
    } :segment_boot_core_stack
}
