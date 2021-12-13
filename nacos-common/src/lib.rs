/// Nacos configuration annotation used to load configurations from remote nacos server.
/// #[nacos_config(group = "group_name", data_id = "data_id", name = "field name from json", auto_refresh = false, time_interval = "60(unit: seconds, default 60s, valid when auto_fresh = true)")]
/// attr name is only valid on fields.
pub trait NacosConfig {
    /// 1. load from remote server
    /// 2. if 1 is ok -> write data to the local file, else 1 failed -> read from local file
    /// 3. if 1,2 all failed, exit the progress.
    ///
    /// # Returns
    /// * Self: [NacosConfig] configurations.
    fn load() -> Self
    where
        Self: Sized;

    /// Reload configuration's from configuration server if `refresh` enabled.
    /// refresh interval can be set or with default 60s per interval;
    fn refresh(&mut self);

    /// This function used to `refresh` configuration from remote server's push.
    /// # Parameters
    /// * configuration: [String] pushed configuration messages.
    /// # Returns
    /// None
    fn notify(&mut self, configuration: String);

    /// check for auto refreshed
    fn is_auto_refreshed(&self) -> bool;

    /// get interval config
    /// default is 30 * 60
    /// unit is secs.
    fn get_interval_secs(&self) -> u32 {
        30 * 60
    }
}
