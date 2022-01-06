use crate::listeners::{Listener, ManagerListenerWrap};
use lazy_static::lazy_static;
use serde::Deserialize;
use state::Storage;
use std::boxed::Box;
use std::sync::RwLock;
use std::time::Instant;

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

lazy_static! {
    static ref CONFIG_LISTENERS: Storage<RwLock<ManagerListenerWrap>> = Storage::new();
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
}

async fn safe_notify_listener(
    data_id: String,
    group: String,
    content: String,
    ty: String,
    md5: String,
    encrypted_data_key: String,
    wrap: ManagerListenerWrap,
) {
    let mut wrap = wrap;
    let mut listener = wrap.listener.clone();
    if wrap.in_notifying {
        warn!("[notify-current-skip] data_id={}, group={}, md5={}, listener={}, listener is not finish yet,will try next time.",
                    data_id, group, md5, "config-change-listener");
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
                    listener.clone(),
                );
            }
        }
    }

    pub fn add_listener(&mut self, listener: ManagerListenerWrap) {
        self.listeners.push(listener);
    }
}
