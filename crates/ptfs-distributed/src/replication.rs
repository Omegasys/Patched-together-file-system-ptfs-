use std::collections::HashMap;

/// Tracks replication state of objects
pub struct ReplicationManager {
    replication_factor: usize,
    object_locations: HashMap<u128, Vec<u128>>, // object_id -> node_ids
}

impl ReplicationManager {
    pub fn new(replication_factor: usize) -> Self {
        Self {
            replication_factor,
            object_locations: HashMap::new(),
        }
    }

    pub fn register_replica(&mut self, object_id: u128, node_id: u128) {
        self.object_locations
            .entry(object_id)
            .or_default()
            .push(node_id);
    }

    pub fn get_replicas(&self, object_id: &u128) -> Option<&Vec<u128>> {
        self.object_locations.get(object_id)
    }

    pub fn needs_replication(&self, object_id: &u128) -> bool {
        match self.object_locations.get(object_id) {
            Some(nodes) => nodes.len() < self.replication_factor,
            None => true,
        }
    }
}
