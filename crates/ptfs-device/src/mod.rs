pub mod block;
pub mod hotplug;
pub mod scanner;
pub mod udev;

use std::sync::Arc;

use crate::block::BlockDevice;
use crate::scanner::DeviceScanner;
use crate::hotplug::HotplugManager;

/// Central device subsystem
pub struct DeviceManager {
    scanner: DeviceScanner,
    hotplug: HotplugManager,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            scanner: DeviceScanner::new(),
            hotplug: HotplugManager::new(),
        }
    }

    /// Initial system scan (boot-time discovery)
    pub fn scan_devices(&self) -> anyhow::Result<Vec<Arc<BlockDevice>>> {
        self.scanner.scan()
    }

    /// Start hotplug monitoring loop
    pub fn start_hotplug(&self) -> anyhow::Result<()> {
        self.hotplug.start()
    }
}
