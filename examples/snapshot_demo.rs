use ptfs_api::Ptfs;

fn main() {
    println!("=== PTFS Snapshot Demo ===");

    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    // Initial write
    fs.write_object(1, b"Version 1".to_vec());

    println!("Creating snapshot...");
    fs.snapshot();

    // Modify data (simulated CoW future)
    fs.write_object(1, b"Version 2".to_vec());

    // Read current version
    if let Some(data) = fs.read_object(&1) {
        println!("Current object: {}", String::from_utf8_lossy(data));
    }

    println!("(Snapshots are placeholders until CoW is implemented)");

    println!("=== Done ===");
}
