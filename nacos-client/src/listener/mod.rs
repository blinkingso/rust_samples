use crate::common::GroupKey;
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Subscription(GroupKey);
impl Subscription {
    fn new(group_key: GroupKey) -> Self {
        Subscription(group_key)
    }
}

type ListenerMap<T> = HashMap<Subscription, Box<dyn Fn(T) + Send + 'static>>;

/// Safety access to listeners, etc...
#[derive(Clone)]
pub struct ListenerSet<T>
where
    T: Send,
{
    listeners: Arc<Mutex<ListenerMap<T>>>,
}

impl<T> ListenerSet<T>
where
    T: Send + Clone,
{
    pub fn new() -> Self {
        ListenerSet {
            listeners: Arc::new(Mutex::new(ListenerMap::new())),
        }
    }

    /// create a new subscribe.
    pub fn subscribe<Listener: Fn(T) + Send + 'static>(
        &self,
        group_key: GroupKey,
        listener: Listener,
    ) -> Subscription {
        let mut lock = self.listeners.lock().unwrap();
        let subscription = Subscription::new(group_key);
        lock.insert(subscription.clone(), Box::new(listener));
        subscription
    }

    /// remove listener from listeners.
    pub fn unsubscribe(&self, subscription: Subscription) {
        let mut lock = self.listeners.lock().unwrap();
        lock.remove(&subscription);
    }

    pub fn notify(&self, payload: &T) {
        let listeners = self.listeners.lock().unwrap();
        for listener in listeners.values() {
            listener(payload.clone())
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.listeners.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::ListenerSet;
    use crate::common::GroupKey;
    use std::sync::mpsc;

    #[test]
    fn test_new_listener_set() {
        let ls = ListenerSet::<()>::new();
        assert_eq!(ls.len(), 0);
    }

    #[test]
    fn test_new_listener_for_chan() {
        let ls = ListenerSet::<bool>::new();
        let gk = GroupKey::new_without_tanant("001", "pay").unwrap();
        ls.subscribe(gk, |data| {});
        assert_eq!(ls.len(), 1);
    }

    #[test]
    fn test_add_listener_to_set() {
        let (tx, rx) = mpsc::channel();
        let ls = ListenerSet::<bool>::new();
        let gk = GroupKey::new_without_tanant("003", "pay").unwrap();
        ls.subscribe(gk, move |e| tx.send(e).unwrap());
        assert_eq!(ls.len(), 1);

        ls.notify(&true);
        assert!(rx.recv().is_ok());
    }

    #[test]
    fn test_remove_listener_from_set() {
        let (tx, rx) = mpsc::channel();
        let ls = ListenerSet::<bool>::new();
        let gk = GroupKey::new_without_tanant("003", "pay").unwrap();
        let sub = ls.subscribe(gk, move |e| tx.send(e).unwrap());
        ls.unsubscribe(sub);
        assert_eq!(ls.len(), 0);
        ls.notify(&true);
        assert!(rx.recv().is_err());
    }
}
//! Listener mod and some pojo structs in base.

use crate::Properties;

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

/// Listener for watch config.
pub trait Listener {

    /// Receive config info.
    /// #Parameters
    /// * config_info [String] config info.
    /// #Returns
    /// Nothing.
    fn receive_config_info(&self, config_info: String);
}

/// ConfigListener for watch config change event.
pub trait ConfigListener: Listener {

    /// Receive ConfigChangeEvent
    /// #Parameters
    /// * event [ConfigChangeEvent] config change event.
    /// #Returns
    /// Nothing.
    fn receive_config_event(&self, event: ConfigChangeEvent);
}

/// Properties Listener
pub trait PropertiesListener: Listener {

    fn receive_config_info(config_info: String) {
        let properties = Properties::new();
        for line in config_info.lines() {

        }
    }
}