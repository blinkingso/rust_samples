//! TCP
extern crate etherparse;
extern crate tun_tap;
use etherparse::SlicedPacket;
use tun_tap::{Iface, Mode};
mod tcp;
fn main() {
    let iface = Iface::new("tun0", Mode::Tun).expect("failed to create new a tun-tap device");
    let name = iface.name();
    println!("tun is : {}", name);
    let mut buf = vec![0; 1504]; // MTU + 4 for the header
    loop {
        let r_bytes = iface.recv(&mut buf).unwrap();
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x800 {
            continue;
        }

        match SlicedPacket::from_ethernet(&buf) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                println!("link: {:?}", value.link);
                println!("vlan: {:?}", value.vlan);
                println!("ip: {:?}", value.ip);
                println!("transport: {:?}", value.transport);
            }
        }

        eprintln!(
            "read {} bytes (flags: {:x}, proto: {:x}): {:x?}",
            r_bytes - 4,
            flags,
            proto,
            &buf[4..r_bytes]
        );
    }
}
