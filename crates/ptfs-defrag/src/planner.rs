#[derive(Debug, Clone)]
pub struct MovePlan {
    pub source: (u64, u64),
    pub target: (u64, u64),
}

pub struct Planner;

impl Planner {
    pub fn plan(extents: &[(u64, u64)]) -> Vec<MovePlan> {
        let mut plans = Vec::new();
        let mut current_offset = 0;

        for &(start, length) in extents {
            if start != current_offset {
                plans.push(MovePlan {
                    source: (start, length),
                    target: (current_offset, length),
                });
            }
            current_offset += length;
        }

        plans
    }
}
