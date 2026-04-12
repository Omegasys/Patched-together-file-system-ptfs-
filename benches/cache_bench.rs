use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use ptfs_cache::arc::ArcCache;

fn arc_cache_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("arc_cache");

    group.measurement_time(Duration::from_secs(5));

    group.bench_function("arc_insert_and_get", |b| {
        b.iter(|| {
            let mut cache = ArcCache::new(1000);

            // insert
            for i in 0..1000 {
                cache.insert(i, vec![0u8; 256]);
            }

            // access (promote to T2)
            for i in 0..1000 {
                let _ = cache.get(&i);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, arc_cache_benchmark);
criterion_main!(benches);
