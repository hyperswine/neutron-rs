#[cfg(test)]
fn test_service() {
    // test reading from and to disk (virtual)
    let service = ServiceManager;
    // create a virtual harddisk and mount, auto mounted at /mnt/vhdi, read writable by default user
    let vhd = tools::create_mount_vhd(disk_number, n_bytes, services::READ_WRITE);
    // ensure the drivers for virtual disk are working
    // maybe dont need since we can just treat it as a direct filesystem? rather than a device filesystem

    // blocking IO => have to implement async?
    let result = services::read_service(disk_number, offset, bytes);

    println!("result = {}", result);
}