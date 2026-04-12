use std::collections::HashMap;

use blake3::Hash;

/// Maps chunk hash → reference count
#[derive(Debug)]
pub struct DedupIndex {
    map: HashMap<Hash, usize>,
}

impl DedupIndex {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, hash: Hash) -> bool {
        let entry = self.map.entry(hash).or_insert(0);
        *entry += 1;

        // returns true if this is a new chunk
        *entry == 1
    }

    pub fn decrement(&mut self, hash: &Hash) {
        if let Some(count) = self.map.get_mut(hash) {
            *count -= 1;
            if *count == 0 {
                self.map.remove(hash);
            }
        }
    }

    pub fn exists(&self, hash: &Hash) -> bool {
        self.map.contains_key(hash)
    }
}
