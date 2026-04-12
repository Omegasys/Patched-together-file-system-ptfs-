use blake3::Hasher;

use crate::chunking::Chunker;
use crate::index::DedupIndex;

/// Deduplication engine
pub struct Deduplicator {
    chunker: Chunker,
    index: DedupIndex,
}

#[derive(Debug)]
pub struct DedupResult {
    pub unique_chunks: usize,
    pub duplicate_chunks: usize,
}

impl Deduplicator {
    pub fn new(chunker: Chunker) -> Self {
        Self {
            chunker,
            index: DedupIndex::new(),
        }
    }

    pub fn process(&mut self, data: &[u8]) -> DedupResult {
        let chunks = self.chunker.chunk(data);

        let mut unique = 0;
        let mut duplicate = 0;

        for chunk in chunks {
            let mut hasher = Hasher::new();
            hasher.update(&chunk);
            let hash = hasher.finalize();

            if self.index.insert(hash) {
                unique += 1;
            } else {
                duplicate += 1;
            }
        }

        DedupResult {
            unique_chunks: unique,
            duplicate_chunks: duplicate,
        }
    }
}
