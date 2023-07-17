use filter::LevelFilter;
use serde::{Deserialize, Deserializer, Serialize};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Registry,
};

use crate::{config::Config, error::MResult};

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    INFO,
    DEBUG,
}
impl Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string())
    }
}
impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(LogLevel::from(s.as_str()))
    }
}
impl LogLevel {
    pub fn to_filterd(&self) -> LevelFilter {
        use LogLevel::*;
        match self {
            INFO => LevelFilter::INFO,
            DEBUG => LevelFilter::DEBUG,
        }
    }
    pub fn to_string(&self) -> &str {
        use LogLevel::*;
        match self {
            INFO => "info",
            DEBUG => "debug",
        }
    }
}
impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value {
            "info" => Self::INFO,
            "debug" => Self::DEBUG,
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

#[cfg(test)]
mod tests {
    use super::LogLevel;

    #[test]
    fn from_test() {
        let level = "info";
        let info = LogLevel::from(level);
        assert_eq!(LogLevel::INFO, info);

        let level = "debug";
        let info = LogLevel::from(level);
        assert_eq!(LogLevel::DEBUG, info);

        let level = "info";
        let info = LogLevel::from(level);
        assert_ne!(LogLevel::DEBUG, info);
    }
}
