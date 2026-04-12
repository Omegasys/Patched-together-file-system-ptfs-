use std::collections::HashMap;
use std::hash::Hash;

/// Generic cache interface
pub trait Cache<K, V> {
    fn get(&mut self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, value: V);
    fn contains(&self, key: &K) -> bool;
}

/// Simple fallback cache (HashMap-based)
pub struct BasicCache<K, V> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash, V> BasicCache<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash, V> Cache<K, V> for BasicCache<K, V> {
    fn get(&mut self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}
