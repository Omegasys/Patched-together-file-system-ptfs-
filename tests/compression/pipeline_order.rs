use ptfs_compression::pipeline::Pipeline;
use ptfs_compression::lz4;
use ptfs_compression::zstd;

#[test]
fn pipeline_order_affects_output() {
    let data = vec![42u8; 512];

    // LZ4 -> Zstd
    let mut pipeline1 = Pipeline::new();
    pipeline1.add(lz4::compress);
    pipeline1.add(zstd::compress);
    let output1 = pipeline1.run(&data);

    // Zstd -> LZ4
    let mut pipeline2 = Pipeline::new();
    pipeline2.add(zstd::compress);
    pipeline2.add(lz4::compress);
    let output2 = pipeline2.run(&data);

    assert_ne!(output1, output2, "Order of compression changes output");
}
