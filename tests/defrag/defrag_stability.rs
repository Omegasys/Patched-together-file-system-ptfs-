use ptfs_defrag::DefragEngine;
use ptfs_defrag::policies::DefragPolicy;

#[test]
fn defrag_is_idempotent() {
    let extents = vec![(0, 100), (200, 100), (400, 100)];

    let mut engine = DefragEngine::new(extents, DefragPolicy::Aggressive);

    engine.run();
    let first = engine.extents.clone();

    engine.run();
    let second = engine.extents.clone();

    assert_eq!(first, second, "Running defrag twice should not change layout");
}
