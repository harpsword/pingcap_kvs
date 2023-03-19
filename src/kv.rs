mod codec;
mod command;
mod file_storage;

use self::command::Command;
use crate::{error::CustomizedError, result::Result};
use std::{
    collections::{BTreeMap, HashMap},
    io::Read,
    path::PathBuf,
};

/// export
pub use crate::kv::codec::SerdeJsonCodec;
pub use crate::kv::file_storage::BaseFileStorage;

/// KvStorage
pub trait KvStorage {
    /// asdf
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// get
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// remove
    fn remove(&mut self, key: String) -> Result<()>;

    /// open a kv storage
    fn open(path: impl Into<PathBuf>) -> Result<Self>
    where
        Self: Sized;
}

/// BaseKvStore is a kvs store
pub struct BaseKvStore {
    file_storage: BaseFileStorage,
    // key -> log pointer
    index: BTreeMap<String, file_storage::LogPointer>,

    codec: SerdeJsonCodec,
}

impl KvStorage for BaseKvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set(command::Set {
            key: key.clone(),
            value,
        });
        let byte_data = self.codec.encode(vec![command])?;
        let log_pointer = self.file_storage.write(byte_data)?;
        self.index.insert(key, log_pointer);
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.index.get(&key) {
            Some(value) => {
                let data = self.file_storage.read(value)?;
                let command = self.codec.decode(data)?;
                match command {
                    Command::Set(set) => Ok(Some(set.value)),
                    Command::Remove(_) => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    fn remove(&mut self, key: String) -> Result<()> {
        match self.index.remove(&key) {
            Some(_) => {
                let command = Command::Remove(command::Remove { key });
                let byte_data = self.codec.encode(vec![command])?;
                self.file_storage.write(byte_data)?;
                Ok(())
            }
            None => Err(CustomizedError::KeyNotFound),
        }
    }

    fn open(path: impl Into<PathBuf>) -> Result<Self>
    where
        Self: Sized,
    {
        // load data from file
        let file = BaseFileStorage::open(path)?;

        Ok(Self {
            file_storage: file,
            index: BTreeMap::new(),
            codec: SerdeJsonCodec::new(),
        })
    }
}
