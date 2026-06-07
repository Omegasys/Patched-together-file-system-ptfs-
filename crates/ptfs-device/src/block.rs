use std::path::PathBuf;

/// Represents a raw block device in PTFS
#[derive(Debug, Clone)]
pub struct BlockDevice {
    pub path: PathBuf,
    pub name: String,
    pub size_bytes: u64,
    pub sector_size: u64,
    pub model: Option<String>,
    pub serial: Option<String>,
}

impl BlockDevice {
    pub fn new(path: PathBuf, name: String) -> Self {
        Self {
            path,
            name,
            size_bytes: 0,
            sector_size: 512,
            model: None,
            serial: None,
        }
    }

    /// Placeholder: read device metadata (later: ioctl/sysfs)
    pub fn probe(&mut self) -> anyhow::Result<()> {
        // In real system:
        // - read /sys/block/*
        // - ioctl BLKGETSIZE64
        // - udev properties

        Ok(())
    }
}
