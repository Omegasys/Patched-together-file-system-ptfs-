use std::sync::atomic::{AtomicU128, Ordering};

static TX_COUNTER: AtomicU128 = AtomicU128::new(1);

#[derive(Debug)]
pub struct Transaction {
    pub id: u128,
    pub operations: Vec<String>,
    committed: bool,
}

impl Transaction {
    pub fn begin() -> Self {
        let id = TX_COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            id,
            operations: Vec::new(),
            committed: false,
        }
    }

    pub fn log(&mut self, op: String) {
        self.operations.push(op);
    }

    pub fn commit(&mut self) {
        self.committed = true;
        // future: persist to WAL
    }

    pub fn rollback(&mut self) {
        self.operations.clear();
        self.committed = false;
        // future: undo operations
    }

    pub fn is_committed(&self) -> bool {
        self.committed
    }
}
