use ptfs_api::Ptfs;

#[test]
fn recovery_after_crash_simulation() {
    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    fs.write_object(1, b"important-data".to_vec());
    fs.write_object(2, b"more-data".to_vec());

    // simulate "crash" by dropping fs
    drop(fs);

    // restart filesystem
    let mut fs = Ptfs::new();
    fs.mount();

    // currently will NOT persist (expected in phase 1)
    // this test defines future WAL behavior

    assert!(fs.read_object(&1).is_none());
}
