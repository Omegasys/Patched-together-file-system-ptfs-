use crate::Compressor;

/// ZSTD compressor (placeholder)
pub struct ZstdCompressor;

impl Compressor for ZstdCompressor {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // TODO: integrate zstd crate
        data.to_vec()
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}
