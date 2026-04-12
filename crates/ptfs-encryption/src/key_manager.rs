use std::collections::HashMap;

/// Key manager (in-memory for now)
pub struct KeyManager {
    keys: HashMap<u128, Vec<u8>>, // object_id → key
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn set_key(&mut self, object_id: u128, key: Vec<u8>) {
        self.keys.insert(object_id, key);
    }

    pub fn get_key(&self, object_id: u128) -> Option<&Vec<u8>> {
        self.keys.get(&object_id)
    }

    pub fn delete_key(&mut self, object_id: u128) {
        self.keys.remove(&object_id);
    }
}
