use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use serde::{Deserialize, Deserializer, Serialize};

use crate::error::MResult;

fn default_port() -> usize {
    4000
}
fn default_log_level() -> LogLevel {
    LogLevel::INFO
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    INFO,
}
impl Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            LogLevel::INFO => "info",
        })
    }
}
impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "info" => LogLevel::INFO,
            _ => LogLevel::INFO,
        })
    }
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
