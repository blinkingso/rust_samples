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

pub mod loader;
pub mod service;
pub mod worker;

pub struct GroupKey<'a> {
    group_id: &'a str,
    data_id: &'a str,
    tanant: &'a str,
}

impl<'a> GroupKey<'a> {
    pub fn new(data_id: &'a str, group_id: &'a str, tanant: Option<&'a str>) -> NacosResult<Self> {
        if data_id.is_empty() {
            return Err(NacosError::msg("data id must not be empty."));
        }
        if group_id.is_empty() {
            return Err(NacosError::msg("group id must not be empty."));
        }
        Ok(GroupKey {
            group_id,
            data_id,
            tanant: if let Some(t) = tanant { t } else { "" },
        })
    }
}

impl<'a> Display for GroupKey<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("{}+{}", self.data_id, self.group_id);
        if !self.tanant.is_empty() {
            result.push_str(format!("+{}", self.tanant).as_str());
        }
        write!(f, "{}", result)
    }
}

impl<'a> TryFrom<&'a str> for GroupKey<'a> {
    type Error = NacosError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut split = value.split("+");
        let data_id = if let Some(data) = split.next() {
            data
        } else {
            return Err(NacosError::msg("data id parse error"));
        };
        let group_id = if let Some(data) = split.next() {
            data
        } else {
            return Err(NacosError::msg("group parse error"));
        };

        GroupKey::new(data_id, group_id, split.next())
    }
}

pub struct SafeAccess<T>
where
    T: Send + 'static,
{
    pub(crate) data: Arc<Mutex<T>>,
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
    use crate::client::ConfigListener;
    use crate::NacosResult;
    use std::collections::HashMap;
    use std::io::BufReader;

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
        const CONFIG_TYPE: &'static str;
        // judge type.
        fn is_responsible_for(&self, ty: String) -> bool;
        /// compare old and new data.
        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
            ty: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>>;

        fn filter_change_data(
            &self,
            old_map: &mut HashMap<String, ConfigChangeItem>,
            new_map: &mut HashMap<String, ConfigChangeItem>,
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
                        old_value: format!("{:?}", val),
                        new_value: format!("{:?}", new_map.get(key).unwrap().clone()),
                        ty: PropertyChangeType::MODIFIED,
                    }
                } else {
                    // only old key exists.
                    cci = ConfigChangeItem {
                        key: key.to_string(),
                        old_value: format!("{:?}", val),
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
                        new_value: format!("{:?}", val),
                        ty: PropertyChangeType::ADDED,
                    };

                    result.insert(key.to_string(), cci);
                }
            }
            result
        }
    }

    pub struct PropertiesConfigChangeParse;
    pub struct YamlConfigChangeParse;

    impl ConfigChangeParser for PropertiesConfigChangeParse {
        const CONFIG_TYPE: &'static str = "PROPERTIES";
        fn is_responsible_for(&self, ty: String) -> bool {
            ConfigChangeParser::CONFIG_TYPE.eq_ignore_ascii_case(ty.as_str())
        }

        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
            ty: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>> {
            let mut old_map = HashMap::new();
            let mut new_map = HashMap::new();
            if !old_content.is_empty() {}
            todo!()
        }
    }

    impl ConfigChangeParser for YamlConfigChangeParse {
        const CONFIG_TYPE: &'static str = "YAML";
        fn is_responsible_for(&self, ty: String) -> bool {
            ConfigChangeParser::CONFIG_TYPE.eq_ignore_ascii_case(ty.as_str())
        }

        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
            ty: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>> {
            todo!()
        }
    }
}
