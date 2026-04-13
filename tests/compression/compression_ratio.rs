use ptfs_compression::lz4;
use ptfs_compression::zstd;

#[test]
fn measure_compression_ratio() {
    let data = vec![7u8; 10_000];

    let lz4_compressed = lz4::compress(&data);
    let zstd_compressed = zstd::compress(&data);

    let lz4_ratio = lz4_compressed.len() as f64 / data.len() as f64;
    let zstd_ratio = zstd_compressed.len() as f64 / data.len() as f64;

    println!("LZ4 ratio: {:.3}", lz4_ratio);
    println!("Zstd ratio: {:.3}", zstd_ratio);

    assert!(lz4_ratio < 1.0, "LZ4 should compress data");
    assert!(zstd_ratio < 1.0, "Zstd should compress data");
}
