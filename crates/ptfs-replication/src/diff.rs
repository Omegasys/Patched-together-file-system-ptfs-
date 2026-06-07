/// Snapshot diffing system (core of incremental backup)

#[derive(Debug)]
pub struct SnapshotDiff {
    pub changed_blocks: Vec<String>,
}

impl SnapshotDiff {
    /// Compare two snapshots (A → B)
    pub fn compute(snapshot_a: &str, snapshot_b: &str) -> anyhow::Result<Self> {
        // Placeholder logic:
        // Real system would:
        // - compare Merkle trees (ptfs-integrity)
        // - detect changed object hashes

        if snapshot_a == snapshot_b {
            return Ok(Self {
                changed_blocks: vec![],
            });
        }

        Ok(Self {
            changed_blocks: vec![
                format!("diff:{}→{}", snapshot_a, snapshot_b),
            ],
        })
    }

    pub fn is_empty(&self) -> bool {
        self.changed_blocks.is_empty()
    }

    pub fn change_count(&self) -> usize {
        self.changed_blocks.len()
    }
}
