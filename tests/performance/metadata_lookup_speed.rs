use std::time::Instant;
use ptfs_metadata::metadata::MetadataStore;

#[test]
fn metadata_lookup_speed_measurement() {
    let mut store = MetadataStore::new();

    for i in 0..100_000 {
        store.insert(i, format!("object_{}", i));
    }

    let start = Instant::now();

    for i in 0..100_000 {
        let _ = store.get(&i);
    }

    let duration = start.elapsed();

    println!("Metadata lookup total time: {:?}", duration);
}
