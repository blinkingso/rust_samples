extern crate config as yaml_config;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use tokio::runtime::Runtime;

pub mod client;
pub mod common;
pub mod config;
pub mod listener;
pub mod security;

pub type Properties = HashMap<String, String>;
pub type NacosResult<T> = anyhow::Result<T>;
pub type NacosError = anyhow::Error;
pub struct NacosClient {
    // hold a runtime.
    rt: Runtime,
}
pub mod property_key_const {
    pub const NAMESPACE: &'static str = "namespace";
    pub const DEFAULT_NAMESPACE: &'static str = "";
    pub const SERVER_ADDR: &'static str = "server_addr";
    pub const ENDPOINT: &'static str = "endpoint";
    pub const ENDPOINT_PORT: &'static str = "endpoint_port";
    pub const CONTEXT_PATH: &'static str = "context_path";
    pub const CLUSTER_NAME: &'static str = "cluster_name";
    pub const SECRET_KEY: &'static str = "secret_key";
    pub const ACCESS_KEY: &'static str = "access_key";
    pub const RAM_ROLE_NAME: &'static str = "ram_role_name";
    pub const CONFIG_LONG_POLL_TIMEOUT: &'static str = "config_long_poll_timeout";
    pub const CONFIG_RETRY_TIME: &'static str = "config_retry_time";
    pub const MAX_RETRY: &'static str = "max_retry";
    pub const ENABLE_REMOTE_SYNC_CONFIG: &'static str = "enable_remote_syn_config";
    pub const USERNAME: &'static str = "username";
    pub const PASSWORD: &'static str = "password";
    pub const DEFAULT_PORT: &'static str = "8848";
    pub const BASE_PATH: &'static str = "/v1/cs";
    pub const CONFIG_CONTROLLER_PATH: &'static str = "/configs";
}
pub mod constants {
    pub const CONFIG_LONG_POLL_TIMEOUT: i32 = 30000;
    pub const MIN_CONFIG_LONG_POLL_TIMEOUT: i32 = 10000;
    pub const CONFIG_RETRY_TIME: i32 = 1000;
}
pub mod resp {
    pub const RESP_ACCESS_TOKEN: &'static str = "accessToken";
    pub const RESP_TOKEN_TTL: &'static str = "tokenTtl";
    pub const RESP_GLOBAL_ADMIN: &'static str = "globalAdmin";
}
pub mod utils {
    use crate::config::props::NacosConfigProperties;
    use crate::NacosResult;
    use serde::Deserialize;
    use yaml_config::{Config, Environment, File};

    /// read config from a file.
    pub fn read_toml_from_resources<'de, T: Deserialize<'de>>(prefix: &str) -> NacosResult<T> {
        let mut s = Config::default();
        let default = format!("resources/{}.toml", prefix);
        s.merge(File::with_name(default.as_str()))?;
        // config environment conf file.
        let env = std::env::var("RUN_MODE").unwrap_or(String::from("dev"));
        let file_name = format!("resources/{}-{}.toml", prefix, env);
        s.merge(File::with_name(file_name.as_str()))?;
        // from environment
        s.merge(Environment::with_prefix(prefix))?;
        Ok(s.try_into()?)
    }

    #[test]
    fn test_read() {
        let a = read_toml_from_resources::<NacosConfigProperties>("nacos");
        if let Ok(ref p) = a {
            println!("{:?}", p);
        } else {
            eprintln!("error: {:?}", a.unwrap_err());
        }
    }
}
pub mod crypto {
    /// get md5 string with lower case.
    pub fn get_md5_string(message: &str) -> String {
        let digest = md5::compute(message.as_bytes());
        format!("{:?}", digest)
    }

    #[test]
    fn test_md5() {
        let digest = get_md5_string("hello world");
        println!("{}", digest);
    }
}
