#[derive(Debug, Clone)]
pub struct FragmentationReport {
    pub total_extents: usize,
    pub fragmented_extents: usize,
    pub fragmentation_ratio: f64,
}

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(extents: &[(u64, u64)]) -> FragmentationReport {
        if extents.is_empty() {
            return FragmentationReport {
                total_extents: 0,
                fragmented_extents: 0,
                fragmentation_ratio: 0.0,
            };
        }

        let mut fragmented = 0;

        for window in extents.windows(2) {
            let (_, end) = window[0];
            let (next_start, _) = window[1];

            if end != next_start {
                fragmented += 1;
            }
        }

        FragmentationReport {
            total_extents: extents.len(),
            fragmented_extents: fragmented,
            fragmentation_ratio: fragmented as f64 / extents.len() as f64,
        }
    }
}
