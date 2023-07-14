use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MError {
    // IO error
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    // Toml error
    #[error("Toml deseialize error: {0}")]
    TomlDeError(#[from] toml::de::Error),
}

pub type MResult<T, E = MError> = anyhow::Result<T, E>;
