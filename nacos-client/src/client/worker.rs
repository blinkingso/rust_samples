use crate::property_key_const::{
    CLUSTER_NAME, CONTEXT_PATH, ENDPOINT, ENDPOINT_PORT, NAMESPACE, SERVER_ADDR,
};
use crate::security::SecurityProxy;
use crate::{NacosError, NacosResult, Properties};

const HTTPS: &'static str = "https://";
const HTTP: &'static str = "http://";

pub struct ClientWorker {
    login: SecurityProxy,
}

impl ClientWorker {
    pub async fn run(&mut self) {}
}

/// A struct to manager Server list.
struct ServerListManager {
    pub server_addrs: Vec<String>,
    pub endpoint: Option<String>,
    pub context_path: Option<String>,
    pub server_list_name: Option<String>,
}

impl ServerListManager {
    pub fn new(properties: Properties) -> NacosResult<Self> {
        let server_addrs_str = properties
            .get(SERVER_ADDR)
            .ok_or_else(Err(NacosError::msg("server_addr must be specified")))?;
        let namespace = properties.get(NAMESPACE).unwrap_or("".as_ref());
        let endpoint = properties.get(ENDPOINT).unwrap_or("".as_ref());
        let endpoint_port = properties
            .get(ENDPOINT_PORT)
            .or(Some("8080".as_ref()))
            .unwrap()
            .parse::<u16>()
            .unwrap();
        let endpoint_url = if !endpoint.is_empty() {
            format!("{}:{}", endpoint, endpoint_port)
        } else {
            "".to_string()
        };
        let context_path = properties.get(CONTEXT_PATH).unwrap_or("".as_ref());
        let server_list_name = properties.get(CLUSTER_NAME).unwrap_or("".as_ref());
        if !server_addrs_str.is_empty() {}
        todo!()
    }
}
