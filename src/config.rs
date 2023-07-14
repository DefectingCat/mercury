use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{error::MResult, logger::LogLevel};

fn default_port() -> usize {
    4000
}
fn default_log_level() -> LogLevel {
    let env = env::var("LOG_LEVEL").unwrap_or("info".to_owned());
    LogLevel::from(env.as_str())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: usize,
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
}

impl Config {
    pub fn build() -> MResult<Self> {
        let config_path = env::var("CONFIG_PATH").unwrap_or("./config.toml".to_owned());
        let config_path = PathBuf::from(config_path);
        let config = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config)?;

        Ok(config)
    }
}
