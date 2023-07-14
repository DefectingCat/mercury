use thiserror::Error;

#[derive(Error, Debug)]
pub enum MError {}

pub type MResult<T, E = MError> = anyhow::Result<T, E>;
