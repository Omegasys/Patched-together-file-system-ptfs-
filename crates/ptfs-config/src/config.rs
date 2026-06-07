use serde::{Deserialize, Serialize};

/// Global PTFS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtfsConfig {
    // General system behavior
    pub auto_mount: bool,
    pub auto_repair: bool,
    pub auto_rebuild: bool,

    // RAID settings
    pub preferred_raid: String,
    pub min_healthy_disks: u8,
    pub rebuild_threshold: u8,

    // Performance tuning
    pub cache_size_mb: usize,
    pub io_threads: usize,

    // Data safety
    pub enable_encryption: bool,
    pub enable_compression: bool,
    pub enable_deduplication: bool,

    // Logging / debugging
    pub log_level: String,
    pub enable_telemetry: bool,
}

impl PtfsConfig {
    /// Load from file (future expansion)
    pub fn load_from_file(_path: &str) -> anyhow::Result<Self> {
        // Placeholder: integrate with serde_json/toml later
        Ok(super::defaults::default_config())
    }

    pub fn save_to_file(&self, _path: &str) -> anyhow::Result<()> {
        // Placeholder for persistence layer
        Ok(())
    }
}
