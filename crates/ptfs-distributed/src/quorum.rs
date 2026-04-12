/// Quorum configuration
pub struct Quorum {
    total_nodes: usize,
}

impl Quorum {
    pub fn new(total_nodes: usize) -> Self {
        Self { total_nodes }
    }

    /// Majority quorum
    pub fn majority(&self) -> usize {
        (self.total_nodes / 2) + 1
    }

    pub fn has_quorum(&self, votes: usize) -> bool {
        votes >= self.majority()
    }
}
