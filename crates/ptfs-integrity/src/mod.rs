pub mod lz4;
pub mod zstd;
pub mod pipeline;

/// Compression trait
pub trait Compressor {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
    fn decompress(&self, data: &[u8]) -> Vec<u8>;
}
