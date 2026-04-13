use ptfs_metadata::metadata::MetadataStore;

#[test]
fn object_id_maps_correctly_to_metadata() {
    let mut store = MetadataStore::new();

    store.insert("file1.txt", 123);
    store.insert("file2.txt", 456);

    assert_eq!(store.get_by_id(123), Some("file1.txt".to_string()));
    assert_eq!(store.get_by_id(456), Some("file2.txt".to_string()));
}
