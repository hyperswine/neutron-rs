ENTRY(_start)

SECTIONS
{
	. = 0x40000000;
	.text : { *(.text) }
	.data : { *(.data) }
	.bss : { *(.bss COMMON) }
	. = ALIGN(8);
	. = . + 0x1000; /* 4kB of stack memory */
	/* make sure to do asm!("ldr x30, =stack_top\n mov sp x30") */
	stack_top = .;
}
