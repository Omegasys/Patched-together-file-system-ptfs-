use ptfs_cache::arc::ArcCache;

#[test]
fn cache_thrashing_pattern() {
    let mut cache = ArcCache::new(100);

    // repeatedly exceed capacity → force eviction cycles
    for i in 0..10_000 {
        cache.insert(i, vec![i as u8; 64]);

        if i % 2 == 0 {
            let _ = cache.get(&i);
        }
    }
}
