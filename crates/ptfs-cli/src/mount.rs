use ptfs_api::Ptfs;

pub fn run() {
    let mut fs = Ptfs::new();
    fs.mount();

    if fs.is_mounted() {
        println!("Filesystem mounted successfully");
    }
}
