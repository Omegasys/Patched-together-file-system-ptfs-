use crate::config::PtfsConfig;

/// Default PTFS configuration
pub fn default_config() -> PtfsConfig {
    PtfsConfig {
        auto_mount: true,
        auto_repair: true,
        auto_rebuild: true,

        preferred_raid: "Custom7".to_string(),
        min_healthy_disks: 4,
        rebuild_threshold: 2,

        cache_size_mb: 512,
        io_threads: 4,

        enable_encryption: true,
        enable_compression: true,
        enable_deduplication: true,

        log_level: "info".to_string(),
        enable_telemetry: false,
    }
}
