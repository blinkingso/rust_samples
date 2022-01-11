extern crate config as toml;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate lazy_static;
extern crate state;

use std::collections::HashMap;

mod cache;
pub mod client;
mod common;
mod config;
mod consts;
mod crypto;
mod http;
mod listeners;
mod security;
mod utils;

pub type Properties = HashMap<String, String>;
pub type NacosResult<T> = anyhow::Result<T>;
pub type NacosError = anyhow::Error;
