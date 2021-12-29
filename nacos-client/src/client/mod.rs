use crate::{NacosError, NacosResult};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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

struct CacheData {
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
    is_init: Mutex<bool>,
    is_sync_with_server: Mutex<bool>,
}

trait Listener {}

struct ListenerWrap {
    listener: Box<dyn Listener>,
    last_content: Option<String>,
    last_call_md5: Option<String>,
    is_notifying: bool,
}
