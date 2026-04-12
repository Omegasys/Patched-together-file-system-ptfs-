/// RAID 1 (mirroring)
pub struct Raid1 {
    copies: usize,
}

impl Raid1 {
    pub fn new(copies: usize) -> Self {
        Self { copies }
    }

    pub fn mirror(&self, data: &[u8]) -> Vec<Vec<u8>> {
        vec![data.to_vec(); self.copies]
    }

    pub fn recover(&self, copies: &[Vec<u8>]) -> Option<Vec<u8>> {
        copies.first().cloned()
    }
}
