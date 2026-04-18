#[derive(Debug, Clone)]
pub struct Partition {
    pub id: u128,
    pub start: u64,
    pub size: u64,
    pub label: String,
}

impl Partition {
    pub fn new(id: u128, start: u64, size: u64, label: String) -> Self {
        Self {
            id,
            start,
            size,
            label,
        }
    }

    pub fn end(&self) -> u64 {
        self.start + self.size
    }
}
