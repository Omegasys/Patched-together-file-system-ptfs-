use std::path::Path;
use std::sync::{Arc, Mutex};

use fuser::{mount2, MountOption};

use ptfs_api::Ptfs;

use crate::ops::PtfsFuse;

/// Mount PTFS using FUSE
pub fn mount_fs(mountpoint: &str) {
    let fs = Arc::new(Mutex::new(Ptfs::new()));

    {
        let mut inner = fs.lock().unwrap();
        inner.mkfs();
        inner.mount();
    }

    let fuse = PtfsFuse::new(fs);

    let options = vec![
        MountOption::RO, // change to RW later
        MountOption::FSName("ptfs".to_string()),
        MountOption::DefaultPermissions,
    ];

    println!("Mounting PTFS at {}", mountpoint);

    mount2(fuse, Path::new(mountpoint), &options).unwrap();
}
