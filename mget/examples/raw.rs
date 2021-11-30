use smoltcp::phy::ChecksumCapabilities;
use smoltcp::socket::{RawSocket, RawSocketBuffer};
use smoltcp::wire::{
    IpAddress, IpProtocol, IpVersion, Ipv4Address, Ipv4Packet, Ipv4Repr, TcpControl, TcpPacket,
    TcpRepr, TcpSeqNumber,
};

fn main() {
    let mut raw_socket = RawSocket::new(
        IpVersion::Ipv4,
        IpProtocol::Tcp,
        RawSocketBuffer::new(vec![], vec![]),
        RawSocketBuffer::new(vec![], vec![]),
    );
    let http_header = "GET / HTTP/1.1\r\nHost: baidu.com\r\nConnection: close\r\n\r\n";
    let http_header = http_header.as_bytes();
    let repr = Ipv4Repr {
        src_addr: Ipv4Address::new(192, 168, 1, 243),
        dst_addr: Ipv4Address::new(104, 193, 88, 123),
        protocol: IpProtocol::Tcp,
        hop_limit: 64,
        payload_len: http_header.len(),
    };

    let mut buffer = Vec::from(http_header);
    let tcp_repr = TcpRepr {
        src_port: 60611,
        dst_port: 80,
        control: TcpControl::Syn,
        seq_number: TcpSeqNumber::default(),
        ack_number: None,
        window_len: 0,
        window_scale: None,
        max_seg_size: None,
        sack_permitted: false,
        sack_ranges: [None; 3],
        payload: &buffer,
    };
    let mut buf = vec![0; 1504];
    let mut pck = TcpPacket::new_unchecked(&mut buf);
    // tcp header
    let check_sum_cap = &ChecksumCapabilities::default();
    let src = Ipv4Address::new(192, 168, 1, 243);
    let dst = Ipv4Address::new(104, 193, 88, 123);
    tcp_repr.emit(
        &mut pck,
        &IpAddress::Ipv4(src),
        &IpAddress::Ipv4(dst),
        check_sum_cap,
    );
    println!("buf is : {:?}", pck.as_ref());
    let _ = raw_socket
        .send_slice(&buf[..])
        .expect("raw socket send error");
}
