use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::block::BlockDevice;

/// Simple polling-based hotplug manager (portable fallback)
pub struct HotplugManager {
    listeners: Arc<Mutex<Vec<Box<dyn Fn(String) + Send + Sync>>>>,
}

impl HotplugManager {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register callback for device events
    pub fn on_event<F>(&self, f: F)
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.listeners.lock().unwrap().push(Box::new(f));
    }

    /// Start polling loop (fallback for udev)
    pub fn start(&self) -> anyhow::Result<()> {
        let listeners = self.listeners.clone();

        thread::spawn(move || {
            let mut known = std::collections::HashSet::<String>::new();

            loop {
                if let Ok(entries) = std::fs::read_dir("/dev") {
                    for entry in entries.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            if name.starts_with("sd") || name.starts_with("nvme") {
                                if known.insert(name.to_string()) {
                                    let mut l = listeners.lock().unwrap();
                                    for cb in l.iter() {
                                        cb(format!("insert:{}", name));
                                    }
                                }
                            }
                        }
                    }
                }

                thread::sleep(Duration::from_secs(2));
            }
        });

        Ok(())
    }
}
