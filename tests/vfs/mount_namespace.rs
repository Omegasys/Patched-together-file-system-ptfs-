use ptfs_vfs::namespace::Namespace;

#[test]
fn mount_and_lookup_namespace() {
    let mut ns = Namespace::new();

    ns.mount("/mnt".to_string(), 42);

    let mount = ns.mounts.get("/mnt");

    assert_eq!(mount, Some(&42));
}

#[test]
fn overwrite_mount_point() {
    let mut ns = Namespace::new();

    ns.mount("/mnt".to_string(), 1);
    ns.mount("/mnt".to_string(), 2);

    let mount = ns.mounts.get("/mnt");

    assert_eq!(mount, Some(&2), "Mount should be overwritten");
}
