use ptfs_compression::zstd;

#[test]
fn zstd_compression_produces_smaller_data() {
    let data = vec![1u8; 2048]; // highly compressible
    let compressed = zstd::compress(&data);
    assert!(compressed.len() < data.len(), "Zstd should reduce size for repetitive data");
}
