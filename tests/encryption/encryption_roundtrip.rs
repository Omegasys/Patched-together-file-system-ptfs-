use ptfs_encryption::crypto::Crypto;
use ptfs_api::Ptfs;

#[test]
fn encryption_roundtrip() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    let plaintext = b"secret data".to_vec();

    let ciphertext = Crypto::encrypt(&plaintext).unwrap();
    let decrypted = Crypto::decrypt(&ciphertext).unwrap();

    assert_eq!(plaintext, decrypted);
}
