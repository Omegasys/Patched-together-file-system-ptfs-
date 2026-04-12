use rand::Rng;

/// Simple symmetric encryption (placeholder)
/// NOTE: Replace with AES-GCM or ChaCha20-Poly1305 later
pub struct Crypto;

impl Crypto {
    pub fn generate_key() -> Vec<u8> {
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill(&mut key[..]);
        key
    }

    pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .zip(key.iter().cycle())
            .map(|(b, k)| b ^ k)
            .collect()
    }

    pub fn decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
        Self::encrypt(data, key) // XOR is symmetric
    }
}
