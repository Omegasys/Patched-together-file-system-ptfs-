pub mod tcp;
pub mod http;
pub mod tls;
pub mod websocket;
pub mod retry;

pub use tcp::TcpTransport;
pub use http::HttpTransport;
pub use tls::TlsLayer;
pub use websocket::WebSocketTransport;
pub use retry::RetryPolicy;

/// Unified transport abstraction
pub trait Transport {
    fn send(&self, target: &str, data: &[u8]) -> anyhow::Result<()>;
    fn receive(&self, source: &str) -> anyhow::Result<Vec<u8>>;
}
