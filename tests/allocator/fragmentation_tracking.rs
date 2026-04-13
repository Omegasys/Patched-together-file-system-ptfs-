use ptfs_allocator::allocator::Allocator;

#[test]
fn fragmentation_increases_and_decreases() {
    let mut alloc = Allocator::new(200);
    let e1 = alloc.allocate(50).unwrap();
    let e2 = alloc.allocate(50).unwrap();
    let e3 = alloc.allocate(50).unwrap();

    alloc.free(e2);

    let frag = alloc.fragmentation();
    assert!(frag > 0, "Freeing middle block increases fragmentation");

    alloc.free(e1);
    alloc.free(e3);

    assert_eq!(alloc.fragmentation(), 0, "All free, fragmentation is zero");
}
