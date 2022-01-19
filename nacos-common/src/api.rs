use std::collections::HashMap;
use std::env;
pub fn create_config_labels() -> HashMap<String, String> {
    let mut labels = HashMap::new();
    labels.insert(String::from("module"), "config".to_string());
    labels.insert(String::from("source"), "sdk".to_string());
    labels.insert(String::from("taskId"), get_env("TASK_ID", "0"));
    labels.insert(String::from("AppName"), get_env("APP_NAME", "unknown"));
    labels.insert(String::from("Vipserver-Tag"), get_env("VIP_SERVER_TAG", ""));
    labels.insert(String::from("Amory-Tag"), get_env("AMORY_TAG", ""));
    labels
}

pub fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(default.to_string())
}

pub mod ability {
    use serde::Serialize;
    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientRemoteAbility {
        support_remote_connection: bool,
    }

    impl ClientRemoteAbility {
        pub fn is_support_remote_connection(&self) -> bool {
            self.support_remote_connection
        }

        pub fn set_support_remote_connection(&mut self, support_remote_connection: bool) {
            self.support_remote_connection = support_remote_connection;
        }
    }

    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientConfigAbility {
        support_remote_metrics: bool,
    }

    impl ClientConfigAbility {
        pub fn is_support_remote_metrics(&self) -> bool {
            self.support_remote_metrics
        }

        pub fn set_support_remote_metrics(&mut self, support_remote_metrics: bool) {
            self.support_remote_metrics = support_remote_metrics;
        }
    }

    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientNamingAbility {
        support_delta_push: bool,
        support_remote_metric: bool,
    }

    impl ClientNamingAbility {
        pub fn is_support_delta_push(&self) -> bool {
            self.support_delta_push
        }

        pub fn set_support_delta_push(&mut self, support_delta_push: bool) {
            self.support_delta_push = support_delta_push;
        }

        pub fn is_support_remote_metric(&self) -> bool {
            self.support_remote_metric
        }

        pub fn set_support_remote_metric(&mut self, support_remote_metric: bool) {
            self.support_remote_metric = support_remote_metric;
        }
    }

    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientAbilities {
        remote_ability: ClientRemoteAbility,
        config_ability: ClientConfigAbility,
        naming_ability: ClientNamingAbility,
    }

    impl ClientAbilities {
        pub fn get_remote_ability(&self) -> &ClientRemoteAbility {
            &self.remote_ability
        }
        pub fn set_remote_ability(&mut self, remote_ability: ClientRemoteAbility) {
            self.remote_ability = remote_ability;
        }

        pub fn get_config_ability(&self) -> &ClientConfigAbility {
            &self.config_ability
        }
        pub fn set_config_ability(&mut self, config_ability: ClientConfigAbility) {
            self.config_ability = config_ability;
        }

        pub fn get_naming_ability(&self) -> ClientNamingAbility {
            self.naming_ability
        }
        pub fn set_naming_ability(&mut self, naming_ability: ClientNamingAbility) {
            self.naming_ability = naming_ability;
        }
    }

    impl Default for ClientAbilities {
        fn default() -> Self {
            ClientAbilities {
                remote_ability: ClientRemoteAbility {
                    support_remote_connection: true,
                },
                config_ability: ClientConfigAbility {
                    support_remote_metrics: true,
                },
                naming_ability: ClientNamingAbility {
                    support_delta_push: false,
                    support_remote_metric: false,
                },
            }
        }
    }
}
