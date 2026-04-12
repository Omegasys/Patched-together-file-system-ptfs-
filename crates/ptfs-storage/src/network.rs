use crate::backend::{Address, StorageBackend};
use anyhow::{Result, anyhow};
use std::io::{Read, Write};
use std::net::TcpStream;

/// Simple network storage backend (prototype)
/// NOTE: This is a minimal request/response protocol
pub struct NetworkBackend {
    pub addr: String,
}

impl NetworkBackend {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    fn send_request(&self, request: &[u8]) -> Result<Vec<u8>> {
        let mut stream = TcpStream::connect(&self.addr)?;

        // Send request length + request
        let len = (request.len() as u32).to_be_bytes();
        stream.write_all(&len)?;
        stream.write_all(request)?;

        // Read response length
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf)?;
        let resp_len = u32::from_be_bytes(len_buf) as usize;

        // Read response
        let mut resp = vec![0u8; resp_len];
        stream.read_exact(&mut resp)?;

        Ok(resp)
    }
}

impl StorageBackend for NetworkBackend {
    fn read(&self, addr: Address, len: usize) -> Result<Vec<u8>> {
        let mut req = vec![0u8; 1 + 16 + 8];
        req[0] = 0; // READ opcode

        req[1..17].copy_from_slice(&addr.to_be_bytes());
        req[17..25].copy_from_slice(&(len as u64).to_be_bytes());

        let resp = self.send_request(&req)?;
        Ok(resp)
    }

    fn write(&self, addr: Address, data: &[u8]) -> Result<()> {
        let mut req = Vec::with_capacity(1 + 16 + 8 + data.len());
        req.push(1); // WRITE opcode

        req.extend_from_slice(&addr.to_be_bytes());
        req.extend_from_slice(&(data.len() as u64).to_be_bytes());
        req.extend_from_slice(data);

        self.send_request(&req)?;
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }

    fn size(&self) -> Result<Address> {
        Err(anyhow!("Network backend size query not implemented"))
    }
}
