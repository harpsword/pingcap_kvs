#![deny(missing_docs)]
#![feature(return_position_impl_trait_in_trait)]
//! A simple key/value store.

mod kv;

mod error;
mod result;

pub use kv::KvsEngine;
pub use result::Result;

/// standard kv store
pub type KvStore = kv::KvStore;
