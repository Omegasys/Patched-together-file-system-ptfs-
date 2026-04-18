use ptfs_vfs::Vfs;

#[test]
fn resolve_nested_paths() {
    let mut vfs = Vfs::new();

    vfs.create_dir("/a/b/c").unwrap();
    vfs.create_file("/a/b/c/file.txt").unwrap();

    let inode = vfs.resolve_path("/a/b/c/file.txt");

    assert!(inode.is_some(), "Path should resolve correctly");
}

#[test]
fn resolve_nonexistent_path() {
    let vfs = Vfs::new();

    let inode = vfs.resolve_path("/does/not/exist");

    assert!(inode.is_none(), "Nonexistent path should return None");
}
