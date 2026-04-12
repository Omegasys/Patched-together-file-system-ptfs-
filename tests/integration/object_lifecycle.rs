use ptfs_api::Ptfs;

#[test]
fn object_create_read_update_delete_lifecycle() {
    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    fs.write_object(1, b"hello".to_vec());

    let data = fs.read_object(&1).expect("object should exist");
    assert_eq!(data, b"hello");

    fs.write_object(1, b"updated".to_vec());

    let updated = fs.read_object(&1).unwrap();
    assert_eq!(updated, b"updated");

    // simulate delete (not implemented yet)
    // future: fs.delete_object(1);
}
