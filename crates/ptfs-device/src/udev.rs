/// udev integration stub (Linux-specific future backend)
pub struct UdevMonitor;

impl UdevMonitor {
    pub fn new() -> Self {
        Self
    }

    /// In a real implementation:
    /// - use libudev
    /// - listen for block device add/remove events
    pub fn listen(&self) -> anyhow::Result<()> {
        // Placeholder for future:
        // - device add
        // - device remove
        // - change events

        Ok(())
    }
}
