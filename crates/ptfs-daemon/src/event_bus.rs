use std::sync::{Arc, Mutex};
use std::thread;

/// Core system events
#[derive(Debug, Clone)]
pub enum PtfsEvent {
    DiskInserted(String),
    DiskRemoved(String),
    RaidDegraded(String),
    RaidHealthy(String),
    MountRequested(String),
    RebuildRequested(String),
}

/// Simple pub-sub event bus
#[derive(Clone)]
pub struct EventBus {
    listeners: Arc<Mutex<Vec<Box<dyn Fn(PtfsEvent) + Send + Sync>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe<F>(&self, f: F)
    where
        F: Fn(PtfsEvent) + Send + Sync + 'static,
    {
        self.listeners.lock().unwrap().push(Box::new(f));
    }

    pub fn emit(&self, event: PtfsEvent) {
        let listeners = self.listeners.clone();
        let listeners = listeners.lock().unwrap();

        for listener in listeners.iter() {
            listener(event.clone());
        }
    }
}
