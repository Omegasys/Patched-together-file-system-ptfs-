use crate::backend::{Address, StorageBackend};
use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

pub struct DiskBackend {
    file: Mutex<File>,
}

impl DiskBackend {
    pub fn open(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        Ok(Self {
            file: Mutex::new(file),
        })
    }
}

impl StorageBackend for DiskBackend {
    fn read(&self, addr: Address, len: usize) -> Result<Vec<u8>> {
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(addr as u64))?;
        let mut buf = vec![0u8; len];
        file.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn write(&self, addr: Address, data: &[u8]) -> Result<()> {
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(addr as u64))?;
        file.write_all(data)?;
}
