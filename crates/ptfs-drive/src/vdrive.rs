use crate::drive::Drive;

#[derive(Debug)]
pub struct VirtualDrive {
    pub id: u128,
    pub drives: Vec<Drive>,
}

impl VirtualDrive {
    pub fn new(id: u128, drives: Vec<Drive>) -> Self {
        Self { id, drives }
    }

    pub fn total_capacity(&self) -> u64 {
        self.drives.iter().map(|d| d.capacity).sum()
    }

    pub fn total_used(&self) -> u64 {
        self.drives.iter().map(|d| d.used).sum()
    }

    pub fn allocate(&mut self, size: u64) -> bool {
        // naive striping (RAID0-style placeholder)
        let mut remaining = size;

        for drive in &mut self.drives {
            if remaining == 0 {
                break;
            }

            let alloc = remaining.min(drive.available());
            if drive.allocate(alloc) {
                remaining -= alloc;
            }
        }

        remaining == 0
    }
}
