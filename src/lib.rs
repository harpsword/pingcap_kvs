
use anyhow::{Result as AnyHowResult};

mod kvs_error;
mod core;
mod command;

pub type Result<T> = AnyHowResult<T, kvs_error::KvsError>;
pub use crate::core::KvStore;
pub use crate::command::Command;
pub use crate::kvs_error::KvsError;