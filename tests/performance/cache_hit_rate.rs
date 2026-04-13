use ptfs_cache::arc::ArcCache;

#[test]
fn cache_hit_rate_measurement() {
    let mut cache = ArcCache::new(1000);

    for i in 0..5000 {
        cache.insert(i % 1000, vec![0u8; 128]);
    }

    let mut hits = 0;
    let mut total = 1000;

    for i in 0..1000 {
        if cache.get(&i).is_some() {
            hits += 1;
        }
    }

    let hit_rate = hits as f64 / total as f64;

    println!("Cache hit rate: {:.2}%", hit_rate * 100.0);
}
