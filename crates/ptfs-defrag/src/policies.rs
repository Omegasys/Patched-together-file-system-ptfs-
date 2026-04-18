#[derive(Debug, Clone)]
pub enum DefragPolicy {
    Aggressive,
    Balanced,
    Conservative,
}

impl DefragPolicy {
    pub fn threshold(&self) -> f64 {
        match self {
            DefragPolicy::Aggressive => 0.1,
            DefragPolicy::Balanced => 0.3,
            DefragPolicy::Conservative => 0.5,
        }
    }
}
