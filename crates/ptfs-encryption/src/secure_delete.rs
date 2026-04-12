/// Secure deletion utilities
pub struct SecureDelete;

impl SecureDelete {
    /// Overwrite data in memory (best-effort)
    pub fn wipe(buffer: &mut [u8]) {
        for byte in buffer.iter_mut() {
            *byte = 0;
        }
    }

    /// Simulated crypto erase (delete key)
    pub fn crypto_erase(key: &mut Vec<u8>) {
        for b in key.iter_mut() {
            *b = 0;
        }
        key.clear();
    }
}
