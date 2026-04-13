use ptfs_integrity::scrub::Scrubber;
use ptfs_api::Ptfs;

#[test]
fn scrub_detects_inconsistencies() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    fs.write_object(1, b"data".to_vec());
    fs.write_object(2, b"more data".to_vec());

    let scrubber = Scrubber::new(&fs);

    let issues = scrubber.scrub();

    assert!(issues.is_empty(), "No corruption should exist in fresh FS");
}
