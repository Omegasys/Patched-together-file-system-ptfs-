use ptfs_cache::l2arc::L2ARC;

#[test]
fn l2arc_promotes_and_eviction() {
    let mut l2 = L2ARC::new(2); // capacity 2

    l2.insert(1, vec![1]);
    l2.insert(2, vec![2]);
    l2.insert(3, vec![3]); // should evict oldest

    assert!(l2.get(&1).is_none(), "Oldest L2ARC block should be evicted");
    assert!(l2.get(&2).is_some());
    assert!(l2.get(&3).is_some());
}
