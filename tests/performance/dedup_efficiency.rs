use ptfs_dedup::chunking::{Chunker, ChunkingStrategy};
use ptfs_dedup::dedup::Deduplicator;

#[test]
fn deduplication_efficiency_measurement() {
    let chunker = Chunker::new(ChunkingStrategy::Fixed(256));
    let mut dedup = Deduplicator::new(chunker);

    let base = vec![7u8; 10_000];

    let mut saved = 0;

    for _ in 0..1000 {
        let result = dedup.process(&base);
        saved += result.duplicate_bytes;
    }

    println!("Dedup saved bytes: {}", saved);
}
