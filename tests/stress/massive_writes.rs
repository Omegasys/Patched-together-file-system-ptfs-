use ptfs_api::Ptfs;

#[test]
fn massive_sequential_writes() {
    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    for i in 0..100_000 {
        fs.write_object(i, vec![42u8; 256]);
    }

    assert!(fs.read_object(&0).is_some());
    assert!(fs.read_object(&99_999).is_some());
}
