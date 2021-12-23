// //! util access to nacos server.
// use crate::config::source::NacosPropertySource;
// use crate::property_key_const::*;
// use crate::Properties;
// use lazy_static::lazy_static;
// use std::collections::HashMap;
// use std::fmt::format;
// use std::sync::Mutex;
// use tokio::task::JoinHandle;
// use trait_async::trait_async;
//
// /// listener to monitor configuration change.
// trait Listener {
//     /// A function to receive remote change messages.
//     fn receive(&self, change_info: String);
// }
//
// struct ConfigListener;
//
// impl Listener for ConfigListener {
//     fn receive(&self, config_info: String) {
//         // todo parse config_info and replace some values.
//     }
// }
//
// #[trait_async]
// trait ConfigService {
//     /// get config.
//     async fn get_config(&self, data_id: String, group: String, timeout_ms: u64);
//
//     /// get config and register listener.
//     async fn get_config_and_sign_listener(
//         &self,
//         data_id: String,
//         group: String,
//         timeout_ms: u64,
//         listener: dyn Listener,
//     );
//
//     /// add a listener to the configuration. after the server modified the configuration,
//     /// the client will use the incoming listener callback.
//     fn add_listener<L>(&mut self, data_id: String, group: String, listener: L)
//     where
//         L: Listener + Sized;
//
//     /// remove a listener.
//     fn remove_listener<L>(&mut self, data_id: String, group: String, listener: L)
//     where
//         L: Listener + Sized;
//
//     /// get server status;
//     async fn get_server_status(&self);
//
//     /// shutdown the resource service.
//     async fn shutdown(&mut self);
// }
//
// /// util access to get configurations from remote server.
// pub struct NacosConfigService<L: Listener + Sized> {
//     namespace: Option<String>,
//     listeners: Mutex<HashMap<String, L>>,
// }
//
// #[trait_async]
// impl<L> ConfigService for NacosConfigService<L>
// where
//     L: Listener + Sized,
// {
//     async fn get_config(&self, data_id: String, group: String, timeout_ms: u64) {
//         todo!()
//     }
//
//     async fn get_config_and_sign_listener(
//         &self,
//         data_id: String,
//         group: String,
//         timeout_ms: u64,
//         listener: Box<dyn Listener>,
//     ) {
//         todo!()
//     }
//
//     fn add_listener(&mut self, data_id: String, group: String, listener: L) {
//         let cache_key = format!(
//             "{}+{}+{}",
//             data_id,
//             group,
//             self.namespace.unwrap_or("".to_string())
//         );
//         {
//             let mut lock = self.listeners.lock().unwrap();
//             lock.insert(cache_key, listener);
//
//             // todo notify to callback.
//         }
//     }
//
//     fn remove_listener(&mut self, data_id: String, group: String, listener: L) {
//         todo!()
//     }
//
//     async fn get_server_status(&self) {
//         todo!()
//     }
//
//     async fn shutdown(&mut self) {
//         todo!()
//     }
// }
//
// pub async fn start_nacos_config_service(source: NacosPropertySource) -> JoinHandle<()> {
//     tokio::task::spawn(async move {})
// }
