use crate::backend::{Address, StorageBackend};
use anyhow::Result;
use std::sync::Arc;

// For now, SSD is a thin wrapper over disk with future hooks
pub struct SsdBackend<B: StorageBackend> {
    inner: Arc<B>,
}

impl<B: StorageBackend> SsdBackend<B> {
    pub fn new(inner: Arc<B>) -> Self {
        Self { inner }
    }
}

impl<B: StorageBackend> StorageBackend for SsdBackend<B> {
    fn read(&self, addr: Address, len: usize) -> Result<Vec<u8>> {
        self.inner.read(addr, len)
    }

    fn write(&self, addr: Address, data: &[u8]) -> Result<()> {
        // Future: write coalescing / alignment
        self.inner.write(addr, data)
    }

    fn flush(&self) -> Result<()> {
        self.inner.flush()
    }

    fn size(&self) -> Result<Address> {
        self.inner.size()
    }
}
