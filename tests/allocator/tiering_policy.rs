use ptfs_allocator::tiering::TieredAllocator;

#[test]
fn allocate_blocks_respecting_tiers() {
    let mut alloc = TieredAllocator::new(vec![100, 200]); // tier0:100, tier1:200

    let e1 = alloc.allocate(50).unwrap();
    assert_eq!(e1.tier, 0);

    let e2 = alloc.allocate(60).unwrap();
    assert_eq!(e2.tier, 1);

    let e3 = alloc.allocate(10).unwrap();
    assert_eq!(e3.tier, 0);
}
