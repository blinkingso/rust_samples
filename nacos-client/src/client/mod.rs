use crate::client::config::ConfigChangeEvent;
#[allow(unused_variables, dead_code, unused_attributes)]
use crate::{NacosError, NacosResult};
use serde::Deserialize;
use std::any::TypeId;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use tokio::runtime::Runtime;

pub mod loader;
pub mod service;
pub mod worker;

pub struct SafeAccess<T>
where
    T: Send + 'static,
{
    pub data: Arc<Mutex<T>>,
}
impl<T> SafeAccess<T>
where
    T: Send + 'static,
{
    pub fn new(data: T) -> SafeAccess<T> {
        SafeAccess {
            data: Arc::new(Mutex::new(data)),
        }
    }
}

impl<T> Clone for SafeAccess<T>
where
    T: Send + 'static,
{
    fn clone(&self) -> Self {
        SafeAccess {
            data: Arc::clone(&self.data),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigResponse {
    pub tenant: String,
    pub data_id: String,
    pub group: String,
    pub content: String,
    pub config_type: String,
    pub encrypted_data_key: String,
}

pub struct CacheData {
    name: String,
    data_id: String,
    group: String,
    tenant: String,
    md5: String,
    is_use_local_config: bool,
    local_config_last_modified: u64,
    content: String,
    encrypted_data_key: String,
    last_modified_ts: u64,
    task_id: String,
    ty: String,
    is_init: bool,
    is_sync_with_server: bool,
    listeners: Vec<ListenerWrap>,
}

async fn safe_notify_listener(
    data_id: String,
    group: String,
    content: String,
    ty: String,
    md5: String,
    encrypted_data_key: String,
    wrap: &mut ListenerWrap,
) {
    let listener = wrap.listener.clone();
    if wrap.is_notifying {
        warn!("[notify-current-skip] data_id={}, group={}, md5={}, listener={}, listener is not finish yet,will try next time.",
                    data_id, group, md5, listener.name());
        return;
    }

    let start = Instant::now();
    let mut cr = ConfigResponse {
        tenant: "".to_string(),
        data_id: data_id.clone(),
        group: group.clone(),
        content: content.clone(),
        config_type: "".to_string(),
        encrypted_data_key: encrypted_data_key.clone(),
    };

    wrap.is_notifying = true;
    (*wrap.listener).receive_config_info(content.clone());
    wrap.last_content = content.clone();
    wrap.last_call_md5 = md5.clone();
}

impl CacheData {
    pub fn check_listener_md5(&self) {
        for listener in &self.listeners {
            if self.md5 != listener.last_call_md5 {
                // notify listener.
                safe_notify_listener(
                    self.data_id.clone(),
                    self.group.clone(),
                    self.content.clone(),
                    self.ty.clone(),
                    self.md5.clone(),
                    self.encrypted_data_key.clone(),
                    &mut listener.clone(),
                );
            }
        }
    }

    pub fn add_listener(&mut self, listener: ListenerWrap) {
        self.listeners.push(listener);
    }
}

/// A trait to listen to change list.
pub trait Listener {
    /// A function to get data_id and group to compose a listener's name.
    fn name(&self) -> String;

    /// receive config information.
    fn receive_config_info(&self, content: String);
}

pub trait ConfigListener: Listener {
    /// handle config change.
    fn receive_config_change(event: ConfigChangeEvent);
}

pub struct ListenerWrap {
    pub listener: Rc<Box<dyn Listener>>,
    pub last_content: String,
    pub last_call_md5: String,
    pub is_notifying: bool,
}
impl Clone for ListenerWrap {
    fn clone(&self) -> Self {
        ListenerWrap {
            last_content: self.last_content.clone(),
            last_call_md5: self.last_call_md5.clone(),
            is_notifying: self.is_notifying,
            listener: Rc::clone(&self.listener),
        }
    }
}

impl ListenerWrap {
    pub fn new(listener: impl Listener + 'static, last_content: String, md5: String) -> Self {
        ListenerWrap {
            listener: Rc::new(Box::new(listener)),
            last_content,
            last_call_md5: md5,
            is_notifying: false,
        }
    }
}

pub mod config {
    use super::Listener;
    use crate::client::service::ConfigService;
    use crate::client::ConfigListener;
    use crate::{NacosError, NacosResult};
    use std::collections::HashMap;
    use std::io::BufReader;
    use std::rc::Weak;

    #[derive(Debug, PartialEq, Clone)]
    pub enum PropertyChangeType {
        ADDED,
        MODIFIED,
        DELETED,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ConfigChangeItem {
        pub key: String,
        pub old_value: String,
        pub new_value: String,
        pub ty: PropertyChangeType,
    }

    pub struct ConfigChangeEvent {
        pub data: HashMap<String, ConfigChangeItem>,
    }
    pub struct ConfigChangeListener {
        name: String,
    }

    impl Listener for ConfigChangeListener {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn receive_config_info(&self, content: String) {
            todo!()
        }
    }

    pub struct DelegatingEventPublishingListener {
        config_service: Weak<Box<dyn ConfigService>>,
        data_id: String,
        group_id: String,
        config_type: String,
        delegate: Weak<Box<dyn Listener>>,
    }

    impl DelegatingEventPublishingListener {
        pub fn new(
            config_service: &Box<dyn ConfigService>,
            data_id: &str,
            group_id: &str,
            config_type: &str,
            delegate: &Box<dyn Listener>,
        ) -> Self {
            DelegatingEventPublishingListener {
                config_service: Weak::new(),
                data_id: data_id.to_string(),
                group_id: group_id.to_string(),
                config_type: config_type.to_string(),
                delegate: Weak::new(),
            }
        }
    }

    impl Listener for DelegatingEventPublishingListener {
        fn name(&self) -> String {
            "DelegatingEventPublishingListener".to_string()
        }

        fn receive_config_info(&self, content: String) {
            todo!()
        }
    }

    impl ConfigListener for ConfigChangeListener {
        fn receive_config_change(event: ConfigChangeEvent) {}
    }

    pub fn parse_change_data(
        last_content: &str,
        content: &str,
        ty: &str,
    ) -> HashMap<String, String> {
        todo!()
    }

    pub trait ConfigChangeParser {
        // judge type.
        fn is_responsible_for(&self, ty: String) -> bool;
        /// compare old and new data.
        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>>;

        fn filter_change_data(
            &self,
            old_map: &HashMap<String, String>,
            new_map: &HashMap<String, String>,
        ) -> HashMap<String, ConfigChangeItem> {
            let mut result: HashMap<String, ConfigChangeItem> = HashMap::with_capacity(16);
            for (key, val) in old_map {
                let mut cci: ConfigChangeItem;
                if new_map.contains_key(key) {
                    if val.eq(new_map.get(key).unwrap()) {
                        // no change key-value here.
                        continue;
                    }
                    cci = ConfigChangeItem {
                        key: key.to_string(),
                        old_value: val.to_string(),
                        new_value: new_map.get(key).unwrap().clone(),
                        ty: PropertyChangeType::MODIFIED,
                    }
                } else {
                    // only old key exists.
                    cci = ConfigChangeItem {
                        key: key.to_string(),
                        old_value: val.to_string(),
                        new_value: "".to_string(),
                        ty: PropertyChangeType::DELETED,
                    }
                }

                result.insert(key.to_string(), cci);
            }

            for (key, val) in new_map {
                if !old_map.contains_key(key) {
                    let cci = ConfigChangeItem {
                        key: key.to_string(),
                        old_value: "".to_string(),
                        new_value: val.to_string(),
                        ty: PropertyChangeType::ADDED,
                    };

                    result.insert(key.to_string(), cci);
                }
            }
            result
        }
    }

    pub struct PropertiesConfigChangeParser;
    pub struct YamlConfigChangeParser;
    impl PropertiesConfigChangeParser {
        fn parse_properties(&self, text: &str) -> NacosResult<HashMap<String, String>> {
            let mut result = HashMap::new();
            if !text.is_empty() {
                for line in text.lines().into_iter() {
                    if line.starts_with('#') {
                        continue;
                    }
                    let (k, v) = line
                        .split_once('=')
                        .ok_or(NacosError::msg("properties parse error"))?;
                    result.insert(k.to_string(), v.to_string());
                }
            }

            Ok(result)
        }
    }
    impl ConfigChangeParser for PropertiesConfigChangeParser {
        fn is_responsible_for(&self, ty: String) -> bool {
            "PROPERTIES".eq_ignore_ascii_case(ty.as_str())
        }

        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>> {
            let old_map = self.parse_properties(old_content)?;
            let new_map = self.parse_properties(new_content)?;

            Ok(self.filter_change_data(&old_map, &new_map))
        }
    }

    impl ConfigChangeParser for YamlConfigChangeParser {
        fn is_responsible_for(&self, ty: String) -> bool {
            "YAML".eq_ignore_ascii_case(ty.as_str())
        }

        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>> {
            todo!()
        }
    }

    pub struct ConfigChangeHandler {
        pub parses: Vec<Box<dyn ConfigChangeParser>>,
    }
    impl ConfigChangeHandler {
        pub fn new() -> Self {
            let mut parses: Vec<Box<dyn ConfigChangeParser>> = Vec::new();
            parses.push(Box::new(PropertiesConfigChangeParser));
            parses.push(Box::new(YamlConfigChangeParser));
            ConfigChangeHandler { parses }
        }

        pub fn parse_change_data(
            &self,
            old_content: &str,
            new_content: &str,
            ty: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>> {
            for parser in self.parses.iter() {
                if parser.is_responsible_for(ty.to_string()) {
                    return parser.do_parse(old_content, new_content);
                }
            }

            Err(NacosError::msg("Unsupported config type parser."))
        }
    }
}
