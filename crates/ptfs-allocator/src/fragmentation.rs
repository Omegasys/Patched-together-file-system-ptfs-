use crate::extents::Extent;

/// Tracks fragmentation statistics
#[derive(Debug, Default)]
pub struct FragmentationStats {
    pub total_extents: usize,
    pub total_space: u128,
    pub largest_extent: u128,
}

impl FragmentationStats {
    pub fn calculate(extents: &[Extent]) -> Self {
        let mut stats = FragmentationStats::default();

        for e in extents {
            stats.total_extents += 1;
            stats.total_space += e.length;
            stats.largest_extent = stats.largest_extent.max(e.length);
        }

        stats
    }

    /// Simple fragmentation metric (0 = perfect, higher = worse)
    pub fn fragmentation_score(&self) -> f64 {
        if self.total_extents == 0 {
            return 0.0;
        }

        let avg = self.total_space as f64 / self.total_extents as f64;
        if avg == 0.0 {
            return 0.0;
        }

        (1.0 - (self.largest_extent as f64 / avg)).abs()
    }
}
