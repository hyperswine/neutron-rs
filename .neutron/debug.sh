lldb -ex file build/neutron_kernel -ex gdb-remote localhost:1234

# NOTE: this only works for "pure builds". So for limine, it makes an iso and thus wont work I think
# Although maybe it could work once it actually boots in. But you'd have to do it manually
