use ptfs_api::Ptfs;

#[test]
fn write_read_consistency_many_objects() {
    let mut fs = Ptfs::new();

    fs.mkfs();
    fs.mount();

    for i in 0..1000 {
        fs.write_object(i, vec![i as u8; 128]);
    }

    for i in 0..1000 {
        let data = fs.read_object(&i).expect("missing object");
        assert_eq!(data, &vec![i as u8; 128]);
    }
}
