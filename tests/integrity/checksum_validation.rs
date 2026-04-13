use ptfs_integrity::checksum::Checksum;
use ptfs_api::Ptfs;

#[test]
fn checksum_correctness() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    fs.write_object(1, b"hello".to_vec());

    let data = fs.read_object(&1).unwrap();
    let checksum = Checksum::compute(&data);

    assert_eq!(checksum, Checksum::compute(&b"hello".to_vec()));
}
