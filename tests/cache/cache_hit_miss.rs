use ptfs_cache::cache::Cache;

#[test]
fn track_cache_hits_and_misses() {
    let mut cache = Cache::new(2);

    cache.insert(1, vec![1]);
    cache.insert(2, vec![2]);

    let _ = cache.get(&1); // hit
    let _ = cache.get(&3); // miss

    assert_eq!(cache.hits(), 1);
    assert_eq!(cache.misses(), 1);
}
