use anyhow::Result;

pub struct Transaction {
    pub operations: Vec<String>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn add_op(&mut self, op: String) {
        self.operations.push(op);
    }

    pub fn commit(self) -> Result<()> {
        // Placeholder for atomic commit logic
        Ok(())
    }

    pub fn rollback(self) {
        // Placeholder for rollback logic
    }
}
