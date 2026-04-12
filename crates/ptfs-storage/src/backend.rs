use anyhow::Result;

pub type Address = u128;

pub trait StorageBackend: Send + Sync {
    fn read(&self, addr: Address, len: usize) -> Result<Vec<u8>>;
    fn write(&self, addr: Address, data: &[u8]) -> Result<()>;
    fn flush(&self) -> Result<()>;
    fn size(&self) -> Result<Address>;
}
