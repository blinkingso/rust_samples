use std::net::{IpAddr, Ipv4Addr};

pub struct ServerAttr {
    pub ip_addr: Option<IpAddr>,
    pub port: Option<u16>,
    pub username: Option<&'static str>,
    pub password: Option<&'static str>,
    pub long_poll_timeout: Option<u32>,
    pub retry_time: Option<u32>,
    pub max_retry: Option<u8>,

    pub enable_remote_sync_config: Option<bool>,
    pub log_level: Option<&'static str>,
    pub cache_path: Option<&'static str>,
}

impl Default for ServerAttr {
    fn default() -> Self {
        ServerAttr {
            ip_addr: Some(IpAddr::V4(Ipv4Addr::from([127, 0, 0, 1]))),
            port: Some(8848),
            username: Some("nacos"),
            password: Some("nacos"),
            long_poll_timeout: Some(30000),
            retry_time: Some(2000),
            max_retry: Some(3),
            enable_remote_sync_config: Some(false),
            log_level: Some("debug"),
            cache_path: Some("nacos/config"),
        }
    }
}
