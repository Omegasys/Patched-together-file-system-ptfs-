#[derive(Debug, Clone)]
pub struct FragmentationMetrics {
    pub total_extents: usize,
    pub contiguous_extents: usize,
    pub fragmented_extents: usize,
    pub fragmentation_ratio: f64,
}

impl FragmentationMetrics {
    pub fn from_extents(extents: &[(u64, u64)]) -> Self {
        if extents.is_empty() {
            return Self {
                total_extents: 0,
                contiguous_extents: 0,
                fragmented_extents: 0,
                fragmentation_ratio: 0.0,
            };
        }

        let mut contiguous = 0;
        let mut fragmented = 0;

        for window in extents.windows(2) {
            let (_, len) = window[0];
            let expected_next = window[0].0 + len;
            let actual_next = window[1].0;

            if expected_next == actual_next {
                contiguous += 1;
            } else {
                fragmented += 1;
            }
        }

        let total = extents.len();

        Self {
            total_extents: total,
            contiguous_extents: contiguous,
            fragmented_extents: fragmented,
            fragmentation_ratio: if total > 1 {
                fragmented as f64 / (total - 1) as f64
            } else {
                0.0
            },
        }
    }

    pub fn is_fragmented(&self, threshold: f64) -> bool {
        self.fragmentation_ratio > threshold
    }
}
