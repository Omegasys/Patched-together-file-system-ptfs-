use ptfs_vfs::Vfs;

#[test]
fn directory_hierarchy_is_correct() {
    let mut vfs = Vfs::new();

    vfs.create_dir("/a/b/c").unwrap();

    let a = vfs.resolve_path("/a").unwrap();
    let b = vfs.resolve_path("/a/b").unwrap();
    let c = vfs.resolve_path("/a/b/c").unwrap();

    assert!(a != b && b != c, "Each directory should have unique inode");
}

#[test]
fn directory_contains_children() {
    let mut vfs = Vfs::new();

    vfs.create_dir("/parent").unwrap();
    vfs.create_file("/parent/file1.txt").unwrap();
    vfs.create_file("/parent/file2.txt").unwrap();

    let parent_id = vfs.resolve_path("/parent").unwrap();
    let parent_inode = &vfs.inodes[&parent_id];

    assert_eq!(parent_inode.children.len(), 2);
}
