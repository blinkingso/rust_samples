use super::config_type::ConfigType;

use getset::{Getters, Setters};
/// Nacos config item properties.
#[derive(Getters, Setters, Debug)]
pub struct NacosConfigProperties {
    #[getset(get, set)]
    prefix: String,
    #[getset(get, set)]
    group_id: String,
    #[getset(get, set)]
    data_id: String,
    #[getset(get, set)]
    yaml: bool,
    #[getset(get, set)]
    auto_refreshed: bool,
    #[getset(get, set)]
    config_type: ConfigType<'static>,
    #[getset(get, set)]
    ignore_invalid_fields: bool,
    #[getset(get, set)]
    ignore_nested_properties: bool,
    #[getset(get, set)]
    ignore_unknown_fields: bool,
}

/// default configuration load from.
impl Default for NacosConfigProperties {
    fn default() -> Self {
        NacosConfigProperties {
            prefix: "".to_string(),
            group_id: "DEFAULT_GROUP".to_string(),
            data_id: "DEFAULT_DATA".to_string(),
            yaml: false,
            auto_refreshed: false,
            config_type: ConfigType::PROPERTIES,
            ignore_invalid_fields: false,
            ignore_nested_properties: false,
            ignore_unknown_fields: false,
        }
    }
}

impl NacosConfigProperties {
    /// attr: auto_refreshed
    pub const NACOS_ATTR_NAME_PREFIX: &'static str = "prefix";
    pub const NACOS_ATTR_NAME_DATA_ID: &'static str = "dataId";
    pub const NACOS_ATTR_NAME_GROUP: &'static str = "groupId";
    pub const NACOS_ATTR_NAME_AUTO_REFRESHED: &'static str = "autoRefreshed";
    pub const NACOS_ATTR_NAME_INTERVAL: &'static str = "interval";
    pub const NACOS_ATTR_NAME_TYPE: &'static str = "type";
    pub const NACOS_ATTR_NAME_IGNORE_INVALID_FIELDS: &'static str = "ignoreInvalidFields";
    pub const NACOS_ATTR_NAME_IGNORE_UNKNOWN_FIELDS: &'static str = "ignoreUnknownFields";
    pub const NACOS_ATTR_NAME_IGNORE_NESTED_PROPERTIES: &'static str = "ignoreNestedProperties";
}

#[test]
fn test_getter_setter() {
    let mut ncp = NacosConfigProperties::default();
    ncp.set_prefix("prefix value".to_string());
    println!("group_id = {}", ncp.group_id());
    println!("{:?}", ncp);
}
