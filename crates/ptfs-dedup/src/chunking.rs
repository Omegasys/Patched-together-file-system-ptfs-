/// Simple chunking strategies
#[derive(Debug, Clone, Copy)]
pub enum ChunkingStrategy {
    Fixed(usize),
    // Future: ContentDefined (Rabin fingerprinting)
}

/// Splits data into chunks
pub struct Chunker {
    strategy: ChunkingStrategy,
}

impl Chunker {
    pub fn new(strategy: ChunkingStrategy) -> Self {
        Self { strategy }
    }

    pub fn chunk(&self, data: &[u8]) -> Vec<Vec<u8>> {
        match self.strategy {
            ChunkingStrategy::Fixed(size) => {
                data.chunks(size).map(|c| c.to_vec()).collect()
            }
        }
    }
}
