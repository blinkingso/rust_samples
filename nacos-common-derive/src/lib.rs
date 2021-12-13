/// Nacos config item properties.
pub struct NacosConfigProperties<'a> {
    group: &'a str,
    data_id: &'a str,
    name: Option<&'a str>,
    auto_refresh: Option<bool>,
    interval: Option<u32>,
}

/// default configuration load from.
impl<'a> Default for NacosConfigProperties<'a> {
    fn default() -> Self {
        NacosConfigProperties {
            group: "nacos",
            data_id: "nacos",
            name: None,
            auto_refresh: None,
            interval: None,
        }
    }
}
