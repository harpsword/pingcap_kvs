mod codec;
mod command;
mod file_storage;

use tracing::{debug, info};

use self::{command::Command, file_storage::LogPointer};
use crate::{
    error::CustomizedError,
    kv::command::{Remove, Set},
    result::Result,
};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
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
        let log_pointer = self.file_storage.write_data(byte_data)?;
        self.index.insert(key, log_pointer);
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.index.get(&key) {
            Some(value) => {
                let data = self.file_storage.read_by_log_pointer(value)?;
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
                self.file_storage.write_data(byte_data)?;
                Ok(())
            }
            None => Err(CustomizedError::KeyNotFound),
        }
    }

    fn open(path: impl Into<PathBuf>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut file = BaseFileStorage::open(path)?;
        let mut index = BTreeMap::new();

        // load data from file
        load_file_data_to_index(file.get_reader(), &mut index)?;

        Ok(Self {
            file_storage: file,
            index: index,
            codec: SerdeJsonCodec::new(),
        })
    }
}

/// load the whole log file and get the index
/// todo using codec's function
fn load_file_data_to_index<R: Seek + Read>(
    reader: &mut R,
    index: &mut BTreeMap<String, LogPointer>,
) -> Result<()> {
    reader.seek(SeekFrom::Start(0))?;

    let mut stream = serde_json::Deserializer::from_reader(reader).into_iter::<Command>();

    let mut pos: u64 = 0;
    while let Some(command) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match command? {
            Command::Set(Set { key, .. }) => {
                debug!("key: {:?}, pos: {}, new_pos: {}", &key, pos, new_pos);
                index.insert(
                    key,
                    LogPointer {
                        offset: pos,
                        len: new_pos - pos,
                    },
                );
            }
            Command::Remove(Remove { key }) => {
                index.remove(&key);
            }
        }
        pos = new_pos;
    }
    Ok(())
}
