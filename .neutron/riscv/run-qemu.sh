QEMU=qemu-system-riscv64

cargo brv && \
$QEMU -M virt -smp 4 -m 2G \
    -display none -serial stdio \
    -bios build/neutron_main
