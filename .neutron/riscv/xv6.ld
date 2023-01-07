OUTPUT_ARCH( "riscv" )
ENTRY( _entry )

/* PROVIDE(NCPU = 2);
stack_size = 4096 * NCPU;
stack0 = [stack_size] */

/* Make stack size the rest of the memory available after .bss (growing down) */

SECTIONS
{
  /*
   * ensure that entry.S / _entry is at 0x80000000,
   * where qemu's -kernel jumps.
   */
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
    *(.srodata .srodata.*) /* do not need to distinguish this from .rodata */
    . = ALIGN(16);
    *(.rodata .rodata.*)
  }

  .data : {
    . = ALIGN(16);
    *(.sdata .sdata.*) /* do not need to distinguish this from .data */
    . = ALIGN(16);
    *(.data .data.*)
  }

  .bss : {
    . = ALIGN(16);
    *(.sbss .sbss.*) /* do not need to distinguish this from .bss */
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
