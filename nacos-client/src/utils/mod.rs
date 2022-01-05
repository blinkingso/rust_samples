use std::collections::HashMap;
use std::io::BufRead;
use crate::config::props::NacosConfigProperties;
use crate::{NacosError, NacosResult};
use serde::Deserialize;
use yaml_config::{Config, Environment, File};

pub mod crypto;

/// read config from a file.
pub fn read_toml_from_resources<'de, T: Deserialize<'de>>(prefix: &str) -> NacosResult<T> {
    let mut s = Config::default();
    let default = format!("resources/{}.toml", prefix);
    s.merge(File::with_name(default.as_str()))?;
    // config environment conf file.
    let env = std::env::var("RUN_MODE").unwrap_or(String::from("dev"));
    let file_name = format!("resources/{}-{}.toml", prefix, env);
    s.merge(File::with_name(file_name.as_str()))?;
    // from environment
    s.merge(Environment::with_prefix(prefix))?;
    Ok(s.try_into()?)
}

/// A function to parse properties file text
pub fn parse_properties<R: BufRead>(reader: R) -> NacosResult<HashMap<String, String>> {
    Ok(reader.lines().filter_map(|line| {
        let line = match line {
          Ok(l) => l,
            Err(e) => {
                warn!("line parse error for: {:?}", e);
                return None;
            }
        };
        if line.starts_with("#") || line.is_empty() {
            return None;
        }
        let (key ,value) = line.split_once("=")?;
        Some((key.trim().to_string(), value.trim().to_string()))
    }).collect())
}

#[test]
fn test_read() {
    let a = read_toml_from_resources::<NacosConfigProperties>("nacos");
    if let Ok(ref p) = a {
        println!("{:?}", p);
    } else {
        eprintln!("error: {:?}", a.unwrap_err());
    }
}

#[test]
fn test_parse_properties() {
    let text = "#test for hello \r\nhello=world";
    let m = parse_properties(text.as_bytes());
    match m {
        Ok(map) => {
            println!("parse result is : {:?}", map);
        },
        Err(error) => {
            eprintln!("parse error for: {:?}", error);
        }
    }
}