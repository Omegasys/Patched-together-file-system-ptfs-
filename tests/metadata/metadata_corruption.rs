use ptfs_metadata::metadata::MetadataStore;

#[test]
fn detect_metadata_corruption() {
    let mut store = MetadataStore::new();

    store.insert("file1.txt", 1);

    // simulate corruption
    store.corrupt_entry("file1.txt");

    let result = store.get("file1.txt");
    assert!(result.is_none(), "Corrupted metadata should be detected");
}
