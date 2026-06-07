use crate::queue::SyncQueue;

/// Sends data to remote backends
pub struct SyncPusher {
    queue: SyncQueue,
}

impl SyncPusher {
    pub fn new(queue: SyncQueue) -> Self {
        Self { queue }
    }

    /// Push all pending changes to remote
    pub fn push_all(&mut self) -> anyhow::Result<()> {
        while let Some(item) = self.queue.pop() {
            println!(
                "[PTFS-SYNC] pushing {} to {}",
                item.payload_id, item.target
            );

            // Real system would:
            // - serialize block/diff
            // - send via ptfs-transport
            // - verify checksum

            self.queue.mark_sent(&item.payload_id);
        }

        Ok(())
    }
}
