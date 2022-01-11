// use crate::config::props::NacosConfigProperties;
// use crate::config::source::NacosPropertySource;
// use crate::consts::names::{
//     ACCESS_KEY, CONFIG_LONG_POLL_TIMEOUT, CONFIG_RETRY_TIME, ENABLE_REMOTE_SYNC_CONFIG, ENDPOINT,
//     NAMESPACE, PASSWORD, RAM_ROLE_NAME, SECRET_KEY, SERVER_ADDR, USERNAME,
// };
// use crate::consts::val::{DEFAULT_GROUP, DEFAULT_NAMESPACE};
// use crate::{NacosError, NacosResult, Properties};
// /// NacosConfigLoader
// pub struct NacosConfigLoader {
//     pub properties: NacosConfigProperties,
// }
// pub struct NacosPropertySourceProcessor;
//
// impl NacosConfigLoader {
//     /// load properties to sources
//     pub fn load_config(&self) -> NacosResult<Vec<NacosPropertySource>> {
//         let properties = &self.properties;
//         let global_properties = build_global_nacos_properties(&properties);
//         if properties.data_id.is_none() && properties.data_ids.is_none() {
//             return Err(NacosError::msg("data_id or data_ids must be specified."));
//         }
//
//         if properties.c_type.is_none() {
//             return Err(NacosError::msg("c_type must be specified."));
//         }
//
//         let mut data_ids = vec![];
//         if properties.data_ids.is_some() {
//             data_ids.append(&mut properties.data_ids.as_ref().unwrap().clone());
//         }
//         if properties.data_id.is_some() {
//             data_ids.push(properties.data_id.as_ref().unwrap().clone());
//         }
//
//         let group_id = properties
//             .group
//             .as_ref()
//             .unwrap_or(&DEFAULT_GROUP.to_string())
//             .to_string();
//         let auto_refreshed = properties
//             .auto_refreshed
//             .as_ref()
//             .unwrap_or(&false)
//             .to_owned();
//
//         let sources = data_ids
//             .iter()
//             .map(|data_id| NacosPropertySource {
//                 group_id: group_id.clone(),
//                 data_id: data_id.to_string(),
//                 c_type: properties.c_type.as_ref().unwrap().to_string(),
//                 auto_refreshed,
//                 properties: global_properties.clone(),
//             })
//             .collect();
//         return Ok(sources);
//     }
// }
//
// pub fn build_global_nacos_properties(conf: &NacosConfigProperties) -> Properties {
//     let mut properties = Properties::new();
//     properties.insert(
//         SERVER_ADDR.to_string(),
//         conf.server_addr
//             .as_ref()
//             .unwrap_or(&String::from("127.0.0.1::8848"))
//             .to_string(),
//     );
//     properties.insert(
//         NAMESPACE.to_string(),
//         conf.namespace
//             .as_ref()
//             .unwrap_or(&DEFAULT_NAMESPACE.to_string())
//             .to_string(),
//     );
//     properties.insert(
//         ENDPOINT.to_string(),
//         conf.endpoint
//             .as_ref()
//             .unwrap_or(&"".to_string())
//             .to_string(),
//     );
//     properties.insert(
//         SECRET_KEY.to_string(),
//         conf.secret_key
//             .as_ref()
//             .unwrap_or(&"".to_string())
//             .to_string(),
//     );
//     properties.insert(
//         ACCESS_KEY.to_string(),
//         conf.access_key
//             .as_ref()
//             .unwrap_or(&"".to_string())
//             .to_string(),
//     );
//     properties.insert(RAM_ROLE_NAME.to_string(), "".to_string());
//     properties.insert(
//         CONFIG_LONG_POLL_TIMEOUT.to_string(),
//         conf.c_long_poll_timeout
//             .as_ref()
//             .unwrap_or(&"30000".to_string())
//             .to_string(),
//     );
//     properties.insert(
//         CONFIG_RETRY_TIME.to_string(),
//         conf.c_retry_time
//             .as_ref()
//             .unwrap_or(&"0".to_string())
//             .to_string(),
//     );
//     properties.insert(
//         ENABLE_REMOTE_SYNC_CONFIG.to_string(),
//         conf.enable_remote_sync_config
//             .as_ref()
//             .unwrap_or(&false)
//             .to_string()
//             .to_string(),
//     );
//     properties.insert(
//         USERNAME.to_string(),
//         conf.username
//             .as_ref()
//             .unwrap_or(&"".to_string())
//             .to_string(),
//     );
//     properties.insert(
//         PASSWORD.to_string(),
//         conf.password
//             .as_ref()
//             .unwrap_or(&"".to_string())
//             .to_string(),
//     );
//     properties
// }
//
// /// A factory to add listeners if auto refreshed, etc...
// impl NacosPropertySourceProcessor {
//     pub fn add_listener_if_auto_refreshed(sources: NacosPropertySource) {
//         if sources.auto_refreshed {
//             let data_id = sources.data_id;
//             let group_id = sources.group_id;
//             let c_type = sources.c_type;
//             let namespace = sources.properties.get(NAMESPACE).unwrap();
//         }
//     }
// }
