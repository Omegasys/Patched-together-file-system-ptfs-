use ptfs_dedup::dedup::Deduplicator;
use ptfs_dedup::chunking::{Chunker, ChunkingStrategy};

#[test]
fn calculate_storage_savings() {
    let chunker = Chunker::new(ChunkingStrategy::Fixed(4));
    let mut dedup = Deduplicator::new(chunker);

    let data1 = b"aaaa1111bbbb2222".to_vec();
    let data2 = b"aaaa1111bbbb2222".to_vec();

    dedup.process(&data1);
    let result = dedup.process(&data2);

    assert!(result.duplicate_bytes > 0, "Dedup should save storage");
}
