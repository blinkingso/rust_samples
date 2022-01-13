use crate::grpc_client::request::Request;
use crate::{Metadata, Payload};
use bytes::BytesMut;
use local_ip_address::local_ip;
use protobuf::well_known_types::Any;
use protobuf::Message;
use serde::Serialize;
use std::any::type_name;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use std::ops::Deref;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::{TcpStream, ToSocketAddrs};

pub mod ability {

    use serde::Serialize;
    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientRemoteAbility {
        support_remote_connection: bool,
    }

    impl ClientRemoteAbility {
        pub fn is_support_remote_connection(&self) -> bool {
            self.support_remote_connection
        }

        pub fn set_support_remote_connection(&mut self, support_remote_connection: bool) {
            self.support_remote_connection = support_remote_connection;
        }
    }

    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientConfigAbility {
        support_remote_metrics: bool,
    }

    impl ClientConfigAbility {
        pub fn is_support_remote_metrics(&self) -> bool {
            self.support_remote_metrics
        }

        pub fn set_support_remote_metrics(&mut self, support_remote_metrics: bool) {
            self.support_remote_metrics = support_remote_metrics;
        }
    }

    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientNamingAbility {
        support_delta_push: bool,
        support_remote_metric: bool,
    }

    impl ClientNamingAbility {
        pub fn is_support_delta_push(&self) -> bool {
            self.support_delta_push
        }

        pub fn set_support_delta_push(&mut self, support_delta_push: bool) {
            self.support_delta_push = support_delta_push;
        }

        pub fn is_support_remote_metric(&self) -> bool {
            self.support_remote_metric
        }

        pub fn set_support_remote_metric(&mut self, support_remote_metric: bool) {
            self.support_remote_metric = support_remote_metric;
        }
    }

    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientAbilities {
        remote_ability: ClientRemoteAbility,
        config_ability: ClientConfigAbility,
        naming_ability: ClientNamingAbility,
    }

    impl ClientAbilities {
        pub fn get_remote_ability(&self) -> &ClientRemoteAbility {
            &self.remote_ability
        }
        pub fn set_remote_ability(&mut self, remote_ability: ClientRemoteAbility) {
            self.remote_ability = remote_ability;
        }

        pub fn get_config_ability(&self) -> &ClientConfigAbility {
            &self.config_ability
        }
        pub fn set_config_ability(&mut self, config_ability: ClientConfigAbility) {
            self.config_ability = config_ability;
        }

        pub fn get_naming_ability(&self) -> ClientNamingAbility {
            self.naming_ability
        }
        pub fn set_naming_ability(&mut self, naming_ability: ClientNamingAbility) {
            self.naming_ability = naming_ability;
        }
    }

    impl Default for ClientAbilities {
        fn default() -> Self {
            ClientAbilities {
                remote_ability: ClientRemoteAbility {
                    support_remote_connection: false,
                },
                config_ability: ClientConfigAbility {
                    support_remote_metrics: false,
                },
                naming_ability: ClientNamingAbility {
                    support_delta_push: false,
                    support_remote_metric: false,
                },
            }
        }
    }
}
pub mod request {
    use crate::grpc_client::ability::*;
    use serde::Serialize;
    use std::collections::HashMap;
    use std::ops::{Deref, DerefMut};

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Request {
        headers: HashMap<String, String>,
        request_id: String,
    }

    impl Request {
        pub fn new(headers: HashMap<String, String>, request_id: &str) -> Self {
            Request {
                headers,
                request_id: request_id.to_string(),
            }
        }
        pub fn put_header(&mut self, key: String, value: String) {
            self.headers.insert(key, value);
        }

        pub fn put_all_headers(&mut self, headers: HashMap<String, String>) {
            self.headers.extend(headers);
        }

        pub fn get_header(&self, key: String) -> Option<String> {
            self.headers.get(&key).map(|v| v.clone())
        }

        pub fn get_header_with_default(&self, key: String, default_value: String) -> String {
            self.headers.get(&key).map_or(default_value, |v| v.clone())
        }

        pub fn get_request_id(&self) -> String {
            self.request_id.clone()
        }

        pub fn set_request_id(&mut self, request_id: String) {
            self.request_id = request_id;
        }

        pub fn get_headers(&self) -> HashMap<String, String> {
            self.headers.clone()
        }

        pub fn clear_headers(&mut self) {
            self.headers.clear()
        }
    }

    pub trait InternalRequest {
        fn get_module(&self) -> String {
            String::from("internal")
        }
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectionSetupRequest {
        #[serde(flatten)]
        request: Request,
        client_version: String,
        abilities: ClientAbilities,
        tenant: String,
        labels: HashMap<String, String>,
    }

    impl InternalRequest for ConnectionSetupRequest {}

