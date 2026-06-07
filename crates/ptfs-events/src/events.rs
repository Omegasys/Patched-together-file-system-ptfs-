use std::time::{SystemTime};

/// Core system-wide events for PTFS
#[derive(Debug, Clone)]
pub enum PtfsEvent {
    // Device layer
    DiskInserted {
        name: String,
        path: String,
    },
    DiskRemoved {
        name: String,
        path: String,
    },

    // Superblock / RAID identity
    SuperblockDetected {
        raid_id: String,
        disk_index: u8,
        total_disks: u8,
        device: String,
    },

    RaidFormed {
        raid_id: String,
    },

    RaidDegraded {
        raid_id: String,
        missing_disks: u8,
    },

    RaidRecovered {
        raid_id: String,
    },

    // Filesystem / mount layer
    MountRequested {
        target: String,
    },

    Mounted {
        target: String,
        mount_point: String,
    },

    UnmountRequested {
        mount_point: String,
    },

    // Maintenance / background ops
    RebuildRequested {
        raid_id: String,
    },

    ScrubRequested {
        raid_id: String,
    },

    ChecksumFailure {
        location: String,
    },

    // Generic system heartbeat
    Tick {
        timestamp: SystemTime,
    },
}
