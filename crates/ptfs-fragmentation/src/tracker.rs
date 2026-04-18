use std::collections::HashMap;
use crate::metrics::FragmentationMetrics;

#[derive(Debug, Clone)]
pub struct FragmentRecord {
    pub extents: Vec<(u64, u64)>,
    pub metrics: FragmentationMetrics,
}

pub struct FragmentationTracker {
    history: HashMap<u128, Vec<FragmentRecord>>, // object/file id → history
}

impl FragmentationTracker {
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }

    pub fn track(&mut self, object_id: u128, extents: Vec<(u64, u64)>) {
        let metrics = FragmentationMetrics::from_extents(&extents);

        let record = FragmentRecord {
            extents,
            metrics,
        };

        self.history
            .entry(object_id)
            .or_insert_with(Vec::new)
            .push(record);
    }

    pub fn latest(&self, object_id: u128) -> Option<&FragmentRecord> {
        self.history.get(&object_id)?.last()
    }

    pub fn history(&self, object_id: u128) -> Option<&Vec<FragmentRecord>> {
        self.history.get(&object_id)
    }

    pub fn is_fragmented(&self, object_id: u128, threshold: f64) -> bool {
        match self.latest(object_id) {
            Some(record) => record.metrics.is_fragmented(threshold),
            None => false,
        }
    }
}
