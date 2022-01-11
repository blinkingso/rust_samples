use bytes::Bytes;
use std::collections::HashMap;

pub mod loader;
pub mod service;
pub mod worker;

struct NacosConfig {}

/// nacos client
pub struct NacosConfigClient {
    /// configurations from resources.
    resources: Bytes,
}

impl NacosConfigClient {}
