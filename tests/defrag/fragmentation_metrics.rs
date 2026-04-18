use ptfs_fragmentation::metrics::FragmentationMetrics;

#[test]
fn fragmentation_metrics_detects_gaps() {
    let extents = vec![(0, 100), (200, 100), (300, 100)];

    let metrics = FragmentationMetrics::from_extents(&extents);

    assert!(metrics.fragmented_extents > 0);
    assert!(metrics.fragmentation_ratio > 0.0);
}
