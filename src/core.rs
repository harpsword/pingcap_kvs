use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, BufReader};
use std::{collections::HashMap, path::PathBuf};

use crate::Result;
use crate::Command;
use crate::kvs_error::KvsError;

pub struct KvStore {
    store: HashMap<String, String>,

    writer: BufWriter<File>,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let mut store = HashMap::new();

        let path = path.into().join("data");
        let f =  OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?;
        let writer = BufWriter::new(f);

        let read_file = File::open(&path)?;
        let reader = BufReader::new(read_file);
        let mut stream = serde_json::Deserializer::from_reader(reader).into_iter::<Command>();

        for cmd in stream {
            match cmd? {
                Command::Set { key, value} => {
                    store.insert(key, value);
                }
                Command::Remove { key } => {
                    store.remove(&key);
                }
            }
        }
        

        Ok(KvStore {
            store,
            writer,
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()>{
        let command = Command::Set{key: key.clone(), value: value.clone()};

        serde_json::to_writer(&mut self.writer, &command)?;
        self.writer.flush()?;
    
        _ = self.store.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    pub fn remove(&mut self, key: String) -> Result<()>{
        let command = Command::Remove { key: key.clone() };
        serde_json::to_writer(&mut self.writer, &command)?;
        self.writer.flush()?;

        self.store.remove(&key)
            .map_or(Err(KvsError::KeyNotFound), |_| Ok(()))
    }
}
