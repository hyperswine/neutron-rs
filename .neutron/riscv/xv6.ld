OUTPUT_ARCH("riscv")
ENTRY(_start)

SECTIONS
{
  /* ensure _start gets loaded at 0x80000000 */
  . = 0x80000000;

  .text : {
    *(.text .text.*)
    . = ALIGN(0x1000);
    _trampoline = .;
    *(trampsec)
    . = ALIGN(0x1000);
    /* ASSERT(. - _trampoline == 0x1000, "error: trampoline larger than one page"); */
    PROVIDE(etext = .);
  }

  .rodata : {
    . = ALIGN(16);
    *(.srodata .srodata.*)
    . = ALIGN(16);
    *(.rodata .rodata.*)
  }

  .data : {
    . = ALIGN(16);
    *(.sdata .sdata.*)
    . = ALIGN(16);
    *(.data .data.*)
  }

  .bss : {
    . = ALIGN(16);
    *(.sbss .sbss.*)
    . = ALIGN(16);
    *(.bss .bss.*)
    _end = .;
  }

  .stack (NOLOAD) : {
    . = ALIGN(16);
    _stack = .;
    *(.stack)
    > end
  }

  .heap (NOLOAD) : {
    . = ALIGN(16);
    _heap = .;
    *(.heap)
    > stack
  }

  PROVIDE(end = _end);
  PROVIDE(stack = _stack);
  PROVIDE(heap = _heap);
}
