use crate::analyzer::Analyzer;
use crate::planner::Planner;
use crate::mover::Mover;
use crate::policies::DefragPolicy;

pub struct DefragEngine {
    pub extents: Vec<(u64, u64)>,
    pub policy: DefragPolicy,
}

impl DefragEngine {
    pub fn new(extents: Vec<(u64, u64)>, policy: DefragPolicy) -> Self {
        Self { extents, policy }
    }

    pub fn run(&mut self) {
        let report = Analyzer::analyze(&self.extents);

        println!("Fragmentation ratio: {}", report.fragmentation_ratio);

        if report.fragmentation_ratio < self.policy.threshold() {
            println!("No defrag needed");
            return;
        }

        let plans = Planner::plan(&self.extents);

        if plans.is_empty() {
            println!("Already optimal");
            return;
        }

        Mover::execute(&plans);

        // simulate defragmented layout
        let mut new_extents = Vec::new();
        let mut offset = 0;

        for &(_, len) in &self.extents {
            new_extents.push((offset, len));
            offset += len;
        }

        self.extents = new_extents;
    }
}
