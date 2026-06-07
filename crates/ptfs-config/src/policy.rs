use crate::config::PtfsConfig;

/// Runtime policy engine that enforces safe system behavior
pub struct PolicyEngine {
    config: PtfsConfig,
}

impl PolicyEngine {
    pub fn new(config: PtfsConfig) -> Self {
        Self { config }
    }

    /// Should system auto-mount newly discovered RAID sets?
    pub fn allow_auto_mount(&self) -> bool {
        self.config.auto_mount
    }

    /// Should degraded RAID trigger automatic rebuild?
    pub fn allow_auto_rebuild(&self, missing_disks: u8) -> bool {
        self.config.auto_rebuild && missing_disks >= self.config.rebuild_threshold
    }

    /// Should encryption be enabled for new volumes?
    pub fn encryption_enabled(&self) -> bool {
        self.config.enable_encryption
    }

    /// Should deduplication run on writes?
    pub fn dedup_enabled(&self) -> bool {
        self.config.enable_deduplication
    }

    /// Should compression pipeline be active?
    pub fn compression_enabled(&self) -> bool {
        self.config.enable_compression
    }

    /// Validate RAID health threshold
    pub fn is_raid_healthy(&self, healthy_disks: u8) -> bool {
        healthy_disks >= self.config.min_healthy_disks
    }

    /// Get preferred RAID type
    pub fn preferred_raid(&self) -> &str {
        &self.config.preferred_raid
    }
}
