# ASSUME YOU HAVE BUILT LIMINE and MADE AN ISO_ROOT DIR
# git clone https://github.com/limine-bootloader/limine.git --branch=v3.0-branch-binary --depth=1
# make -C limine

set -x

cargo bx86l

# ENSURE THESE THINGS HAVE BEEN COPIED
# cp limine.cfg limine/limine.sys limine/limine-cd.bin limine/limine-cd-efi.bin iso_root/

cp build/neutron_main build/iso_root

cd build && \
xorriso -as mkisofs -b limine-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot limine-cd-efi.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o neutron-x86_64-limine.iso && \
limine/limine-deploy neutron-x86_64-limine.iso && \
qemu-system-x86_64 -cdrom neutron-x86_64-limine.iso --no-reboot -d int -D qemu.log
