use std::thread;
use std::time::Duration;

use crate::event_bus::{EventBus, PtfsEvent};
use crate::scheduler::Scheduler;

/// Main runtime brain of PTFS
pub struct OrchestratorLoop {
    bus: EventBus,
    scheduler: Scheduler,
}

impl OrchestratorLoop {
    pub fn new(bus: EventBus, scheduler: Scheduler) -> Self {
        Self { bus, scheduler }
    }

    /// Initialize event handlers
    pub fn init(&mut self) -> anyhow::Result<()> {
        let bus = self.bus.clone();

        bus.subscribe(|event| {
            match event {
                PtfsEvent::DiskInserted(name) => {
                    println!("[PTFS] disk inserted: {}", name);
                }
                PtfsEvent::DiskRemoved(name) => {
                    println!("[PTFS] disk removed: {}", name);
                }
                PtfsEvent::RaidDegraded(id) => {
                    println!("[PTFS] RAID degraded: {}", id);
                }
                PtfsEvent::RaidHealthy(id) => {
                    println!("[PTFS] RAID healthy: {}", id);
                }
                PtfsEvent::MountRequested(target) => {
                    println!("[PTFS] mount requested: {}", target);
                }
                PtfsEvent::RebuildRequested(id) => {
                    println!("[PTFS] rebuild requested: {}", id);
                }
            }
        });

        Ok(())
    }

    /// Main daemon loop
    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            // 1. Handle scheduled tasks
            let tasks = self.scheduler.tick();
            for task in tasks {
                println!("[PTFS] executing task: {}", task.name);
            }

            // 2. Simulated system heartbeat
            self.bus.emit(PtfsEvent::RaidHealthy("system".into()));

            // 3. Sleep (real daemon would block on events)
            thread::sleep(Duration::from_secs(5));
        }
    }
}
