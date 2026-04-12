use ptfs_allocator::allocator::Allocator;

#[test]
fn allocator_exhaustion_and_fragmentation() {
    let mut allocator = Allocator::new(1_000_000);

    let mut allocated = vec![];

    // allocate until near exhaustion
    for _ in 0..10_000 {
        if let Some(extent) = allocator.allocate(64) {
            allocated.push(extent);
        } else {
            break;
        }
    }

    // free every other block → fragmentation pressure
    for (i, e) in allocated.into_iter().enumerate() {
        if i % 2 == 0 {
            allocator.free(e);
        }
    }

    let stats = allocator.fragmentation();

    assert!(stats.total_extents > 0);
}
