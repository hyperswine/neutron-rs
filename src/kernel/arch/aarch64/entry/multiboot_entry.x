ENTRY(_multiboot_entry)

SECTIONS
{
	.startup . : { multiboot.o(.text) }
	/* should start at 1MB for multiboot */
	/* 0x100000 = 1MiB */
	. = 0x100000;

    /* .multiboot : { *(.multiboot_header) } */
	.text : { *(.text) }
	.data : { *(.data) }
	.bss : { *(.bss COMMON) }

	. = ALIGN(8);
	. = . + 0x1000; /* 4kB of stack memory */
	stack_top = .;
}
