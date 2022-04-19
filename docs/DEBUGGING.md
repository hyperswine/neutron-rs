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

## GDB

Just replace `gdb-remote localhost:1234` with `target remote localhost:1234`.
