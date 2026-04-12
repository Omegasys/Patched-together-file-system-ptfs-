use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use ptfs_dedup::chunking::{Chunker, ChunkingStrategy};
use ptfs_dedup::dedup::Deduplicator;

fn dedup_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dedup");

    group.measurement_time(Duration::from_secs(5));

    group.bench_function("dedup_repeated_data", |b| {
        b.iter(|| {
            let chunker = Chunker::new(ChunkingStrategy::Fixed(1024));
            let mut dedup = Deduplicator::new(chunker);

            // Highly duplicate data
            let data = vec![42u8; 1024 * 100];

            let _ = dedup.process(&data);
        })
    });

    group.bench_function("dedup_random_data", |b| {
        b.iter(|| {
            let chunker = Chunker::new(ChunkingStrategy::Fixed(1024));
            let mut dedup = Deduplicator::new(chunker);

            // Low duplication
            let data: Vec<u8> = (0..1024 * 100).map(|i| (i % 256) as u8).collect();

            let _ = dedup.process(&data);
        })
    });

    group.finish();
}

criterion_group!(benches, dedup_benchmark);
criterion_main!(benches);
