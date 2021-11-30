//! physical device abstraction
use smoltcp::phy::{Checksum, ChecksumCapabilities, DeviceCapabilities};

fn main() {}

#[test]
#[cfg(feature = "proto-ipv4")]
fn test_device_cap() {
    let ethernet_cap = DeviceCapabilities {
        max_transmission_unit: 1520,
        ..Default::default()
    };

    println!("default cap: {:?}", ethernet_cap);
}
