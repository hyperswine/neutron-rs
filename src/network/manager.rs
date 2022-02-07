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
