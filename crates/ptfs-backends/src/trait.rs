/// Unified interface for all storage destinations
pub trait StorageBackend {
    fn name(&self) -> &str;

    /// Upload raw snapshot / diff
    fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<()>;

    /// Download snapshot / diff
    fn download(&self, key: &str) -> anyhow::Result<Vec<u8>>;

    /// Check if backend is reachable
    fn health_check(&self) -> bool;
}
