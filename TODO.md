# TODO

- everything else is meh
- rn, i have no idea why this is happening:

```asm
40001048 e8 03 1f aa     mov        x8,xzr
4000104c 00 00 00 00     udf        0x0
```

Apparently this means [undefined instruction](https://en-support.renesas.com/knowledgeBase/16980260):

```c++
UNRECOVERED_JUMPTABLE = (code *)UndefinedInstructionException(0,0x4000104c);
(*UNRECOVERED_JUMPTABLE)();
```

Can mean:

- Various noise
- Data corruption
- Device destruction
- **Stack not set**
- **Stack overflow**
- A target device does not correspond to the endian setting of debugger
- Jump to the wrong destination
- Flash memory access wait is not set

ALSO:

```c++
halt_baddata();
```