    impl ConnectionSetupRequest {
        pub fn new(
            request: Request,
            client_version: &str,
            tenant: &str,
            labels: HashMap<String, String>,
        ) -> Self {
            ConnectionSetupRequest {
                request,
                labels,
                client_version: client_version.to_string(),
                tenant: tenant.to_string(),
                abilities: ClientAbilities::default(),
            }
        }
        pub fn get_client_version(&self) -> String {
            self.client_version.clone()
        }

        pub fn set_client_version(&mut self, client_version: &str) {
            self.client_version = client_version.to_string();
        }

        pub fn get_labels(&self) -> HashMap<String, String> {
            self.labels.clone()
        }

        pub fn set_labels(&mut self, labels: HashMap<String, String>) {
            self.labels = labels;
        }

        pub fn get_tenant(&self) -> String {
            self.tenant.clone()
        }

        pub fn set_tenant(&mut self, tenant: &str) {
            self.tenant = tenant.to_string();
        }

        pub fn get_abilities(&self) -> ClientAbilities {
            self.abilities
        }

        pub fn set_abilities(&mut self, abilities: ClientAbilities) {
            self.abilities = abilities;
        }
    }

    impl Deref for ConnectionSetupRequest {
        type Target = Request;

        fn deref(&self) -> &Self::Target {
            &self.request
        }
    }

    impl DerefMut for ConnectionSetupRequest {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.request
        }
    }

    #[cfg(test)]
    mod test {
        use crate::grpc_client::convert;
        use crate::grpc_client::request::{ConnectionSetupRequest, InternalRequest, Request};

        #[test]
        fn test_deref_request() {
            let mut request = Request {
                headers: Default::default(),
                request_id: "102123".to_string(),
            };
            request.set_request_id("rid-10123123".to_string());
            println!("{}", request.get_request_id());
            let connection_setup_request = ConnectionSetupRequest {
                request,
                client_version: "2.0.1".to_string(),
                abilities: Default::default(),
                tenant: "".to_string(),
                labels: Default::default(),
            };
            println!("module=> {}", connection_setup_request.get_module());
            println!("headers=> {:?}", connection_setup_request.get_headers());
            println!(
                "client_version=> {}",
                connection_setup_request.get_client_version()
            );

            println!(
                "json str: {}",
                serde_json::to_string(&connection_setup_request).unwrap()
            );
            let payload = convert(connection_setup_request);
            println!("payload: {:?}", payload);
        }
    }
}
pub struct GrpcConnection {
    // The `TcpStream`. It is decorated with a `BufWriter`, which provides write
    // level buffering. The `BufWriter` implementation provided by Tokio is
    // sufficient for our needs.
    stream: BufWriter<TcpStream>,
    // The buffer for reading frames.
    buffer: BytesMut,
}

impl GrpcConnection {
    pub fn new(socket: TcpStream) -> Self {
        GrpcConnection {
            stream: BufWriter::new(socket),
            // Default to a 4kb read buffer.
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn send_request<R: Serialize + Deref<Target = Request>>(&mut self, request: R) {
        let convert = convert(request);
        match self
            .stream
            .write_all(convert.write_to_bytes().unwrap().as_slice())
            .await
        {
            Ok(_) => {
                log::info!("send request successfully.");
            }
            Err(e) => {
                log::error!("send request error: {:?}", e);
                return;
            }
        }
        let _ = self.stream.flush().await;
    }
}

pub struct GrpcClient {
    connection: GrpcConnection,
}

pub struct ServerInfo {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl ServerInfo {
    pub fn to_socket_addrs(&self) -> (Ipv4Addr, u16) {
        (self.ip.clone(), self.port)
    }
}

impl GrpcClient {
    pub async fn connect_to_server(server_info: ServerInfo) -> Result<GrpcClient, Box<dyn Error>> {
        connect(server_info.to_socket_addrs()).await
    }

    pub async fn send_request<R: Serialize + Deref<Target = Request>>(&mut self, request: R) {
        self.connection.send_request(request);
    }
}

pub async fn connect<A: ToSocketAddrs>(socket_addrs: A) -> Result<GrpcClient, Box<dyn Error>> {
    let stream = TcpStream::connect(socket_addrs).await?;
    Ok(GrpcClient {
        connection: GrpcConnection::new(stream),
    })
}

pub fn convert<T: Serialize + Deref<Target = Request>>(request: T) -> Payload {
    let _type_name = type_name::<T>();
    let _type_name = _type_name.rsplit_once("::").unwrap();
    let mut new_meta = Metadata::new();
    new_meta.set_field_type(_type_name.1.to_string());
    new_meta.set_clientIp(
        local_ip()
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
            .to_string(),
    );
    new_meta.set_headers(request.get_headers());
    let json_str = serde_json::to_string(&request).unwrap();
    let mut payload = Payload::new();
    let mut any = Any::new();
    any.set_value(Vec::from(json_str.as_bytes()));
    payload.set_body(any);
    payload.set_metadata(new_meta);
    payload
}

#[test]
fn test_ip() {
    println!("local ip is: {:?}", local_ip().unwrap())
}
