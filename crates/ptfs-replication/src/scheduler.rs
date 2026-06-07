use std::time::{Duration, Instant};

/// Replication target
#[derive(Clone, Debug)]
pub struct ReplicationTarget {
    pub name: String,
    pub priority: u8,
}

/// Handles WHEN replication occurs
pub struct ReplicationScheduler {
    targets: Vec<ReplicationTarget>,
    last_run: Option<Instant>,
    interval: Duration,
}

impl ReplicationScheduler {
    pub fn new() -> Self {
        Self {
            targets: vec![
                ReplicationTarget {
                    name: "local_nas".into(),
                    priority: 1,
                },
                ReplicationTarget {
                    name: "remote_server".into(),
                    priority: 2,
                },
                ReplicationTarget {
                    name: "github_repo".into(),
                    priority: 3,
                },
            ],
            last_run: None,
            interval: Duration::from_secs(60),
        }
    }

    /// Check if replication should run
    pub fn should_run(&mut self) -> bool {
        let now = Instant::now();

        match self.last_run {
            Some(last) if now.duration_since(last) < self.interval => false,
            _ => {
                self.last_run = Some(now);
                true
            }
        }
    }

    pub fn active_targets(&self) -> Vec<String> {
        self.targets.iter().map(|t| t.name.clone()).collect()
    }
}
