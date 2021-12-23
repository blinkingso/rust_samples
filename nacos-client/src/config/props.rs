use super::config_type::ConfigType;
use crate::NacosError;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::convert::TryFrom;
use std::str::FromStr;
use yaml_config::{FileFormat, Source};

#[derive(Debug, Serialize, Deserialize)]
pub enum CType {
    XML,
    JSON,
    HTML,
    PROPERTIES,
    YAML,
}

impl ToString for CType {
    fn to_string(&self) -> String {
        format!("{:?}", &self).clone()
    }
}

impl FromStr for CType {
    type Err = NacosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CType::try_from(s)
    }
}

impl TryFrom<&str> for CType {
    type Error = NacosError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "xml" => Ok(CType::XML),
            "json" => Ok(CType::JSON),
            "html" => Ok(CType::HTML),
            "yaml" => Ok(CType::YAML),
            "properties" => Ok(CType::PROPERTIES),
            unknown => Err(NacosError::msg(format!("unsupported type : {}", unknown))),
        }
    }
}

/// A struct to store Nacos configuration read from .toml Configuration files.
#[derive(Debug, Deserialize)]
pub struct NacosConfigProperties {
    pub server_addr: Option<String>,
    pub context_path: Option<String>,
    pub encode: Option<String>,
    pub endpoint: Option<String>,
    pub namespace: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub auto_refreshed: Option<bool>,
    pub data_id: Option<String>,
    pub data_ids: Option<Vec<String>>,
    pub group: Option<String>,
    pub c_type: Option<CType>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub max_retry: Option<String>,
    pub c_long_poll_timeout: Option<String>,
    pub c_retry_time: Option<String>,
    pub enable_remote_sync_config: Option<bool>,
}

impl Default for NacosConfigProperties {
    fn default() -> Self {
        NacosConfigProperties {
            server_addr: Some("127.0.0.1:8848".to_string()),
            context_path: Some("".to_string()),
            encode: Some("UTF-8".to_string()),
            endpoint: None,
            namespace: None,
            access_key: None,
            secret_key: None,
            auto_refreshed: Some(false),
            data_id: None,
            data_ids: None,
            group: Some("DEFAULT_GROUP".to_string()),
            c_type: None,
            username: None,
            password: None,
            max_retry: None,
            c_long_poll_timeout: None,
            c_retry_time: None,
            enable_remote_sync_config: None,
        }
    }
}

#[test]
fn test_type() {
    #[derive(Serialize, Deserialize)]
    struct P {
        t: CType,
        s: String,
    }

    let p = P {
        t: CType::XML,
        s: String::from("hello world"),
    };

    println!("{}", serde_json::to_string(&p).unwrap());
    let s = r#"{"t":"JSON","s":"hello world"}"#;
    let p = serde_json::from_str::<P>(s).unwrap();
    println!("{}", p.t.to_string());
}
