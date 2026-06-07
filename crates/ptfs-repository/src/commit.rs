use std::time::{SystemTime, UNIX_EPOCH};

/// A snapshot → commit mapping (Git-style object)
#[derive(Debug, Clone)]
pub struct Commit {
    pub id: String,
    pub parent: Option<String>,
    pub snapshot_id: String,
    pub message: String,
    pub timestamp: u64,
}

impl Commit {
    pub fn new(snapshot_id: String, parent: Option<String>, message: String) -> Self {
        Self {
            id: Self::generate_id(&snapshot_id, &message),
            parent,
            snapshot_id,
            message,
            timestamp: Self::now(),
        }
    }

    fn generate_id(snapshot: &str, msg: &str) -> String {
        // lightweight hash placeholder (replace with SHA256 later)
        format!("c_{}_{}", snapshot, msg.len())
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
