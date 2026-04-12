use crate::Compressor;

/// Compression pipeline (can chain multiple compressors later)
pub struct CompressionPipeline<C: Compressor> {
    compressor: C,
}

impl<C: Compressor> CompressionPipeline<C> {
    pub fn new(compressor: C) -> Self {
        Self { compressor }
    }

    pub fn encode(&self, data: &[u8]) -> Vec<u8> {
        self.compressor.compress(data)
    }

    pub fn decode(&self, data: &[u8]) -> Vec<u8> {
        self.compressor.decompress(data)
    }
}
