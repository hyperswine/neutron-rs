ENTRY(arc_entry)

SECTIONS
{
	. = 0xffff ffff 0000 0000;

    /* They should be loaded by arcboot goblin without need for specifying */
	/* .text : { *(.text) }
    .rodata: { *(.rodata) }
	.data : { *(.data) }
	.bss : { *(.bss COMMON) } */

    /* Align on 8B */
	. = ALIGN(8);
	. = . + 0x1000;
	stack_top = .;
}
