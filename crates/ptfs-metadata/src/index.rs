use std::collections::HashMap;

use crate::metadata::Metadata;

/// Metadata index (object_id → metadata)
pub struct MetadataIndex {
    map: HashMap<u128, Metadata>,
}

impl MetadataIndex {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, metadata: Metadata) {
        self.map.insert(metadata.object_id, metadata);
    }

    pub fn get(&self, object_id: &u128) -> Option<&Metadata> {
        self.map.get(object_id)
    }

    pub fn remove(&mut self, object_id: &u128) {
        self.map.remove(object_id);
    }

    pub fn all(&self) -> Vec<&Metadata> {
        self.map.values().collect()
    }
}
