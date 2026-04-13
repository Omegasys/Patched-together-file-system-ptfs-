use ptfs_compression::lz4;

#[test]
fn lz4_compression_produces_smaller_data() {
    let data = vec![0u8; 1024]; // highly compressible
    let compressed = lz4::compress(&data);
    assert!(compressed.len() < data.len(), "LZ4 should reduce size for repetitive data");
}
