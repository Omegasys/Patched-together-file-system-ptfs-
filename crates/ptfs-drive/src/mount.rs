use std::collections::HashMap;

#[derive(Debug)]
pub struct MountManager {
    mounts: HashMap<String, u128>, // mount path → drive id
}

impl MountManager {
    pub fn new() -> Self {
        Self {
            mounts: HashMap::new(),
        }
    }

    pub fn mount(&mut self, path: String, drive_id: u128) {
        self.mounts.insert(path, drive_id);
    }

    pub fn unmount(&mut self, path: &str) {
        self.mounts.remove(path);
    }

    pub fn get_drive(&self, path: &str) -> Option<u128> {
        self.mounts.get(path).cloned()
    }

    pub fn list_mounts(&self) -> Vec<(String, u128)> {
        self.mounts.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }
}
