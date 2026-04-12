use crate::log::LogEntry;

/// Journal entry types
#[derive(Debug, Clone)]
pub enum JournalEntry {
    BeginTx(u64),
    CommitTx(u64),
    AbortTx(u64),
    Data(LogEntry),
}

/// Simple journal system
pub struct Journal {
    entries: Vec<JournalEntry>,
}

impl Journal {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn begin_tx(&mut self, tx_id: u64) {
        self.entries.push(JournalEntry::BeginTx(tx_id));
    }

    pub fn commit_tx(&mut self, tx_id: u64) {
        self.entries.push(JournalEntry::CommitTx(tx_id));
    }

    pub fn abort_tx(&mut self, tx_id: u64) {
        self.entries.push(JournalEntry::AbortTx(tx_id));
    }

    pub fn record(&mut self, entry: LogEntry) {
        self.entries.push(JournalEntry::Data(entry));
    }

    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }
}
