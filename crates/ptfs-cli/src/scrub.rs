use ptfs_api::Ptfs;

pub fn run() {
    let fs = Ptfs::new();
    fs.scrub();
}
