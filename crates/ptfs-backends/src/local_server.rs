use crate::trait_::StorageBackend;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// NAS / LAN storage backend
pub struct LocalServerBackend {
    pub root: PathBuf,
}

impl LocalServerBackend {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn path(&self, key: &str) -> PathBuf {
        self.root.join(key)
    }
}

impl StorageBackend for LocalServerBackend {
    fn name(&self) -> &str {
        "local_server"
    }

    fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<()> {
        let path = self.path(key);

        println!("[PTFS-BACKEND] writing to local server: {:?}", path);

        let mut file = File::create(path)?;
        file.write_all(data)?;

        Ok(())
    }

    fn download(&self, key: &str) -> anyhow::Result<Vec<u8>> {
        let path = self.path(key);

        println!("[PTFS-BACKEND] reading from local server: {:?}", path);

        Ok(std::fs::read(path)?)
    }

    fn health_check(&self) -> bool {
        self.root.exists()
    }
}
