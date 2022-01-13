use crate::grpc_client::request::{ConnectionSetupRequest, Request};
use crate::grpc_client::{convert, GrpcClient, ServerInfo};
use crate::proto::nacos_grpc_service::{Metadata, Payload};
use protobuf::well_known_types::Any;
use protobuf::{Message, SingularPtrField};
use std::collections::HashMap;
use std::env::set_var;
use std::error::Error;
use std::time::Duration;
use tokio::io::Interest;
use tokio::net::TcpStream;

mod grpc_client;
mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "debug");
    env_logger::init();
    let mut client = GrpcClient::connect_to_server(ServerInfo {
        ip: "127.0.0.1".parse().unwrap(),
        port: 9848,
    })
    .await?;
    let request = Request::new(HashMap::new(), "1230123");
    let connection_setup_request =
        ConnectionSetupRequest::new(request, "2.0.1", "", Default::default());
    client.send_request(connection_setup_request).await;
    Ok(())
}
