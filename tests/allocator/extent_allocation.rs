use ptfs_allocator::allocator::Allocator;

#[test]
fn allocate_and_release_extents() {
    let mut alloc = Allocator::new(1024); // 1024 blocks
    let ext1 = alloc.allocate(100).unwrap();
    let ext2 = alloc.allocate(200).unwrap();

    assert_eq!(ext1.start, 0);
    assert_eq!(ext1.length, 100);
    assert_eq!(ext2.start, 100);
    assert_eq!(ext2.length, 200);

    alloc.free(ext1);
    alloc.free(ext2);

    assert_eq!(alloc.free_space(), 1024);
}
