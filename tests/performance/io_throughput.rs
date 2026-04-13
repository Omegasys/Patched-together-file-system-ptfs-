use std::time::Instant;
use ptfs_api::Ptfs;

#[test]
fn io_throughput_measurement() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    let start = Instant::now();

    for i in 0..50_000 {
        fs.write_object(i, vec![0u8; 1024]);
    }

    let duration = start.elapsed();
    let mb_written = (50_000 * 1024) as f64 / (1024.0 * 1024.0);

    println!("Throughput: {:.2} MB/s", mb_written / duration.as_secs_f64());
}
