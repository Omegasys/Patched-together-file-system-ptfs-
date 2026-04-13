use ptfs_encryption::secure_delete::SecureDelete;
use ptfs_api::Ptfs;

#[test]
fn secure_delete_erases_data() {
    let mut fs = Ptfs::new();
    fs.mkfs();
    fs.mount();

    fs.write_object(1, b"sensitive".to_vec());

    SecureDelete::erase(&mut fs, 1);

    let data = fs.read_object(&1);
    assert!(data.is_none(), "Object should be fully erased");
}
