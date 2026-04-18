use crate::defrag::DefragEngine;

pub struct Scheduler;

impl Scheduler {
    pub fn run_background(engine: &mut DefragEngine) {
        // placeholder: periodic run
        engine.run();
    }
}
