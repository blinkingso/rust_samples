use crate::client::SafeAccess;
use crate::{NacosError, NacosResult};
use log::{debug, error, info, warn};
use reqwest::{Error, Response, StatusCode};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

pub const LOGIN_URL: &'static str = "/v1/auth/users/login";
pub const HTTP_PREFIX: &'static str = "http";

#[derive(Debug)]
pub struct SecurityProxy {
    pub username: String,
    pub password: String,
    access_token: String,
    context_path: String,
    token_ttl: i64,
    last_refresh_time: i64,
    token_refresh_window: i64,
}

impl SecurityProxy {
    /// Check whether Security is enabled or not.
    pub fn enabled(&self) -> bool {
        self.username.is_empty()
    }
}

pub mod login {
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct LoginResponse {
        token_ttl: i64,
        global_admin: bool,
        access_token: String,
    }
    use crate::client::SafeAccess;
    use crate::resp::{RESP_ACCESS_TOKEN, RESP_GLOBAL_ADMIN, RESP_TOKEN_TTL};
    use crate::security::{post_form, SecurityProxy, HTTP_PREFIX, LOGIN_URL};
    use crate::{NacosError, NacosResult};
    use chrono::Utc;
    use log::debug;
    use std::collections::HashMap;

    pub async fn login(
        server_urls: &Vec<String>,
        security: SafeAccess<SecurityProxy>,
    ) -> NacosResult<()> {
        {
            let now = Utc::now().timestamp_millis();
            let lock = security.data.lock().unwrap();
            if now - lock.last_refresh_time < lock.token_ttl - lock.token_refresh_window {
                return Ok(());
            }
        }

        for server in server_urls {
            if let Err(e) = login_server(security.clone(), server).await {
                warn!("login error for: {:?}", e);
            } else {
                return Ok(());
            }
        }
        Err(NacosError::msg("nacos server login failed."))
    }

    async fn login_server(
        security: SafeAccess<SecurityProxy>,
        server_url: &str,
    ) -> NacosResult<()> {
        let mut url;
        let params;
        let body;
        {
            let lock = security.data.lock().unwrap();
            url = format!("http://{}{}{}", server_url, lock.context_path, LOGIN_URL);
            params = [("username", lock.username.clone())];
            body = [("password", lock.password.clone())];
            if server_url.contains(HTTP_PREFIX) {
                url = format!("{}{}{}", server_url, lock.context_path, LOGIN_URL);
            }
        }

        let resp = post_form(url, &params, &body).await?;
        debug!("response string is : {}", resp);
        let result = serde_json::from_str::<LoginResponse>(resp.as_str())?;
        let mut lock = security.data.lock().unwrap();
        lock.access_token = result.access_token;
        lock.token_ttl = result.token_ttl;
        lock.token_refresh_window = lock.token_ttl / 10;
        Ok(())
    }
}

pub async fn post_form(
    url: String,
    params: &[(&'static str, String)],
    body: &[(&'static str, String)],
) -> NacosResult<String> {
    let client = reqwest::ClientBuilder::new()
        .https_only(false)
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(5))
        .no_proxy()
        .gzip(true)
        .build()
        .unwrap();
    debug!("request url : {}", &url);
    let response = client.post(&url).query(&params).form(&body).send().await;
    match response {
        Ok(resp) => {
            let code = resp.status();
            if code.is_success() {
                Ok(resp.text_with_charset("UTF-8").await?)
            } else {
                Err(NacosError::msg(format!(
                    "request to {} error for: {}",
                    &url,
                    code.canonical_reason().unwrap_or("unknown error.")
                )))
            }
        }
        Err(e) => {
            error!("http response error: {:?}", e);
            return Err(NacosError::new(e));
        }
    }
}

#[test]
fn test_req() {
    pretty_env_logger::init();
    let test = async {
        let mut security = SecurityProxy {
            username: "nacos".to_string(),
            password: "nacos".to_string(),
            access_token: "".to_string(),
            context_path: "/nacos".to_string(),
            token_ttl: 0,
            last_refresh_time: 0,
            token_refresh_window: 0,
        };
        let sa = SafeAccess::new(security);
        let res = login::login(&["127.0.0.1:8848".to_string()].to_vec(), sa.clone()).await;
        if res.is_ok() {
            Ok(sa)
        } else {
            Err(NacosError::msg("login error."))
        }
    };

    let result = tokio::runtime::Runtime::new().unwrap().block_on(test);
    match result {
        Ok(flag) => {
            info!("login success: {:?}", flag.data.lock().unwrap());
        }
        Err(e) => {
            warn!("login error for: {:?}", e);
        }
    }
}

pub mod identify {

    /// Credential Listener.
    pub trait CredentialListener {
        /// update Credential
        fn on_update_credential(&mut self);
    }

    /// create a runtime to watch credential.
    pub fn credential_watcher() -> tokio::runtime::Runtime {
        todo!()
    }
}
