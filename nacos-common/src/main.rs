// use crate::grpc_client::request::{ConnectionSetupRequest, Request};
// use crate::grpc_client::{GrpcClient, ServerInfo, ServerRequestHandler};
use nacos_grpc_service::*;
use protobuf::well_known_types::Any;
use protobuf::{Message, SingularPtrField};
use serde_json::json;
use std::collections::HashMap;
use std::env::set_var;
use std::error::Error;
use std::io::ErrorKind;
use std::time::Duration;
use tokio::io::Interest;
use tokio::net::TcpStream;
use tonic::client::Grpc;
use tonic::{IntoRequest, IntoStreamingRequest};

// mod grpc_client;
mod proto;
mod nacos_grpc_service {
    include!("proto/nacos/_.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "debug");
    env_logger::init();
    // let mut client = GrpcClient::connect_to_server(ServerInfo {
    //     ip: "127.0.0.1".parse().unwrap(),
    //     port: 9848,
    // })
    // .await?;
    // let request = Request::new(HashMap::new(), "1230123");
    // let connection_setup_request =
    //     ConnectionSetupRequest::new(request, "2.0.1", "", Default::default());
    // client.send_request(connection_setup_request.clone()).await;
    // tokio::time::sleep(Duration::from_secs(1));

    let mut client = request_client::RequestClient::connect("http://127.0.0.1:9848").await?;
    let mut headers = HashMap::new();
    headers.insert("accessToken".to_string(), "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJuYWNvcyIsImV4cCI6MTY0MjE3Mzc2Nn0.xQccuBO50TTW2SXjdiGrBmtVw_fKu4c1NDr0-UBmuW8".to_string());
    let metadata = Metadata {
        r#type: "ConnectionSetupRequest".to_string(),
        client_ip: "127.0.0.1".to_string(),
        headers,
    };
    let request = tonic::Request::new(Payload {
        metadata: Some(metadata),
        body: Some(prost_types::Any {
            type_url: "".to_string(),
            value: json!({
                "client_version": "2.0.1".to_string(),
                "tenant": "".to_string(),
                "labels": {},
                "headers": {},
            })
            .to_string()
            .as_bytes()
            .to_vec(),
        }),
    });
    let response = client.request(request).await?;
    log::info!("response is : {:?}", response);

    let _ = tokio::signal::ctrl_c().await;
    Ok(())
}
