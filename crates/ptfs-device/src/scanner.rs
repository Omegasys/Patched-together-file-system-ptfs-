use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::block::BlockDevice;

/// Scans system for block devices
pub struct DeviceScanner;

impl DeviceScanner {
    pub fn new() -> Self {
        Self
    }

    /// Basic Linux /dev scanner
    pub fn scan(&self) -> anyhow::Result<Vec<Arc<BlockDevice>>> {
        let mut devices = Vec::new();

        let entries = fs::read_dir("/dev")?;

        for entry in entries.flatten() {
            let path = entry.path();

            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            // Filter likely block devices
            if name.starts_with("sd")
                || name.starts_with("nvme")
                || name.starts_with("vd")
            {
                let mut dev = BlockDevice::new(path.clone(), name.to_string());
                dev.probe()?;

                devices.push(Arc::new(dev));
            }
        }

        Ok(devices)
    }
}
