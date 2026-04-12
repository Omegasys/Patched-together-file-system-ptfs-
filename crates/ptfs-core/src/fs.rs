use crate::inode::Inode;
use crate::object::Object;
use crate::transaction::Transaction;
use anyhow::Result;
use std::collections::HashMap;

pub struct Filesystem {
    pub inodes: HashMap<u128, Inode>,
}

impl Filesystem {
    pub fn new() -> Self {
        Self {
            inodes: HashMap::new(),
        }
    }

    pub fn create_file(&mut self, id: u128) -> Result<()> {
        let inode = Inode::new(id);
        self.inodes.insert(id, inode);
        Ok(())
    }

    pub fn write(&mut self, id: u128, data: Vec<u8>) -> Result<()> {
        let inode = self.inodes.get_mut(&id).ok_or_else(|| anyhow::anyhow!("inode not found"))?;
        inode.write(data)?;
        Ok(())
    }

    pub fn read(&self, id: u128) -> Result<Vec<u8>> {
        let inode = self.inodes.get(&id).ok_or_else(|| anyhow::anyhow!("inode not found"))?;
        inode.read()
    }

    pub fn begin_transaction(&self) -> Transaction {
        Transaction::new()
}
