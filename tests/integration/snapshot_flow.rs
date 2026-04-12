use ptfs_api::Ptfs;

#[test]
fn snapshot_creation_does_not_break_state() {
    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    fs.write_object(1, b"v1".to_vec());

    fs.snapshot();

    fs.write_object(1, b"v2".to_vec());

    let data = fs.read_object(&1).unwrap();
    assert_eq!(data, b"v2");
}
