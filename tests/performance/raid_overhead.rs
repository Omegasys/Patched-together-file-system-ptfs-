use ptfs_raid::raid0::Raid0;

#[test]
fn raid_overhead_measurement() {
    let raid = Raid0::new(4);

    let start = std::time::Instant::now();

    for i in 0..10_000 {
        raid.write(i, vec![9u8; 1024]);
    }

    let duration = start.elapsed();

    println!("RAID0 write overhead: {:?}", duration);
}
