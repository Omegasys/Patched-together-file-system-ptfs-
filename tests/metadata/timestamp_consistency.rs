use ptfs_metadata::metadata::MetadataStore;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn timestamps_increase_consistently() {
    let mut store = MetadataStore::new();

    let ts1 = store.insert("file1.txt", 1);
    sleep(Duration::from_millis(10));
    let ts2 = store.insert("file2.txt", 2);

    assert!(ts2 > ts1, "Timestamps should increase monotonically");
}
