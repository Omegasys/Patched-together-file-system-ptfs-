use std::collections::HashMap;

#[derive(Debug)]
pub struct Namespace {
    pub mounts: HashMap<String, u128>, // mount path → root inode
}

impl Namespace {
    pub fn new() -> Self {
        Self {
            mounts: HashMap::new(),
        }
    }

    pub fn mount(&mut self, path: String, inode_id: u128) {
        self.mounts.insert(path, inode_id);
    }
}
