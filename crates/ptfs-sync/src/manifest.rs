use std::collections::HashMap;

/// Tracks sync state across all targets
#[derive(Debug, Default)]
pub struct SyncManifest {
    pub state: HashMap<String, String>, // target -> last snapshot hash
}

impl SyncManifest {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn update(&mut self, target: &str, snapshot: &str) {
        self.state.insert(target.to_string(), snapshot.to_string());
    }

    pub fn get(&self, target: &str) -> Option<&String> {
        self.state.get(target)
    }

    pub fn is_synced(&self, target: &str, snapshot: &str) -> bool {
        self.state.get(target).map(|v| v == snapshot).unwrap_or(false)
    }
}
