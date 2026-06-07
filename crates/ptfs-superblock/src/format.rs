use crate::uuid::PtfsUuid;
use uuid::Uuid;

/// Magic header for PTFS disks
pub const PTFS_MAGIC: [u8; 4] = *b"PTFS";
pub const VERSION: u16 = 1;

/// On-disk superblock format
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Superblock {
    pub magic: [u8; 4],
    pub version: u16,

    /// RAID set identifier
    pub raid_id: Uuid,

    /// Disk index within RAID (0..n)
    pub disk_index: u8,

    /// Total expected disks in RAID group
    pub total_disks: u8,

    /// Block size in bytes
    pub block_size: u32,

    /// Feature flags (future-proofing)
    pub flags: u64,
}

impl Superblock {
    pub fn new(raid_id: Uuid, disk_index: u8, total_disks: u8) -> Self {
        Self {
            magic: PTFS_MAGIC,
            version: VERSION,
            raid_id,
            disk_index,
            total_disks,
            block_size: 4096,
            flags: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == PTFS_MAGIC && self.version == VERSION
    }
}
