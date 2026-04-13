use ptfs_encryption::multi_layer::MultiLayerCrypto;

#[test]
fn multiple_encryption_layers() {
    let crypto = MultiLayerCrypto::new();

    let data = b"important".to_vec();

    let layer1 = crypto.encrypt_layer1(&data).unwrap();
    let layer2 = crypto.encrypt_layer2(&layer1).unwrap();

    let decrypted1 = crypto.decrypt_layer2(&layer2).unwrap();
    let decrypted2 = crypto.decrypt_layer1(&decrypted1).unwrap();

    assert_eq!(data, decrypted2);
}
