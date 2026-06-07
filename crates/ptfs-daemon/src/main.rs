mod orchestrator_loop;
mod event_bus;
mod scheduler;

use orchestrator_loop::OrchestratorLoop;
use event_bus::EventBus;
use scheduler::Scheduler;

/// PTFS daemon entry point
fn main() -> anyhow::Result<()> {
    let bus = EventBus::new();
    let scheduler = Scheduler::new();
    let mut loop_engine = OrchestratorLoop::new(bus.clone(), scheduler);

    println!("[PTFS] daemon starting...");

    loop_engine.init()?;
    loop_engine.run()?;
    Ok(())
}
