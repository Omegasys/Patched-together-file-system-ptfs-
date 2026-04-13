use ptfs_integrity::checksum::Checksum;
use ptfs_api::Ptfs;

#[test]
fn simulate_corruption_and_detect() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    fs.write_object(1, b"hello".to_vec());

    // direct memory manipulation to simulate corruption
    let mut obj = fs.read_object_mut(&1).unwrap();
    obj[0] ^= 0xff; // flip first byte

    let checksum_valid = Checksum::verify(&obj);

    assert!(!checksum_valid, "Corruption should be detected");
}
