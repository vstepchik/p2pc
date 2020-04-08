use anyhow::Result;
use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};
use toml;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub log: LogConfig,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self> {
        Ok(toml::from_str(&read_to_string(filename)?)?)
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub url: String,
    pub cert: PathBuf,
    pub key: PathBuf,
    pub redirect_from: Vec<String>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LogConfig {
    pub webapp: String,
}
