# syscall API for 64bit Neutron
# when linking, make sure not to use `write, read, open, close` in your own code or ensure they're mangled to something different

# reg vals in a1, a2, a3 should be already loaded by the caller
# with C FFI. ! Actually dunno if I need to specify it again

open:
    li a0, 0
    syscall

close:
    li a1, 1
    syscall

read:
    li a0, 2
    syscall

write:
    li a0, 3
    syscall
