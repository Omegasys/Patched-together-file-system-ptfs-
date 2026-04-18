use crate::planner::MovePlan;

pub struct Mover;

impl Mover {
    pub fn execute(plans: &[MovePlan]) {
        for plan in plans {
            // placeholder: simulate data move
            println!(
                "Moving extent from {} (len {}) → {}",
                plan.source.0, plan.source.1, plan.target.0
            );
        }
    }
}
