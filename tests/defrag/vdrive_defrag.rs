use ptfs_defrag::DefragEngine;
use ptfs_defrag::policies::DefragPolicy;

#[test]
fn virtual_drive_defrag_handles_multiple_extents() {
    let extents = vec![
        (0, 100),
        (300, 100),
        (600, 100),
        (900, 100),
    ];

    let mut engine = DefragEngine::new(extents, DefragPolicy::Aggressive);
    engine.run();

    let mut offset = 0;
    for (start, len) in engine.extents {
        assert_eq!(start, offset);
        offset += len;
    }
}
