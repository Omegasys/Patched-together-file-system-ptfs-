#[derive(Debug)]
pub struct FileHandle {
    pub inode_id: u128,
    pub offset: usize,
}

impl FileHandle {
    pub fn new(inode_id: u128) -> Self {
        Self { inode_id, offset: 0 }
    }
}
