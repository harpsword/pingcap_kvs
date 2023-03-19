use tracing::info;

use crate::Result;
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::{
    fs::File,
    io::{BufReader, Read, Seek, Write},
    path::PathBuf,
};

pub struct LogPointer {
    pub offset: u64,
    pub len: u64,
}

// pub trait FileStorage {
//     fn open(path: impl Into<PathBuf>) -> Result<Self>
//     where
//         Self: Sized;

//     fn write(&mut self, data: Vec<u8>) -> Result<LogPointer>;

//     fn read(&mut self, log_pointer: &LogPointer) -> Result<Vec<u8>>;
// }

pub struct BaseFileStorage {
    folder_path: PathBuf,
    reader: BufReaderWithPos<File>,
    writer: BufWriterWithPos<File>,
}

impl BaseFileStorage {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let folder_path = path.into();
        let file_path = folder_path.join("1.data");
        std::fs::create_dir_all(&folder_path)?;

        info!("file_path: {:?}", &file_path);

        let write_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&file_path)?;
        Ok(Self {
            reader: BufReaderWithPos::new(File::open(&file_path)?)?,
            // todo fix it, using writable file open option
            writer: BufWriterWithPos::new(write_file)?,
            folder_path: folder_path,
        })
    }

    pub fn write(&mut self, data: Vec<u8>) -> Result<LogPointer> {
        let len = self.writer.write(&data)?;
        Ok(LogPointer {
            offset: self.writer.pos,
            len: len as u64,
        })
    }

    pub fn read(&mut self, log_pointer: &LogPointer) -> Result<Vec<u8>> {
        if self.reader.pos != log_pointer.offset {
            self.reader.seek(SeekFrom::Start(log_pointer.offset))?;
        }
        let mut buffer = vec![0; log_pointer.len as usize];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}

pub struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pub pos: u64,
}

impl<R> BufReaderWithPos<R>
where
    R: Read + Seek,
{
    pub fn new(mut inner: R) -> Result<Self> {
        let current_pos = inner.seek(SeekFrom::Current(0))?;

        Ok(Self {
            reader: BufReader::new(inner),
            pos: current_pos,
        })
    }
}

impl<R> Read for BufReaderWithPos<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.reader.read(buf)?;
        self.pos += size as u64;
        Ok(size)
    }
}

impl<R> Seek for BufReaderWithPos<R>
where
    R: Read + Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let current_pos = self.reader.seek(pos)?;
        self.pos = current_pos;
        Ok(current_pos)
    }
}

pub struct BufWriterWithPos<W>
where
    W: Write + Seek,
{
    writer: W,
    pos: u64,
}

impl<W> BufWriterWithPos<W>
where
    W: Write + Seek,
{
    pub fn new(mut inner: W) -> Result<Self> {
        let current_pos = inner.seek(SeekFrom::Current(0))?;
        Ok(Self {
            writer: inner,
            pos: current_pos,
        })
    }
}

impl<W> Write for BufWriterWithPos<W>
where
    W: Write + Seek,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let size = self.writer.write(buf)?;
        self.pos += size as u64;
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W> Seek for BufWriterWithPos<W>
where
    W: Write + Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let current_pos = self.writer.seek(pos)?;
        self.pos = current_pos;
        Ok(current_pos)
    }
}
