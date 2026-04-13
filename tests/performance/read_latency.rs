use std::time::Instant;
use ptfs_api::Ptfs;

#[test]
fn read_latency_measurement() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    for i in 0..10_000 {
        fs.write_object(i, vec![1u8; 512]);
    }

    let start = Instant::now();

    for i in 0..10_000 {
        let _ = fs.read_object(&i);
    }

    let latency = start.elapsed().as_nanos() as f64 / 10_000.0;

    println!("Avg read latency: {:.2} ns", latency);
}
