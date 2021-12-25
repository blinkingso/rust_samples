use crate::{NacosError, NacosResult};
use reqwest::{Error, Response, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use std::ops::Sub;
use std::time::Duration;

pub const LOGIN_URL: &'static str = "/v1/auth/users/login";
pub const HTTP_PREFIX: &'static str = "http";

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
    /// A function login the remote nacos server.
    /// #Arguments
    /// *servers: [Vec<String>], server list.
    /// #Returns
    pub async fn login(&mut self, servers: &Vec<String>) -> NacosResult<()> {
        let now = chrono::Utc::now().timestamp_millis();
        if now - self.last_refresh_time < self.token_ttl - self.token_refresh_window {
            return Ok(());
        }
        for server in servers {
            if self.login_server(server.as_str()).await.is_ok() {
                self.last_refresh_time = chrono::Utc::now().timestamp_millis();
                return Ok(());
            }
        }
        Err(NacosError::msg("nacos server login failed."))
    }

    async fn login_server(&mut self, server: &str) -> NacosResult<()> {
        if !self.username.is_empty() {
            let mut url = format!("http://{}{}{}", server, self.context_path, LOGIN_URL);
            let params = [("username", self.username.as_str())];
            let body = [("password", self.password.as_str())];
            if server.contains(HTTP_PREFIX) {
                url = format!("{}{}{}", server, self.context_path, LOGIN_URL);
            }

            let resp = post_form(url, &params, &body).await?;
            eprintln!("resp is : {}", resp);
            let mut result = serde_json::from_str::<HashMap<String, String>>(resp.as_str())?;
            self.access_token = result.get("accessToken").unwrap().to_string();
            self.token_ttl = result.get("tokenTtl").unwrap().parse::<i64>()?;
            self.token_refresh_window = self.token_ttl / 10;
        }
        Ok(())
    }
}

pub async fn post_form(
    url: String,
    params: &[(&str, &str)],
    body: &[(&str, &str)],
) -> NacosResult<String> {
    let mut client = reqwest::ClientBuilder::new()
        .https_only(false)
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(5))
        .no_proxy()
        .gzip(true)
        .build()
        .unwrap();
    println!("url : {}", &url);
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
            eprintln!("error: {:?}", e);
            return Err(NacosError::new(e));
        }
    }
}

#[test]
fn test_req() {
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
        security
            .login(&["127.0.0.1:8848".to_string()].to_vec())
            .await
    };

    let result = tokio::runtime::Runtime::new().unwrap().block_on(test);
    match result {
        Ok(flag) => {
            println!("login success");
        }
        Err(e) => {
            eprintln!("login error for: {:?}", e);
        }
    }
}
