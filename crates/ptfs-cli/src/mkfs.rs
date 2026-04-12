use ptfs_api::Ptfs;

pub fn run() {
    let mut fs = Ptfs::new();
    fs.mkfs();
}
