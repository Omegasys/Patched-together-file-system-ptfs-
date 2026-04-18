use std::collections::HashMap;

use ptfs_vfs::Vfs;

use crate::object_store::{ObjectStore, ObjectId};
use crate::transaction::Transaction;

pub struct Ptfs {
    pub vfs: Vfs,
    pub object_store: ObjectStore,

    // inode → object mapping
    pub inode_objects: HashMap<u128, ObjectId>,
}

impl Ptfs {
    pub fn new() -> Self {
        Self {
            vfs: Vfs::new(),
            object_store: ObjectStore::new(),
            inode_objects: HashMap::new(),
        }
    }

    pub fn mkfs(&mut self) {
        // reset filesystem
        *self = Self::new();
    }

    pub fn mount(&mut self) {
        // placeholder (future: connect to devices)
    }

    pub fn write_file(&mut self, path: &str, data: Vec<u8>) -> Result<(), String> {
        let mut tx = Transaction::begin();

        let inode = match self.vfs.resolve_path(path) {
            Some(id) => id,
            None => {
                // create file if it doesn't exist
                self.vfs.create_file(path)?
            }
        };

        let obj_id = self.object_store.write_object(data);

        self.inode_objects.insert(inode, obj_id);

        tx.log(format!("write_file inode={} obj={}", inode, obj_id));
        tx.commit();

        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Option<Vec<u8>> {
        let inode = self.vfs.resolve_path(path)?;
        let obj_id = self.inode_objects.get(&inode)?;

        let obj = self.object_store.read_object(obj_id)?;
        Some(obj.data.clone())
    }

    pub fn delete_file(&mut self, path: &str) -> Result<(), String> {
        let mut tx = Transaction::begin();

        let inode = self.vfs.resolve_path(path)
            .ok_or("File not found")?;

        if let Some(obj_id) = self.inode_objects.remove(&inode) {
            self.object_store.delete_object(&obj_id);
        }

        tx.log(format!("delete_file inode={}", inode));
        tx.commit();

        Ok(())
    }

    pub fn verify_file(&self, path: &str) -> bool {
        match self.vfs.resolve_path(path) {
            Some(inode) => {
                if let Some(obj_id) = self.inode_objects.get(&inode) {
                    return self.object_store.verify(obj_id);
                }
                false
            }
            None => false,
        }
    }
}
