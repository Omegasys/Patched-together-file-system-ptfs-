#[derive(Debug, Clone)]
pub struct Drive {
    pub id: u128,
    pub name: String,
    pub capacity: u64,
    pub used: u64,
}

impl Drive {
    pub fn new(id: u128, name: String, capacity: u64) -> Self {
        Self {
            id,
            name,
            capacity,
            used: 0,
        }
    }

    pub fn allocate(&mut self, size: u64) -> bool {
        if self.used + size > self.capacity {
            return false;
        }

        self.used += size;
        true
    }

    pub fn free(&mut self, size: u64) {
        self.used = self.used.saturating_sub(size);
    }

    pub fn available(&self) -> u64 {
        self.capacity - self.used
    }
}
