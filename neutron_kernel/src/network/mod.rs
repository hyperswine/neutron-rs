pub mod manager;

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

// ----------------
// INTERFACE
// ----------------

struct Socket {
    socket_addr: Addr,
}

struct Addr {
    addr: u64,
}
