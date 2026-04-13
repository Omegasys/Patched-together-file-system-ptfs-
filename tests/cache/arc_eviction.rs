use ptfs_cache::arc::ARC;

#[test]
fn arc_eviction_behavior() {
    let mut cache = ARC::new(3); // capacity 3

    cache.insert(1, vec![1]);
    cache.insert(2, vec![2]);
    cache.insert(3, vec![3]);

    // Cache full, next insert should evict the least recently used
    cache.insert(4, vec![4]);

    assert!(cache.get(&1).is_none(), "Least recently used item should be evicted");
    assert!(cache.get(&2).is_some());
    assert!(cache.get(&3).is_some());
    assert!(cache.get(&4).is_some());
}
