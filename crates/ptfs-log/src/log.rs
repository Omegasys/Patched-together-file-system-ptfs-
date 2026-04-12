use std::collections::VecDeque;

/// Basic log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: u64,
    pub payload: Vec<u8>,
}

/// Append-only log (in-memory for now)
pub struct Log {
    entries: VecDeque<LogEntry>,
    next_id: u64,
}

impl Log {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            next_id: 0,
        }
    }

    pub fn append(&mut self, payload: Vec<u8>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.entries.push_back(LogEntry { id, payload });
        id
    }

    pub fn iter(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
