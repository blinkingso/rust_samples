use crate::property_key_const::{DEFAULT_NAMESPACE, NAMESPACE, SERVER_ADDR};
use crate::{NacosError, NacosResult, Properties};
use std::net::{IpAddr, Ipv4Addr};
use std::num::NonZeroU16;
use std::str::FromStr;

pub(crate) struct ServerListManager {
    server_lists: Vec<IpAddr>,
    namespace: String,
    tenant: Option<String>,
    is_started: bool,
    endpoint: String,
    endpoint_port: NonZeroU16,
}

impl ServerListManager {
    pub fn init(properties: Properties) -> NacosResult<ServerListManager> {
        let servers = properties.get(SERVER_ADDR);
        let servers = match servers {
            None => return Err(NacosError::msg("server_addr is not set.")),
            Some(server_str) => {
                /// just ip4 here.
                let server_str_list = server_str.split(",").collect::<Vec<String>>();
                server_str_list.iter().flat_map(|server_port| {
                    let ip_addr = Ipv4Addr::from_str(server_port);
                    if ip_addr.is_err() {
                        return Err(NacosError::msg("server_addr parse error."));
                    }
                    return Ok(IpAddr::V4(ip_addr.unwrap()));
                })
            }
        }
        .collect::<Vec<IpAddr>>();
        if servers.is_empty() {
            return Err(NacosError::msg("server_addr is not set."));
        }
        let mut slm = ServerListManager {
            server_lists: vec![],
            namespace: "".to_string(),
            tenant: None,
            is_started: false,
            endpoint: "".to_string(),
            endpoint_port: unsafe { NonZeroU16(8080) },
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
