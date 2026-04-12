use ptfs_api::Ptfs;

#[test]
fn basic_filesystem_init_and_mount() {
    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    assert!(fs.is_mounted(), "Filesystem should be mounted after mount()");
}
