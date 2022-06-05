use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write, Seek, SeekFrom, Read};
use std::{collections::HashMap, path::PathBuf};


use anyhow::Ok as AnyHowOk;

use crate::kvs_error::KvsError;
use crate::Command;
use crate::Result;

struct DataPointer {
    file_id: u32,
    pos: u32,
    len: u32,
}


pub struct KvStore {
    store: HashMap<String, String>,

    pointer_store: HashMap<String, DataPointer>,
    readers: BTreeMap<u32, BufReaderWithPos<File>>,

    writer: BufWriterWithPos<File>,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        std::fs::create_dir_all(path)?;

        let index_list: Vec<u64> = std::fs::read_dir(path)?
            .flat_map(|v| -> Result<_> { Ok(v?.path()) })
            .filter(|v| v.is_file() && v.extension() == Some("log".as_ref()))
            .flat_map(|p| {
                p.file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .map(|s| s.trim_end_matches(".log"))
                    .map(str::parse::<u64>)
            }).flatten()
            .collect();

        let mut store = HashMap::new();
        let mut pointer_store = HashMap::new();
        let mut readers = BTreeMap::new();

        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?;
        let writer = BufWriter::new(f);

        let read_file = File::open(&path)?;
        let reader = BufReader::new(read_file);
        let stream = serde_json::Deserializer::from_reader(reader).into_iter::<Command>();

        for cmd in stream {
            match cmd? {
                Command::Set { key, value } => {
                    store.insert(key, value);
                }
                Command::Remove { key } => {
                    store.remove(&key);
                }
            }
        }

        // Ok(KvStore { store, writer })
        todo!()
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };

        serde_json::to_writer(&mut self.writer, &command)?;
        self.writer.flush()?;

        _ = self.store.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let command = Command::Remove { key: key.clone() };
        let data = serde_json::to_vec(&command)?;

        self.writer.write(&data);


        serde_json::to_writer(&mut self.writer, &command)?;

        self.writer.flush()?;

        self.store
            .remove(&key)
            .map_or(Err(KvsError::KeyNotFound), |_| Ok(()))
    }
}


struct BufWriterWithPos<T: Write + Seek> {
    inner: T,
    pos: u64,
}

impl<T: Write + Seek> BufWriterWithPos<T> {
    pub fn new (mut inner: T) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos { inner, pos})
    }
}

impl<T: Write + Seek> Write for BufWriterWithPos<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.inner.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl <T: Write + Seek> Seek for BufWriterWithPos<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.inner.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufReaderWithPos<T: Read + Seek> {
    inner: T,
    pos: u64,
}

impl<T: Read+Seek> BufReaderWithPos<T> {
    pub fn new(mut inner: T) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos{inner, pos})
    }
}

impl<T: Read+Seek> Read for BufReaderWithPos<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.inner.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<T: Read+Seek> Seek for BufReaderWithPos<T> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.inner.seek(pos)?;
        Ok(self.pos)
    }
}
