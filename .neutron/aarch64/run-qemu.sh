QEMU=qemu-system-aarch64

cargo barm --release && \
$QEMU -machine virt -cpu cortex-a57 -kernel build/arm -serial stdio -display none
