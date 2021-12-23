use crate::property_key_const::{DEFAULT_NAMESPACE, NAMESPACE, SERVER_ADDR};
use crate::{NacosError, NacosResult, Properties};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

pub(crate) struct ServerListManager {
    server_lists: Vec<IpAddr>,
    namespace: String,
    tenant: Option<String>,
    is_started: bool,
    endpoint: Option<String>,
    endpoint_port: Option<u16>,
}

impl ServerListManager {
    pub fn init(properties: Properties) -> NacosResult<ServerListManager> {
        let servers = properties.get(SERVER_ADDR);
        if servers.is_none() {
            return Err(NacosError::msg("server_addr mut be set."));
        }
        let servers = servers
            .unwrap()
            .split(",")
            .into_iter()
            .flat_map(|address| {
                let ip_addr = Ipv4Addr::from_str(address);
                if ip_addr.is_err() {
                    return None;
                }
                return Some(IpAddr::V4(ip_addr.unwrap()));
            })
            .collect::<Vec<IpAddr>>();
        if servers.is_empty() {
            return Err(NacosError::msg("server_addr is not set."));
        }
        let mut slm = ServerListManager {
            server_lists: vec![],
            namespace: "".to_string(),
            tenant: None,
            is_started: false,
            endpoint: None,
            endpoint_port: Some(8080),
        };
        slm.server_lists = servers;
        slm.namespace = properties
            .get(NAMESPACE)
            .unwrap_or(&DEFAULT_NAMESPACE.to_string())
            .to_string();
        // todo tenant configuration here.

        Ok(slm)
    }
}
