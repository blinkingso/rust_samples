//! util access to nacos server.
use crate::property_key_const::*;
use crate::Properties;

/// util access to get configurations from remote server.
pub struct NacosConfigService(Properties);

impl NacosConfigService {
    pub fn init_namespace(&mut self) {
        if self.0.get(NAMESPACE).is_none() {
            // set default namespace to public
            self.0
                .insert(NAMESPACE.to_string(), DEFAULT_NAMESPACE.to_string());
        }
    }
}
