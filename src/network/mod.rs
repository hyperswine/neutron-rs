pub struct IpPacket {
    content: String,
}

pub impl IpPacket {
    // automatically pad the stuff, apply crypto algorithms with a crypto chip (on SoC if there is one? maybe let the driver do that)
    // simply append the data to the default string
    fn new() -> IpPacket {
        IpPacket{content: String::from("
        4<IHL><TOS><LENGTH_TOTAL><ID><FLAGS><FRAGMENT_OFFSET><TTL><PROTOCOL><CHECKSUM_HEADER><SRC_IPV4><DST_IPV4><OPTIONS>
        ")}
    }

    fn add_data(data: String) {
        // split up the packet into multiple packets if necessary
    }
}

pub struct NetworkManager {
    n_processes_using_network: u32,
    manager_status: NetworkManagerStatus,

    // NIC, assume only a single instance of each for now
    ethernet_status: bool,
    wifi_status: bool,
    bt_status: bool,
}

// maybe a single manager status for every manager, place in lib.rs
enum NetworkManagerStatus {
    UP,
    DOWN,
}

struct WifiCard {
    _type: WifiType,
}

struct EthernetCard {
    _type: EthernetType,
}

struct BluetoothCard {
    _type: BluetoothType,
}

enum WifiType {
    WIFI5,
    WIFI6,
    WIFI6E,
}

enum EthernetType {
    GIGABIT100,
    GIGABIT1000,
}

enum BluetoothType {
    BT5,
    BT51,
    BT52,
}

// ----------------
// INTERFACE
// ----------------

struct Socket {
    socket_addr: Addr,
}

struct Addr {
    addr: u64,
}
