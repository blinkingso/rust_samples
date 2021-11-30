//! EthernetInterface
use smoltcp::iface::{EthernetInterface, EthernetInterfaceBuilder, NeighborCache, Routes};
use smoltcp::phy::{Device, RawSocket as BpfDevice};
use smoltcp::socket::{RawPacketMetadata, RawSocket, RawSocketBuffer, SocketSet};
use smoltcp::storage::{PacketBuffer, PacketMetadata};
use smoltcp::time::Instant;
use smoltcp::wire::{
    EthernetAddress, IpAddress, IpCidr, IpProtocol, IpVersion, Ipv4Address, Ipv4Cidr,
};
use std::collections::BTreeMap;
use std::net::Ipv4Addr;
use std::os::unix::prelude::AsRawFd;
use std::str::FromStr;

fn main() {}

static MAC_ADDRESS: [u8; 6] = [0x00, 0xe0, 0x4c, 0x68, 0x01, 0x04];

#[test]
fn test_iface() {
    let cidr1 = Ipv4Cidr::new(
        Ipv4Address::from(Ipv4Addr::from_str("192.168.1.53").expect("ip addr invalid")),
        24,
    );
    let cidr2 = Ipv4Cidr::new(
        Ipv4Address::from(Ipv4Addr::from_str("192.168.1.233").expect("ip addr invalid")),
        24,
    );

    let ip_addrs = vec![IpCidr::Ipv4(cidr1), IpCidr::Ipv4(cidr2)];
    let device = BpfDevice::new("en4").expect("no `en4` device found");
    let fd = device.as_raw_fd();
    let mut routes = Routes::new(BTreeMap::new());
    // routes.add_default_ipv4_route(Ipv4Address::new(192, 168, 1, 1));
    let mut ei = EthernetInterfaceBuilder::new(device)
        .routes(Routes::new(BTreeMap::new()))
        .neighbor_cache(NeighborCache::new(BTreeMap::new()))
        // .ip_addrs(ip_addrs)
        .any_ip(true)
        // routes
        .routes(routes)
        .ethernet_addr(EthernetAddress::from_bytes(&MAC_ADDRESS))
        .finalize();
    println!(
        "fd: {}, device capabilities: {:?}",
        fd,
        ei.device().capabilities()
    );
    let mut sockets = SocketSet::new(vec![]);
    let mut rx_buffer = RawSocketBuffer::new(vec![PacketMetadata::EMPTY; 4], vec![0x00_u8; 1514]);
    let mut tx_buffer = RawSocketBuffer::new(vec![PacketMetadata::EMPTY; 4], vec![0x00_u8; 1514]);
    let raw_socket = RawSocket::new(IpVersion::Ipv4, IpProtocol::Tcp, rx_buffer, tx_buffer);
    let tcp_handle = sockets.add(raw_socket);
    let mut timestamp = Instant::now();
    loop {
        match ei.poll(&mut sockets, timestamp) {
            Ok(process) => {
                println!("process or receive: {}", process);
            }
            Err(e) => {
                eprintln!("poll error: {}", e);
            }
        }
    }
}
