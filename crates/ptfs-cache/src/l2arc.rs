use std::collections::VecDeque;

/// L2ARC (secondary cache, e.g., SSD)
pub struct L2Arc {
    storage: VecDeque<Vec<u8>>,
    capacity: usize,
}

impl L2Arc {
    pub fn new(capacity: usize) -> Self {
        Self {
            storage: VecDeque::new(),
            capacity,
        }
    }

    pub fn insert(&mut self, data: Vec<u8>) {
        if self.storage.len() >= self.capacity {
            self.storage.pop_back();
        }
        self.storage.push_front(data);
    }

    pub fn fetch(&self, index: usize) -> Option<&Vec<u8>> {
        self.storage.get(index)
    }
}
