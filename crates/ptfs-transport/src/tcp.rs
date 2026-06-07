use crate::Transport;

/// LAN / local server transport (fast path)
pub struct TcpTransport;

impl TcpTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Transport for TcpTransport {
    fn send(&self, target: &str, data: &[u8]) -> anyhow::Result<()> {
        println!("[PTFS-TCP] sending {} bytes to {}", data.len(), target);

        // Real implementation:
        // - std::net::TcpStream
        // - streaming chunks
        // - ack-based delivery

        Ok(())
    }

    fn receive(&self, source: &str) -> anyhow::Result<Vec<u8>> {
        println!("[PTFS-TCP] receiving from {}", source);

        Ok(vec![1, 2, 3]) // placeholder payload
    }
}
