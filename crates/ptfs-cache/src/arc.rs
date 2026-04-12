use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Simplified ARC cache (not full adaptive yet)
pub struct ArcCache<K, V> {
    capacity: usize,

    t1: VecDeque<K>, // recent
    t2: VecDeque<K>, // frequent

    map: HashMap<K, V>,
}

impl<K: Clone + Eq + Hash, V> ArcCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            t1: VecDeque::new(),
            t2: VecDeque::new(),
            map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // promote to T2
            self.t1.retain(|k| k != key);
            if !self.t2.contains(key) {
                self.t2.push_front(key.clone());
            }
            return self.map.get(key);
        }
        None
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity {
            self.evict();
        }

        self.t1.push_front(key.clone());
        self.map.insert(key, value);
    }

    fn evict(&mut self) {
        if let Some(old) = self.t1.pop_back().or_else(|| self.t2.pop_back()) {
            self.map.remove(&old);
        }
    }
}
