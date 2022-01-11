use crate::proto::nacos_grpc_service::{Metadata, Payload};
use protobuf::well_known_types::Any;
use protobuf::{Message, SingularPtrField};
use std::time::Duration;
use tokio::io::Interest;
use tokio::net::TcpStream;

mod grpc_client;
mod proto;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:9848").await?;
    loop {
        let ready = stream
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await?;
        if ready.is_readable() {
            let mut data = vec![0; 1024];
            match stream.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                }
                Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        if ready.is_writable() {
            let mut metadata = Metadata::new();
            metadata.set_clientIp("192.168.1.184".to_string());
            metadata.set_field_type(String::from("ServerCheckRequest"));
            let mut payload = Payload::new();
            payload.metadata = SingularPtrField::some(metadata);
            let mut body = Any::new();
            body.value = br#"{"headers":{},"module":"internal"}"#.to_vec();
            match stream.try_write(body.write_to_bytes().unwrap().as_slice()) {
                Ok(n) => {
                    println!("write success: {} bytes", n);
                }
                Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => continue,
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        std::thread::sleep(Duration::from_millis(1000));
    }
}
