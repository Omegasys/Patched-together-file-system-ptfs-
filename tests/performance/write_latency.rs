use std::time::Instant;
use ptfs_api::Ptfs;

#[test]
fn write_latency_measurement() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    let start = Instant::now();

    for i in 0..10_000 {
        fs.write_object(i, vec![2u8; 512]);
    }

    let latency = start.elapsed().as_nanos() as f64 / 10_000.0;

    println!("Avg write latency: {:.2} ns", latency);
}
