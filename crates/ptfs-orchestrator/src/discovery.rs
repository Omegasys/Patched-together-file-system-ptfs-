use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Disk {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub id: Option<String>,
}

pub struct DiskInventory;

impl DiskInventory {
    pub fn new() -> Self {
        Self
    }

    /// Scan for candidate block devices (Linux-style)
    pub fn scan(&self) -> anyhow::Result<Vec<Disk>> {
        let mut disks = Vec::new();

        let dev_paths = fs::read_dir("/dev")?;

        for entry in dev_paths.flatten() {
            let path = entry.path();

            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // crude filter for block devices
                if name.starts_with("sd") || name.starts_with("nvme") {
                    let metadata = fs::metadata(&path).ok();

                    disks.push(Disk {
                        path: path.clone(),
                        size_bytes: metadata.map(|m| m.len()).unwrap_or(0),
                        id: Some(name.to_string()),
                    });
                }
            }
        }

        Ok(disks)
    }
}
