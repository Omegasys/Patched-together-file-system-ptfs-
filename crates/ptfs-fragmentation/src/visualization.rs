use crate::metrics::FragmentationMetrics;

pub struct Visualizer;

impl Visualizer {
    pub fn ascii_map(extents: &[(u64, u64)]) -> String {
        let mut output = String::new();

        for &(start, len) in extents {
            output.push_str(&format!("[{}:{}]", start, start + len));
        }

        output
    }

    pub fn summary(metrics: &FragmentationMetrics) -> String {
        format!(
            "Extents: {}\nContiguous: {}\nFragmented: {}\nFragmentation Ratio: {:.3}",
            metrics.total_extents,
            metrics.contiguous_extents,
            metrics.fragmented_extents,
            metrics.fragmentation_ratio
        )
    }

    pub fn bar(metrics: &FragmentationMetrics) -> String {
        let total = 20;
        let filled = (metrics.fragmentation_ratio * total as f64) as usize;

        let mut bar = String::new();

        for i in 0..total {
            if i < filled {
                bar.push('#');
            } else {
                bar.push('-');
            }
        }

        format!("[{}]", bar)
    }
}
