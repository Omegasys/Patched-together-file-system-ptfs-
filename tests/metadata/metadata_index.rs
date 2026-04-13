use ptfs_metadata::metadata::MetadataStore;

#[test]
fn metadata_index_insertion_and_lookup() {
    let mut store = MetadataStore::new();

    store.insert("file1.txt", 1);
    store.insert("file2.txt", 2);

    assert_eq!(store.get("file1.txt"), Some(&1));
    assert_eq!(store.get("file2.txt"), Some(&2));
    assert_eq!(store.get("file3.txt"), None);
}
