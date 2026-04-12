/// RAID 0 (striping)
pub struct Raid0 {
    disks: usize,
}

impl Raid0 {
    pub fn new(disks: usize) -> Self {
        Self { disks }
    }

    pub fn stripe(&self, data: &[u8]) -> Vec<Vec<u8>> {
        let mut stripes = vec![Vec::new(); self.disks];

        for (i, byte) in data.iter().enumerate() {
            stripes[i % self.disks].push(*byte);
        }

        stripes
    }
}
