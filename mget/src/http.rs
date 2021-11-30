use std::collections::BTreeMap;
use std::os::unix::prelude::AsRawFd;
use std::str::FromStr;
use std::vec;

use smoltcp::iface::{EthernetInterfaceBuilder, NeighborCache, Routes};
use smoltcp::phy::{EthernetTracer, RawSocket as RawSocketDevice};
use smoltcp::socket::{RawSocket, RawSocketBuffer, SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpEndpoint, IpProtocol, IpVersion, Ipv4Address};
use std::borrow::Borrow;
use url::Url;

/// Http get Request through specified network device;
pub fn get(
    network_device: &str,
    mac_address: EthernetAddress,
    addr: String,
    url: Url,
) -> Result<(), UpstreamError> {
    let domain_name = url.host_str().ok_or(UpstreamError::InvalidUrl)?;

    let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);
    // let raw_rx_buffer = RawSocketBuffer::new(vec![], vec![]);
    // let raw_tx_buffer = RawSocketBuffer::new(vec![], vec![]);
    // let raw_socket = RawSocket::new(
    //     IpVersion::Ipv4,
    //     IpProtocol::Tcp,
    //     raw_rx_buffer,
    //     raw_tx_buffer,
    // );
    let mut sockets = SocketSet::new(vec![]);
    let tcp_handle = sockets.add(tcp_socket);
    let http_header = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept: */*\r\n\r\n",
        url.path(),
        domain_name
    );

    println!("header is : \r\n{}, ip is : {}", &http_header, &addr);

    let neighbor_cache = NeighborCache::new(BTreeMap::new());
    // local device
    let ip_addrs = [smoltcp::wire::IpCidr::new(
        smoltcp::wire::IpAddress::v4(192, 168, 1, 174),
        24,
    )];
    let mut routes = Routes::new(BTreeMap::new());
    routes.add_default_ipv4_route(Ipv4Address::new(192, 168, 1, 1))?;
    let device = RawSocketDevice::new(network_device)?;
    let fd = device.as_raw_fd();
    println!("fd: {}", &fd);
    let device = EthernetTracer::new(device, |_timestamp, _printer| {
        println!("{}", _printer);
    });

    let mut iface = EthernetInterfaceBuilder::new(device)
        .ethernet_addr(mac_address)
        .ip_addrs(ip_addrs)
        .neighbor_cache(neighbor_cache)
        .routes(routes)
        .any_ip(false)
        .finalize();
    let mut state = HttpState::Connect;
    'http: loop {
        let timestamp = Instant::now();
        match iface.poll(&mut sockets, timestamp) {
            Ok(_) => {}
            Err(smoltcp::Error::Unrecognized) => {}
            Err(e) => {
                eprintln!("error: {:?}", e);
            }
        }

        {
            let mut socket = sockets.get::<TcpSocket>(tcp_handle);
            state = match state {
                HttpState::Connect if !socket.is_active() => {
                    eprintln!("connecting");
                    socket.connect(
                        IpEndpoint::new(
                            // Ipv4Address::from_str("&addr")?,
                            IpAddress::Ipv4(std::net::Ipv4Addr::from_str(&addr)?.into()),
                            80,
                        ),
                        random_port(),
                    )?;
                    HttpState::Request
                }
                HttpState::Request if socket.may_send() => {
                    eprintln!("sending request");
                    socket.send_slice(http_header.as_ref())?;
                    HttpState::Response
                }
                HttpState::Response if socket.can_recv() => {
                    socket.recv(|raw_data| {
                        let output = String::from_utf8_lossy(raw_data);
                        println!("{}", output);
                        (raw_data.len(), ())
                    })?;
                    HttpState::Response
                }
                HttpState::Response if !socket.may_recv() => {
                    eprintln!("received complete response");
                    break 'http;
                }
                _ => state,
            }
        }

        smoltcp::phy::wait(fd, iface.poll_delay(&sockets, timestamp)).expect("wait error");
    }

    Ok(())
}

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    InvalidUrl,
    NetWork(smoltcp::Error),
    Content(std::str::Utf8Error),
    Address(std::net::AddrParseError),
    IO(std::io::Error),
}

impl std::fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::Error> for UpstreamError {
    fn from(err: smoltcp::Error) -> Self {
        UpstreamError::NetWork(err)
    }
}

impl From<std::net::AddrParseError> for UpstreamError {
    fn from(err: std::net::AddrParseError) -> Self {
        UpstreamError::Address(err)
    }
}

impl From<std::str::Utf8Error> for UpstreamError {
    fn from(err: std::str::Utf8Error) -> Self {
        UpstreamError::Content(err)
    }
}

impl From<std::io::Error> for UpstreamError {
    fn from(err: std::io::Error) -> Self {
        eprintln!("io error occured: {:?}", err);
        UpstreamError::IO(err)
    }
}

impl std::error::Error for UpstreamError {}

/// random a client connection port
fn random_port() -> u16 {
    49152 + rand::random::<u16>() % 16384
}
