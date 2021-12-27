use crate::property_key_const::{
    CLUSTER_NAME, CONTEXT_PATH, DEFAULT_PORT, ENDPOINT, ENDPOINT_PORT, NAMESPACE, SERVER_ADDR,
};
use crate::security::SecurityProxy;
use crate::{NacosError, NacosResult, Properties};
use std::cell::RefCell;
use std::fmt::format;
use std::sync::{Arc, Mutex};

const HTTPS: &'static str = "https://";
const HTTP: &'static str = "http://";

pub struct ClientWorker {
    login: SecurityProxy,
}

impl ClientWorker {
    pub async fn run(&mut self) {}
}

struct SafeAccess<T> {
    data: Mutex<Arc<RefCell<T>>>,
}
impl<T> SafeAccess<T> {
    pub fn new(data: T) -> SafeAccess<T> {
        SafeAccess {
            data: Mutex::new(Arc::new(RefCell::new(data))),
        }
    }

    pub fn value(&self) -> Arc<RefCell<&T>> {
        self.data.lock().unwrap().clone()
    }

    pub fn set(&self, data: T) {
        *self.data.lock().unwrap().clone().get_mut() = data;
    }
}

/// A struct to manager Server list.
struct ServerListManager {
    pub endpoint: Option<String>,
    pub endpoint_port: Option<u16>,
    pub context_path: Option<String>,
    pub server_list_name: Option<String>,
    pub namespace: Option<String>,
    pub tenant: Option<String>,
    pub address_server_url: SafeAccess<Option<String>>,
    pub server_urls: SafeAccess<Option<Vec<String>>>,
    pub is_started: SafeAccess<bool>,
    pub server_addrs_str: SafeAccess<Option<String>>,
}

fn init_server_list_param(
    properties: &Properties,
) -> (Option<String>, Option<u16>, Option<String>, Option<String>) {
    let endpoint = match properties.get(ENDPOINT) {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let endpoint_port = properties
        .get(ENDPOINT_PORT)
        .unwrap_or(&DEFAULT_PORT.to_string())
        .parse::<u16>()
        .unwrap();

    let content_path = match properties.get(CONTEXT_PATH) {
        Some(value) => Some(value.to_string()),
        None => None,
    };
    let server_list_name = match properties.get(CLUSTER_NAME) {
        Some(value) => Some(value.to_string()),
        None => None,
    };
    (
        endpoint,
        Some(endpoint_port),
        content_path,
        server_list_name,
    )
}

fn get_apache_server_list(url: &str) {}

impl ServerListManager {
    pub fn new(properties: Properties) -> NacosResult<Self> {
        let is_started = SafeAccess::new(false);
        let server_addrs_str = properties.get(SERVER_ADDR);
        let namespace = if let Some(namespace) = properties.get(NAMESPACE) {
            Some(namespace.to_string())
        } else {
            None
        };
        if server_addrs_str.is_some() {
            let server_addrs: Vec<String> = server_addrs_str
                .unwrap()
                .split(",")
                .map(|address| {
                    return if address.starts_with(HTTPS) | address.starts_with(HTTP) {
                        address.to_string()
                    } else {
                        let ip_ports = address.split(":");
                        if ip_ports.into_iter().count() == 1 {
                            format!("{}{}:{}", HTTP, address.trim(), DEFAULT_PORT)
                        } else {
                            format!("{}{}", HTTP, address.trim())
                        }
                    };
                })
                .collect();
            let tenant = if namespace.is_empty() {
                "".to_string()
            } else {
                namespace.to_string()
            };
            let address_server_url = format!("http://{}:{}/{}", )
            ServerListManager {
                endpoint: None,
                endpoint_port: None,
                context_path: None,
                server_list_name: None,
                namespace,
                tenant: Some(tenant),
                address_server_url: SafeAccess::new(None),
                server_urls: SafeAccess::new(Some(server_addrs)),
                is_started,
                server_addrs_str: SafeAccess::new(Some(server_addrs_str.unwrap().to_string())),
            }
        } else {
            // todo! when using endpoint to connect to nacos server.
            let (endpoint, endpoint_port, content_path, server_list_name) =
                init_server_list_param(&properties);
            let endpoint_url = if !endpoint.is_empty() {
                format!("{}:{}", endpoint.unwrap(), endpoint_port.unwrap())
            } else {
                "".to_string()
            };
        }
        todo!()
    }

    pub fn start(&mut self) {
        if self.is_started.value().clone() {
            return;
        }

        // create a thread to load servers.
        let server_addrs_url = self
            .address_server_url
            .value()
            .clone()
            .borrow()
            .as_ref()
            .unwrap()
            .to_string();
        let task = std::thread::spawn(move || {
            // do task here.
        });
    }
}
