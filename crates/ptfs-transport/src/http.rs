use crate::Transport;

/// Internet / API-based transport (GitHub, cloud, remote nodes)
pub struct HttpTransport;

impl HttpTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Transport for HttpTransport {
    fn send(&self, target: &str, data: &[u8]) -> anyhow::Result<()> {
        println!(
            "[PTFS-HTTP] POST {} bytes to {}",
            data.len(),
            target
        );

        // Real system:
        // - reqwest client
        // - chunked uploads
        // - authentication headers

        Ok(())
    }

    fn receive(&self, source: &str) -> anyhow::Result<Vec<u8>> {
        println!("[PTFS-HTTP] GET from {}", source);

        Ok(vec![4, 5, 6])
    }
}
