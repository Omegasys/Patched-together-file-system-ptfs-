use ptfs_compression::pipeline::Pipeline;
use ptfs_compression::lz4;
use ptfs_compression::zstd;

#[test]
fn compression_roundtrip() {
    let data = b"The quick brown fox jumps over the lazy dog".to_vec();

    let compressed_lz4 = lz4::compress(&data);
    let decompressed_lz4 = lz4::decompress(&compressed_lz4).unwrap();
    assert_eq!(data, decompressed_lz4);

    let compressed_zstd = zstd::compress(&data);
    let decompressed_zstd = zstd::decompress(&compressed_zstd).unwrap();
    assert_eq!(data, decompressed_zstd);
}
