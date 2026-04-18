use std::collections::HashMap;

use crate::inode::{Inode, InodeType};
use crate::path::split_path;

pub struct Vfs {
    pub inodes: HashMap<u128, Inode>,
    pub root: u128,
    next_inode: u128,
}

impl Vfs {
    pub fn new() -> Self {
        let mut inodes = HashMap::new();

        let root_inode = Inode::new(0, "/".into(), InodeType::Directory, None);
        inodes.insert(0, root_inode);

        Self {
            inodes,
            root: 0,
            next_inode: 1,
        }
    }

    fn allocate_inode(&mut self) -> u128 {
        let id = self.next_inode;
        self.next_inode += 1;
        id
    }

    pub fn create_dir(&mut self, path: &str) -> Result<(), String> {
        let parts = split_path(path);
        let mut current = self.root;

        for part in parts {
            let exists = self.inodes[&current].children.get(part).cloned();

            current = match exists {
                Some(id) => id,
                None => {
                    let new_id = self.allocate_inode();
                    let inode = Inode::new(new_id, part.into(), InodeType::Directory, Some(current));

                    self.inodes.insert(new_id, inode);
                    self.inodes.get_mut(&current).unwrap().add_child(part.into(), new_id);

                    new_id
                }
            };
        }

        Ok(())
    }

    pub fn create_file(&mut self, path: &str) -> Result<u128, String> {
        let mut parts = split_path(path);

        if parts.is_empty() {
            return Err("Invalid path".into());
        }

        let file_name = parts.pop().unwrap();
        let mut current = self.root;

        for part in parts {
            current = match self.inodes[&current].children.get(part) {
                Some(id) => *id,
                None => return Err("Directory does not exist".into()),
            };
        }

        let inode_id = self.allocate_inode();
        let inode = Inode::new(inode_id, file_name.into(), InodeType::File, Some(current));

        self.inodes.insert(inode_id, inode);
        self.inodes.get_mut(&current).unwrap().add_child(file_name.into(), inode_id);

        Ok(inode_id)
    }

    pub fn resolve_path(&self, path: &str) -> Option<u128> {
        let parts = split_path(path);
        let mut current = self.root;

        for part in parts {
            match self.inodes.get(&current)?.children.get(part) {
                Some(id) => current = *id,
                None => return None,
            }
        }

        Some(current)
    }
}
