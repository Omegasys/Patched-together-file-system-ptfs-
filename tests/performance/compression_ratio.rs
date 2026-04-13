use ptfs_compression::lz4::compress;

#[test]
fn compression_ratio_measurement() {
    let data = vec![5u8; 100_000];

    let compressed = compress(&data);

    let ratio = compressed.len() as f64 / data.len() as f64;

    println!("Compression ratio: {:.3}", ratio);
}
