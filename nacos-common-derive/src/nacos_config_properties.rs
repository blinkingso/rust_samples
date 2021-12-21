use crate::config_type::ConfigType;

use getset::{Getters, Setters};
/// Nacos config item properties.
#[derive(Getters, Setters, Debug)]
pub struct NacosConfigProperties<'a> {
    #[getset(get, set)]
    prefix: &'a str,
    #[getset(get, set)]
    group_id: &'a str,
    #[getset(get, set)]
    data_id: &'a str,
    #[getset(get, set)]
    yaml: bool,
    #[getset(get, set)]
    auto_refreshed: bool,
    #[getset(get, set)]
    config_type: ConfigType,
    #[getset(get, set)]
    ignore_invalid_fields: bool,
    #[getset(get, set)]
    ignore_nested_properties: bool,
    #[getset(get, set)]
    ignore_unknown_fields: bool,
}

/// default configuration load from.
impl<'a> Default for NacosConfigProperties<'a> {
    fn default() -> Self {
        NacosConfigProperties {
            prefix: "",
            group_id: "DEFAULT_GROUP",
            data_id: "DEFAULT_DATA",
            yaml: false,
            auto_refreshed: false,
            config_type: ConfigType::PROPERTIES,
            ignore_invalid_fields: false,
            ignore_nested_properties: false,
            ignore_unknown_fields: false,
        }
    }
}

impl<'a> NacosConfigProperties<'a> {
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
    ncp.set_prefix("prefix value");
    println!("group_id = {}", ncp.group_id());
    println!("{:?}", ncp);
}
