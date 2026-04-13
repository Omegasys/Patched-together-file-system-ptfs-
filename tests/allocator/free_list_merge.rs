use ptfs_allocator::allocator::{Allocator, Extent};

#[test]
fn merge_adjacent_free_list_entries() {
    let mut alloc = Allocator::new(500);
    let e1 = alloc.allocate(100).unwrap();
    let e2 = alloc.allocate(50).unwrap();

    alloc.free(e1);
    alloc.free(e2);

    // After freeing, adjacent blocks should merge
    let free_list = alloc.free_list();
    assert_eq!(free_list.len(), 1);
    assert_eq!(free_list[0].length, 500);
}
