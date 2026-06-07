pub mod format;
pub mod reader;
pub mod writer;
pub mod uuid;

use uuid::Uuid;

pub use format::Superblock;
pub use uuid::PtfsUuid;

/// High-level superblock API entry point
pub struct SuperblockManager;

impl SuperblockManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_new(raid_id: Uuid, disk_index: u8, total_disks: u8) -> Superblock {
        Superblock::new(raid_id, disk_index, total_disks)
    }
}
