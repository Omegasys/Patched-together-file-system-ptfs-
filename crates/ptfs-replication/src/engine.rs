use std::sync::Arc;

use crate::diff::SnapshotDiff;
use crate::scheduler::ReplicationScheduler;

/// Core replication engine:
/// decides WHAT to send and WHERE
pub struct ReplicationEngine {
    scheduler: ReplicationScheduler,
}

impl ReplicationEngine {
    pub fn new() -> Self {
        Self {
            scheduler: ReplicationScheduler::new(),
        }
    }

    /// Replicate a snapshot to all configured targets
    pub fn replicate_snapshot(
        &self,
        snapshot_a: &str,
        snapshot_b: &str,
    ) -> anyhow::Result<()> {
        let diff = SnapshotDiff::compute(snapshot_a, snapshot_b)?;

        if diff.is_empty() {
            return Ok(());
        }

        let targets = self.scheduler.active_targets();

        for target in targets {
            println!(
                "[PTFS-REPL] sending {} changes to {}",
                diff.change_count(),
                target
            );

            // In real system:
            // - serialize diff
            // - send via ptfs-sync
        }

        Ok(())
    }
}
