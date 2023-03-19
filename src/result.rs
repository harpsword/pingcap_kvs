use std::result::Result as StdResult;

use crate::error::CustomizedError;

/// Costomized Result
pub type Result<T> = StdResult<T, CustomizedError>;
