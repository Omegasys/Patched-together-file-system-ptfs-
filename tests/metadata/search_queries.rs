use ptfs_metadata::metadata::MetadataStore;

#[test]
fn search_returns_correct_entries() {
    let mut store = MetadataStore::new();

    store.insert("file1.txt", 1);
    store.insert("file2.log", 2);
    store.insert("image.png", 3);

    let results = store.search("file");
    assert_eq!(results.len(), 2);
    assert!(results.contains(&1));
    assert!(results.contains(&2));
}
