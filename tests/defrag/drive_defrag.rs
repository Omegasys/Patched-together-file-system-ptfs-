use ptfs_defrag::DefragEngine;
use ptfs_defrag::policies::DefragPolicy;

#[test]
fn drive_level_defrag_reduces_fragmentation() {
    let extents = vec![(0, 50), (100, 50), (200, 50), (300, 50)];

    let mut engine = DefragEngine::new(extents.clone(), DefragPolicy::Balanced);

    let before = engine.extents.clone();
    engine.run();
    let after = engine.extents.clone();

    assert_ne!(before, after, "Defrag should change layout");
}
