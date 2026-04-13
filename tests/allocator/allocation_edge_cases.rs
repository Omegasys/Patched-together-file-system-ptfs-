use ptfs_allocator::allocator::Allocator;

#[test]
fn allocation_edge_cases() {
    let mut alloc = Allocator::new(10);

    // Allocate exactly the remaining space
    let e1 = alloc.allocate(10).unwrap();
    assert_eq!(alloc.free_space(), 0);

    // Allocate zero bytes
    let zero = alloc.allocate(0);
    assert!(zero.is_none(), "Zero allocation should return None");

    // Allocate more than available
    let over = alloc.allocate(20);
    assert!(over.is_none(), "Over-allocation should fail");

    alloc.free(e1);
    assert_eq!(alloc.free_space(), 10);
}
