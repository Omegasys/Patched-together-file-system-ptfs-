use crate::Compressor;

/// LZ4 compressor (placeholder implementation)
pub struct Lz4Compressor;

impl Compressor for Lz4Compressor {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // TODO: replace with real lz4 crate
        data.to_vec()
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        // TODO: replace with real lz4 crate
        data.to_vec()
    }
}
