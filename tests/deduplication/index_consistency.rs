use ptfs_dedup::dedup::Deduplicator;
use ptfs_dedup::chunking::{Chunker, ChunkingStrategy};

#[test]
fn dedup_index_remains_consistent() {
    let chunker = Chunker::new(ChunkingStrategy::Fixed(8));
    let mut dedup = Deduplicator::new(chunker);

    let data = b"abcdefghabcdefgh".to_vec();
    dedup.process(&data);

    let index_keys = dedup.index_keys();
    assert!(index_keys.len() > 0);
}
