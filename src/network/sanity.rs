// test the stuff
#[cfg(test)]
fn test_network_interfaces() {
    // start, ping and shutdown the interfaces -> would be integration testing?

    // device_id => must be mounted in /dev/ and have nic_wifi as an attribute
    // let the driver for it handle the use of the following functions
    let nic_wifi = WifiDevice::new(wifidevice_id);
    // adv_options => struct with variable fields for stuff. With a on/off flag for each field as the master flag
    nic_wifi.connect_wpa2(ssid, passcode, adv_options);
    // NOTE: some things to think of are the amount of extra space (instruction + data) to pull off. Prob doesnt matter too much

    // ping 3 times. Ping returns Vec<PingResult>
    let res = nic_wifi.ping("google.com", 3);
    // see if they made it past the stuff
    res.iter().map(|r| assert!(r.success));

    // do the same for ethernet
    let nic_eth = EthDevice::new(ethdevice_id);
    // requests a connection via DHCP
    nic_eth.attempt_connect(nic::Requests::DHCP);
    let res = nic_eth.ping("google.com", 3);

    // send an IP packet
    // ipv4: 32bit unsigned value, contents: IpPacket
    nic_eth.send_ip_packet(ipv4_addr_dest, contents);

    // BT
    let nic_bt = BTDevice::new(bt_device_id);
    // bd_addr: 48bit id assigned to each device. Should never be two of the same ids apparently or maybe theres a way to change it somehow
    nic_bt.connect(bd_addr)
    // contents: BTPacket
    nic_bt.send_packet(bd_addr, contents);

}

// remember, kernel mostly deals with sockets to manage io for processes and servers, IP packets and frames, reliable data delivery through checking higher level headers and where the stuff is coming from (?)
// try not to do too much in the kernel though. So prob just managing sockets and minimal security and safety with IP and MAC addresses
// merely to communicate with the network driver and application. Let the firmware (and drivers) and software do most of the stuff
