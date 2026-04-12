use ptfs_api::Ptfs;

fn main() {
    println!("=== PTFS Simple Filesystem Demo ===");

    let mut fs = Ptfs::new();

    // Create filesystem
    fs.mkfs();

    // Mount it
    fs.mount();

    // Write some objects
    fs.write_object(1, b"Hello, PTFS!".to_vec());
    fs.write_object(2, b"Another object".to_vec());

    // Read them back
    if let Some(data) = fs.read_object(&1) {
        println!("Object 1: {}", String::from_utf8_lossy(data));
    }

    if let Some(data) = fs.read_object(&2) {
        println!("Object 2: {}", String::from_utf8_lossy(data));
    }

    println!("=== Done ===");
}
