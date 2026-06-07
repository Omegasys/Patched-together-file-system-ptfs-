/// Backup policy system (config-driven behavior rules)

#[derive(Debug, Clone)]
pub struct BackupPolicy {
    pub auto_backup: bool,
    pub interval_seconds: u64,
    pub max_backups: usize,
    pub require_verification: bool,
    pub allowed_backends: Vec<String>,
}

impl BackupPolicy {
    pub fn default() -> Self {
        Self {
            auto_backup: true,
            interval_seconds: 300,
            max_backups: 10,
            require_verification: true,
            allowed_backends: vec![
                "local_server".into(),
                "remote_node".into(),
                "github".into(),
                "s3".into(),
            ],
        }
    }

    /// Check if backend is allowed
    pub fn is_allowed(&self, backend: &str) -> bool {
        self.allowed_backends.contains(&backend.to_string())
    }

    /// Should backup run now?
    pub fn should_run(&self, last_run: u64, now: u64) -> bool {
        if !self.auto_backup {
            return false;
        }

        now.saturating_sub(last_run) >= self.interval_seconds
    }
}
