/// Write buffer (coalesces small writes before flush)
pub struct WriteBuffer {
    buffer: Vec<u8>,
    capacity: usize,
}

impl WriteBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> bool {
        if self.buffer.len() + data.len() > self.capacity {
            return false; // caller should flush
        }

        self.buffer.extend_from_slice(data);
        true
    }

    pub fn flush(&mut self) -> Vec<u8> {
        let flushed = self.buffer.clone();
        self.buffer.clear();
        flushed
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
