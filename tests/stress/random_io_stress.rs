use rand::Rng;
use ptfs_api::Ptfs;

#[test]
fn random_read_write_stress() {
    let mut fs = Ptfs::new();
    let mut rng = rand::thread_rng();

    fs.mkfs();
    fs.mount();

    for _ in 0..50_000 {
        let id: u128 = rng.gen_range(0..10_000);
        let data = vec![rng.gen::<u8>(); 128];

        fs.write_object(id, data);

        let _ = fs.read_object(&id);
    }
}
