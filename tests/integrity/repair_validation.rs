use ptfs_integrity::repair::RepairManager;
use ptfs_integrity::checksum::Checksum;
use ptfs_api::Ptfs;

#[test]
fn repair_corrupted_object() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    fs.write_object(1, b"original".to_vec());

    // simulate corruption
    let obj = fs.read_object_mut(&1).unwrap();
    obj[0] ^= 0xff;

    let mut repair = RepairManager::new(&fs);

    let repaired = repair.repair_object(1);

    assert_eq!(repaired.unwrap(), b"original".to_vec());
    assert!(Checksum::verify(&repaired.unwrap()));
}
