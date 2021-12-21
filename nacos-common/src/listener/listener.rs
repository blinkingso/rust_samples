use async_trait::async_trait;
#[async_trait]
trait Listener {
    /// receive configuration from remote server
    async fn receive_config_info(config_info: String);
}
