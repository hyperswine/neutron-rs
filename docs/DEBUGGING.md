# Debugging

## Clang

To debug, just do something like:

```bash
cargo barm # debug config
# serial mon:stdio for ctrl + c
qemu-system-aarch64 -M virt -nographic -serial mon:stdio -kernel build/neutron_kernel -S -s
lldb
(lldb) file build/neutron_kernel
(lldb) gdb-remote localhost:1234
```

Or

```bash
lldb -- build/neutron_kernel gdb-remote localhost:1234
```

```bash
lldb --source debug.cmd
```

## GDB

Just replace `gdb-remote localhost:1234` with `target remote localhost:1234`.

```bash
qemu-system-aarch64 -M virt -nographic -kernel build/neutron_kernel -S -s
# on another shell, or container
gdb-multiarch -q -ex 'file build/neutron_kernel' -ex 'target remote localhost:1234'
```
