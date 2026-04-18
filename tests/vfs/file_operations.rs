use ptfs_vfs::Vfs;

#[test]
fn create_and_resolve_file() {
    let mut vfs = Vfs::new();

    vfs.create_dir("/data").unwrap();
    let file_inode = vfs.create_file("/data/file.txt").unwrap();

    let resolved = vfs.resolve_path("/data/file.txt");

    assert_eq!(resolved, Some(file_inode));
}

#[test]
fn fail_create_file_in_missing_directory() {
    let mut vfs = Vfs::new();

    let result = vfs.create_file("/missing/file.txt");

    assert!(result.is_err(), "Should fail if parent directory doesn't exist");
}
