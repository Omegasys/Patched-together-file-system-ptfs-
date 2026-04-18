use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum InodeType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct Inode {
    pub id: u128,
    pub inode_type: InodeType,
    pub name: String,
    pub parent: Option<u128>,
    pub children: HashMap<String, u128>, // name → inode id
}

impl Inode {
    pub fn new(id: u128, name: String, inode_type: InodeType, parent: Option<u128>) -> Self {
        Self {
            id,
            name,
            inode_type,
            parent,
            children: HashMap::new(),
        }
    }
}
