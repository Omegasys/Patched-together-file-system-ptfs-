use ptfs_encryption::crypto::Crypto;
use ptfs_api::Ptfs;

#[test]
fn detect_tampering() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    let plaintext = b"tamper-me".to_vec();
    fs.write_object(1, plaintext.clone());

    // simulate corruption
    let obj = fs.read_object_mut(&1).unwrap();
    obj[0] ^= 0xff;

    let tampered = Crypto::verify_integrity(&obj);
    assert!(!tampered, "Tampering should be detected");
}
