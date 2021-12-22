/// ConfigType
#[derive(Debug)]
pub struct ConfigType(&'static str);

macro_rules! config_type {
    (
        $(
            $(#[$docs:meta])*
            ($type:ident, $value:expr);
        )+
    ) => {
        impl ConfigType {
            // const types here.
            $(
                $(#[$docs])*
                pub const $type: ConfigType = ConfigType($value);
            )+

            fn get_config_type(&self) -> &str {
                self.0
            }
        }
    };
}

config_type! {
    #[doc = "type of properties"]
    (PROPERTIES, "properties");
    #[doc = "type of xml"]
    (XML, "xml");
    #[doc = "type of json"]
    (JSON, "json");
    #[doc = "type of text"]
    (TEXT, "text");
    #[doc = "type of html, now not supported"]
    (HTML, "html");
    #[doc = "type of yaml"]
    (YAML, "yaml");
}

impl From<&str> for ConfigType {
    fn from(value: &str) -> Self {
        match value {
            "text" => ConfigType::TEXT,
            "json" => ConfigType::JSON,
            "html" => ConfigType::HTML,
            "properties" => ConfigType::PROPERTIES,
            "yaml" => ConfigType::YAML,
            "xml" => ConfigType::XML,
            others => panic!("Unsupported config type: {}", others),
        }
    }
}

impl From<ConfigType> for &str {
    fn from(value: ConfigType) -> Self {
        value.0
    }
}
