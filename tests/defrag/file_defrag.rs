use ptfs_defrag::DefragEngine;
use ptfs_defrag::policies::DefragPolicy;

#[test]
fn file_defrag_compacts_extents() {
    let extents = vec![(0, 100), (200, 100), (400, 100)];

    let mut engine = DefragEngine::new(extents.clone(), DefragPolicy::Aggressive);
    engine.run();

    let expected = vec![(0, 100), (100, 100), (200, 100)];

    assert_eq!(engine.extents, expected);
}
