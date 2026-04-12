use std::ops::Range;

/// Represents a contiguous region of storage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extent {
    pub start: u128,
    pub length: u128,
}

impl Extent {
    pub fn new(start: u128, length: u128) -> Self {
        Self { start, length }
    }

    pub fn end(&self) -> u128 {
        self.start + self.length
    }

    pub fn overlaps(&self, other: &Extent) -> bool {
        self.start < other.end() && other.start < self.end()
    }

    pub fn merge(&self, other: &Extent) -> Option<Extent> {
        if self.end() == other.start {
            Some(Extent::new(self.start, self.length + other.length))
        } else if other.end() == self.start {
            Some(Extent::new(other.start, self.length + other.length))
        } else {
            None
        }
    }

    pub fn to_range(&self) -> Range<u128> {
        self.start..self.end()
    }
}
