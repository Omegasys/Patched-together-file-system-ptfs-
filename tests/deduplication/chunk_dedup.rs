use ptfs_dedup::chunking::{Chunker, ChunkingStrategy};

#[test]
fn chunking_generates_consistent_chunks() {
    let data = b"aaaaabbbbcccccdddddeeeee".to_vec();
    let chunker = Chunker::new(ChunkingStrategy::Fixed(5));

    let chunks = chunker.chunk(&data);

    assert_eq!(chunks.len(), 5);
    assert_eq!(chunks[0], b"aaaaa");
    assert_eq!(chunks[4], b"eeeee");
}
