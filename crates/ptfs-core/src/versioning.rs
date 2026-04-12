use crate::snapshot::Snapshot;

pub struct Versioning {
    pub snapshots: Vec<Snapshot>,
}

impl Versioning {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    pub fn add_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshots.push(snapshot);
    }

    pub fn get_snapshot(&self, id: u128) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }
}
