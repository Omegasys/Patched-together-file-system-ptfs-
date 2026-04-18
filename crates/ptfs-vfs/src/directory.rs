use crate::inode::{Inode, InodeType};

impl Inode {
    pub fn add_child(&mut self, name: String, inode_id: u128) {
        if let InodeType::Directory = self.inode_type {
            self.children.insert(name, inode_id);
        }
    }
}
