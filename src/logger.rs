use serde::{Deserialize, Deserializer, Serialize};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Registry,
};

use crate::{config::Config, error::MResult};

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
impl LogLevel {
    pub fn to_filterd(&self) -> filter::LevelFilter {
        match self {
            LogLevel::INFO => filter::LevelFilter::INFO,
        }
    }
}
impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value {
            "info" => Self::INFO,
            _ => Self::INFO,
        }
    }
}

pub fn logger_init(config: &Config) -> MResult<()> {
    let formattin_layer = fmt::layer()
        .pretty()
        .with_thread_ids(true)
        // .with_target(false)
        .with_file(false)
        .with_writer(std::io::stdout);

    let filter = config.log_level.to_filterd();
    Registry::default()
        .with(filter)
        .with(ErrorLayer::default())
        .with(formattin_layer)
        .init();

    Ok(())
}
