use std::sync::{Arc, Mutex};
use std::thread;

use crate::events::PtfsEvent;

/// Event handler type
pub type EventHandler = Box<dyn Fn(PtfsEvent) + Send + Sync + 'static>;

/// Thread-safe pub/sub event bus
#[derive(Clone)]
pub struct EventBus {
    listeners: Arc<Mutex<Vec<EventHandler>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribe to all events
    pub fn subscribe<F>(&self, handler: F)
    where
        F: Fn(PtfsEvent) + Send + Sync + 'static,
    {
        self.listeners.lock().unwrap().push(Box::new(handler));
    }

    /// Emit an event to all subscribers
    pub fn emit(&self, event: PtfsEvent) {
        let listeners = self.listeners.clone();
        let listeners = listeners.lock().unwrap();

        for handler in listeners.iter() {
            handler(event.clone());
        }
    }

    /// Emit asynchronously (non-blocking fanout)
    pub fn emit_async(&self, event: PtfsEvent) {
        let listeners = self.listeners.clone();

        thread::spawn(move || {
            let listeners = listeners.lock().unwrap();

            for handler in listeners.iter() {
                handler(event.clone());
            }
        });
    }
}
