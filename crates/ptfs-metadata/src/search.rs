use crate::metadata::Metadata;
use crate::index::MetadataIndex;

/// Simple search layer (expand later)
pub struct MetadataSearch<'a> {
    index: &'a MetadataIndex,
}

impl<'a> MetadataSearch<'a> {
    pub fn new(index: &'a MetadataIndex) -> Self {
        Self { index }
    }

    /// Search by size range
    pub fn by_size(&self, min: u64, max: u64) -> Vec<&Metadata> {
        self.index
            .all()
            .into_iter()
            .filter(|m| m.size >= min && m.size <= max)
            .collect()
    }

    /// Search by time range
    pub fn by_modified_after(&self, timestamp: u64) -> Vec<&Metadata> {
        self.index
            .all()
            .into_iter()
            .filter(|m| m.modified >= timestamp)
            .collect()
    }
}
