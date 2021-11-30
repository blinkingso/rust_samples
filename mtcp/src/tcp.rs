//! Package TCP PACKAGE
struct EthernetHeader {
    /// source mac address
    src_mac_addr: [u8; 6],
    /// dest mac address
    dest_mac_addr: [u8; 6],
    /// protocol type
    protocol: [u8; 2],
}

struct IpHeader {
     
}