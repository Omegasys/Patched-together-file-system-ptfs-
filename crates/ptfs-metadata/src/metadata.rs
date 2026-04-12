use std::time::{SystemTime, UNIX_EPOCH};

/// Core metadata structure (like inode-lite)
#[derive(Debug, Clone)]
pub struct Metadata {
    pub object_id: u128,
    pub size: u64,
    pub created: u64,
    pub modified: u64,
    pub checksum: [u8; 32],
}

impl Metadata {
    pub fn new(object_id: u128, size: u64, checksum: [u8; 32]) -> Self {
        let now = current_time();

        Self {
            object_id,
            size,
            created: now,
            modified: now,
            checksum,
        }
    }

    pub fn update_modified(&mut self) {
        self.modified = current_time();
    }
}

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
