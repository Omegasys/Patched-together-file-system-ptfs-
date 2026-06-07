use crate::trait_::StorageBackend;

/// Remote PTFS node (custom server over HTTP/TCP via transport layer)
pub struct RemoteNodeBackend {
    pub endpoint: String,
}

impl RemoteNodeBackend {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

impl StorageBackend for RemoteNodeBackend {
    fn name(&self) -> &str {
        "remote_node"
    }

    fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<()> {
        println!(
            "[PTFS-BACKEND] uploading {} bytes → remote node {} ({})",
            data.len(),
            self.endpoint,
            key
        );

        // Real system:
        // - uses ptfs-transport::http or https
        // - POST /store/{key}

        Ok(())
    }

    fn download(&self, key: &str) -> anyhow::Result<Vec<u8>> {
        println!(
            "[PTFS-BACKEND] downloading {} from remote node {}",
            key,
            self.endpoint
        );

        Ok(vec![1, 2, 3]) // placeholder
    }

    fn health_check(&self) -> bool {
        true
    }
}
