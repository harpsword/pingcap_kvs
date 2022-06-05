use anyhow::Result as AnyHowResult;

mod command;
mod core;
mod kvs_error;

pub type Result<T> = AnyHowResult<T, kvs_error::KvsError>;
pub use crate::command::Command;
pub use crate::core::KvStore;
pub use crate::kvs_error::KvsError;
