use ptfs_dedup::chunking::{Chunker, ChunkingStrategy};
use ptfs_dedup::dedup::Deduplicator;

#[test]
fn dedup_heavy_duplicate_load() {
    let chunker = Chunker::new(ChunkingStrategy::Fixed(256));
    let mut dedup = Deduplicator::new(chunker);

    let base = vec![7u8; 10_000];

    for _ in 0..1_000 {
        dedup.process(&base);
    }
}
