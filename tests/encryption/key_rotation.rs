use ptfs_encryption::key_manager::KeyManager;
use ptfs_encryption::crypto::Crypto;

#[test]
fn key_rotation_preserves_data() {
    let mut km = KeyManager::new();

    let plaintext = b"rotate-me".to_vec();
    let key_before = km.current_key();

    let ciphertext = Crypto::encrypt_with_key(&plaintext, &key_before).unwrap();

    km.rotate_key();

    let key_after = km.current_key();
    let decrypted = Crypto::decrypt_with_key(&ciphertext, &key_before).unwrap();

    assert_eq!(plaintext, decrypted);
    assert_ne!(key_before, key_after);
}
