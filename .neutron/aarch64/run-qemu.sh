QEMU=qemu-system-aarch64

cargo barm && \
$QEMU -M raspi3b \
    -serial stdio \
    -kernel build/arm
