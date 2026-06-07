use std::collections::VecDeque;

/// A single sync operation
#[derive(Debug, Clone)]
pub struct SyncItem {
    pub payload_id: String,
    pub target: String,
    pub data_ref: String,
}

/// Simple FIFO sync queue
#[derive(Default)]
pub struct SyncQueue {
    queue: VecDeque<SyncItem>,
    sent: Vec<String>,
}

impl SyncQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            sent: Vec::new(),
        }
    }

    pub fn push(&mut self, item: SyncItem) {
        self.queue.push_back(item);
    }

    pub fn pop(&mut self) -> Option<SyncItem> {
        self.queue.pop_front()
    }

    pub fn mark_sent(&mut self, id: &str) {
        self.sent.push(id.to_string());
    }

    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}
