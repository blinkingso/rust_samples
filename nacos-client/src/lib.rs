#[macro_use]
extern crate serde;
extern crate config as yaml_config;
extern crate serde_json;

use std::collections::HashMap;

pub mod client;
pub mod config;
pub mod listener;
pub mod security;
pub mod utils;

pub type Properties = HashMap<String, String>;
pub type NacosResult<T> = anyhow::Result<T>;
pub type NacosError = anyhow::Error;
pub mod property_key_const {
    pub const NAMESPACE: &'static str = "namespace";
    pub const DEFAULT_NAMESPACE: &'static str = "public";
    pub const SERVER_ADDR: &'static str = "server_addr";
    pub const ENDPOINT: &'static str = "endpoint";
    pub const SECRET_KEY: &'static str = "secret_key";
    pub const ACCESS_KEY: &'static str = "access_key";
    pub const RAM_ROLE_NAME: &'static str = "ram_role_name";
    pub const CONFIG_LONG_POLL_TIMEOUT: &'static str = "config_long_poll_timeout";
    pub const CONFIG_RETRY_TIME: &'static str = "config_retry_time";
    pub const MAX_RETRY: &'static str = "max_retry";
    pub const ENABLE_REMOTE_SYNC_CONFIG: &'static str = "enable_remote_syn_config";
    pub const USERNAME: &'static str = "username";
    pub const PASSWORD: &'static str = "password";
}
