//! ch-08 networking.

use std::time::Duration;
use std::{io::Write, net::TcpStream};
fn main() {}
#[test]
fn test_network() -> std::io::Result<()> {
    let domain = "www.baidu.com:443";
    let mut tcp_stream = TcpStream::connect(domain).unwrap();
    tcp_stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    tcp_stream.set_write_timeout(Some(Duration::from_secs(5)));
    tcp_stream.write(b"GET / HTTP/1.0")?;
    tcp_stream.write(b"\r\n")?;
    tcp_stream.write(b"Host: www.baidu.com")?;
    tcp_stream.write(b"\r\n\r\n")?;

    let size = std::io::copy(&mut tcp_stream, &mut std::io::stdout())?;
    println!("read data len: {}", size);
    Ok(())
}
