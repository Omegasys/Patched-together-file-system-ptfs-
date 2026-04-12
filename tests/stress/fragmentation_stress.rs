use ptfs_api::Ptfs;

#[test]
fn fragmentation_simulation() {
    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    // write many objects
    for i in 0..10_000 {
        fs.write_object(i, vec![i as u8; 512]);
    }

    // overwrite in non-sequential pattern (fragmentation pressure)
    for i in (0..10_000).step_by(3) {
        fs.write_object(i, vec![0u8; 1024]);
    }

    assert!(fs.read_object(&3).is_some());
}
