use blake3::Hasher;

/// Wrapper around BLAKE3 hash
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checksum(pub [u8; 32]);

impl Checksum {
    pub fn compute(data: &[u8]) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(data);
        let hash = hasher.finalize();
        Self(*hash.as_bytes())
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        &Self::compute(data) == self
    }
}
