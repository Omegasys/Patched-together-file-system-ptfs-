use ptfs_dedup::dedup::Deduplicator;
use ptfs_dedup::chunking::{Chunker, ChunkingStrategy};

#[test]
fn detect_duplicate_chunks() {
    let chunker = Chunker::new(ChunkingStrategy::Fixed(4));
    let mut dedup = Deduplicator::new(chunker);

    let data1 = b"abcdabcdabcd".to_vec();
    let data2 = b"abcdabcdabcd".to_vec();

    let result1 = dedup.process(&data1);
    let result2 = dedup.process(&data2);

    assert_eq!(result2.duplicate_chunks, result1.total_chunks);
}
