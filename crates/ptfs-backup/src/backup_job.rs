use std::time::{SystemTime, UNIX_EPOCH};

/// A single backup execution unit
pub struct BackupJob {
    pub job_id: String,
    pub source_snapshot: String,
    pub target_backends: Vec<String>,
    pub created_at: u64,
}

impl BackupJob {
    pub fn new(source_snapshot: String, target_backends: Vec<String>) -> Self {
        Self {
            job_id: Self::generate_id(&source_snapshot),
            source_snapshot,
            target_backends,
            created_at: Self::now(),
        }
    }

    fn generate_id(snapshot: &str) -> String {
        format!("job_{}", snapshot)
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Execute backup job
    pub fn execute(&self) -> anyhow::Result<()> {
        println!("[PTFS-BACKUP] executing job {}", self.job_id);

        for target in &self.target_backends {
            println!(
                "[PTFS-BACKUP] snapshot {} → {}",
                self.source_snapshot,
                target
            );

            // Real system:
            // - fetch snapshot from repository
            // - call ptfs-sync
            // - push via ptfs-transport
            // - store via ptfs-backends
        }

        Ok(())
    }
}
