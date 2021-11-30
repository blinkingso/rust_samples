use rand::{thread_rng, RngCore};
/// MacAddress Struct
#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    /// new random local unicast MacAddress
    pub fn new() -> Self {
        let mut buffer = [0_u8; 6];
        thread_rng().fill_bytes(&mut buffer);
        // ensures local address bit set to 1, and unicast bit set to 0;
        buffer[0] |= 0b_0000_0000;
        buffer[0] &= 0b_1111_1110;
        MacAddress(buffer)
    }

    /// Create a Mac Address with specified Address
    pub fn new_with_buf(buf: [u8; 6]) -> Self {
        MacAddress(buf)
    }
}

/// Custom MacAddress to
/// ```
/// let mac_address = smoltcp::wire::EthernetAddress([192_u8, 10, 10, 20, 88, 182]);
/// println!("got mac address is : {:?}", &mac_address);
/// ```
///
impl Into<smoltcp::wire::EthernetAddress> for MacAddress {
    fn into(self) -> smoltcp::wire::EthernetAddress {
        smoltcp::wire::EthernetAddress(self.0)
    }
}

impl std::fmt::Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer = self.0;
        // format with 2bits hex string
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            &buffer[0], &buffer[1], &buffer[2], &buffer[3], &buffer[4], &buffer[5]
        )
    }
}
