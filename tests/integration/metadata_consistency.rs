use ptfs_api::Ptfs;

#[test]
fn metadata_consistency_placeholder() {
    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    fs.write_object(42, b"metadata-test".to_vec());

    let data = fs.read_object(&42).unwrap();

    assert_eq!(data, b"metadata-test");

    // future:
    // - validate checksum
    // - validate timestamps
    // - validate index consistency
}
