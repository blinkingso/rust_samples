//! smoltcp wire layer tester
use smoltcp::wire::pretty_print::{PrettyIndent, PrettyPrint};
use smoltcp::wire::{ArpPacket, Ipv4Address, Ipv4Cidr, Ipv4Packet, TcpPacket};
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::net::Ipv4Addr;
use std::str::FromStr;

fn main() {}
#[test]
fn test_cidr() {
    // prefix_len < 32
    let cidr = Ipv4Cidr::new(Ipv4Address::new(192, 168, 1, 243), 24);
    let netmask = cidr.netmask();
    println!("netmask: {:?}", netmask.0);
    let c = Cidr::new(Ipv4Address::new(192, 168, 1, 1), 32);
    println!("c is : {:?}", c);
    let cidr = Ipv4Cidr::from_netmask(
        Ipv4Address::new(192, 168, 1, 153),
        Ipv4Address::new(255, 255, 255, 0),
    )
    .expect("not valid ip - netmask pair");
    println!("prefix_len is : {}", cidr.prefix_len());
    println!(
        "contains ip address: 192.168.1.1 is : {}",
        cidr.contains_addr(&Ipv4Address::from(
            // "192.168.1.1".parse::<Ipv4Addr>().expect("ip parse error")
            Ipv4Addr::from_str("192.168.1.1").expect("ip parse error")
        ))
    )
}

struct Printer<'a, T: PrettyPrint> {
    prefix: &'static str,
    buffer: &'a dyn AsRef<[u8]>,
    phantom: PhantomData<T>,
}

impl<'a, T: PrettyPrint> Printer<'a, T> {
    pub fn new(prefix: &'static str, buffer: &'a dyn AsRef<[u8]>) -> Printer<'a, T> {
        Printer {
            prefix,
            buffer,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: PrettyPrint + AsRef<[u8]>> Printer<'a, T> {
    pub fn print(printable: &'a T) -> Printer<'a, T> {
        Printer {
            prefix: "",
            buffer: printable,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: PrettyPrint> std::fmt::Display for Printer<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::pretty_print(&self.buffer, f, &mut PrettyIndent::new(self.prefix))
    }
}

#[test]
fn test_pretty_print() {
    use smoltcp::wire::*;
    static PACKET_BYTES: [u8; 28] = [
        0xbf, 0x00, 0x00, 0x50, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x60, 0x35, 0x01,
        0x23, 0x01, 0xb6, 0x02, 0x01, 0x03, 0x03, 0x0c, 0x01, 0xaa, 0x00, 0x00, 0xff,
    ];
    let packet = TcpPacket::new_checked(&PACKET_BYTES).expect("packet parse error");
    let printer = Printer::<TcpPacket<&'static [u8]>>::new("", packet.into_inner());
    println!("TCP PACKET : {}", printer);
}

#[test]
fn test_arp_packet() {
    let packet = ArpPacket::new_checked(&IPV4_PACKET_BYTES).expect("invalid arp packet");
    println!("Arp packet protocol type is : {}", packet.protocol_type());
    println!("hardware type : {:?}", packet.hardware_type());
}

static IPV4_PACKET_BYTES: [u8; 30] = [
    0x45, 0x00, 0x00, 0x1e, 0x01, 0x02, 0x62, 0x03, 0x1a, 0x01, 0xd5, 0x6e, 0x11, 0x12, 0x13, 0x14,
    0x21, 0x22, 0x23, 0x24, 0xaa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
];

#[test]
fn test_ip_packet() {
    let packet = Ipv4Packet::new_checked(&IPV4_PACKET_BYTES).expect("IPV4 CHECKED FAILED");
}

#[derive(Debug)]
struct Cidr {
    address: Ipv4Address,
    prefix_len: u8,
}

impl Cidr {
    pub const fn new(address: Ipv4Address, prefix_len: u8) -> Self {
        ["prefix_len must <= 32"][(prefix_len > 32) as usize];
        Cidr {
            address,
            prefix_len,
        }
    }
}

struct Obj {
    name: String,
    lifetime: u8,
}

impl AsRef<Obj> for Obj {
    fn as_ref(&self) -> &Obj {
        &self
    }
}

impl Obj {
    pub fn to_str(&self) -> String {
        format!("name: {}, lifetime: {}", self.name, self.lifetime)
    }
}

#[test]
fn test_as_ref() {
    let obj = Obj {
        name: "pref".to_string(),
        lifetime: 10,
    };
    let obj_ref = obj.as_ref();
    println!("{}", obj_ref.to_str());
}

pub enum ManagedMap<'a, K: 'a, V: 'a> {
    ToOwned(BTreeMap<K, V>),
    /// Array reference
    Borrowed(&'a mut [Option<(K, V)>]),
}

impl<'a, K: 'a, V: 'a> From<BTreeMap<K, V>> for ManagedMap<'a, K, V> {
    fn from(owned: BTreeMap<K, V>) -> Self {
        ManagedMap::ToOwned(owned)
    }
}

impl<'a, K: 'a, V: 'a> From<&'a mut [Option<(K, V)>]> for ManagedMap<'a, K, V> {
    fn from(val: &'a mut [Option<(K, V)>]) -> Self {
        Self::Borrowed(val)
    }
}
