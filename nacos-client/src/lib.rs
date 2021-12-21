use std::collections::HashMap;

pub mod utils;

pub type Properties = HashMap<String, String>;
pub type NacosResult<T> = anyhow::Result<T>;
pub type NacosError = anyhow::Error;
pub mod property_key_const {
    pub const NAMESPACE: &'static str = "namespace";
    pub const DEFAULT_NAMESPACE: &'static str = "public";
    pub const SERVER_ADDR: &'static str = "serverAddr";
}
