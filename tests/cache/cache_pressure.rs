use ptfs_cache::arc::ARC;

#[test]
fn simulate_cache_pressure_eviction() {
    let mut cache = ARC::new(3);

    cache.insert(1, vec![1]);
    cache.insert(2, vec![2]);
    cache.insert(3, vec![3]);

    // Access 1 and 2, so 3 becomes LRU
    let _ = cache.get(&1);
    let _ = cache.get(&2);

    cache.insert(4, vec![4]); // should evict 3

    assert!(cache.get(&3).is_none(), "LRU under pressure should be evicted");
}
