use crate::crypto::Crypto;

/// Multi-layer encryption pipeline
pub struct MultiLayerEncryption {
    layers: Vec<Vec<u8>>, // list of keys
}

impl MultiLayerEncryption {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, key: Vec<u8>) {
        self.layers.push(key);
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut result = data.to_vec();

        for key in &self.layers {
            result = Crypto::encrypt(&result, key);
        }

        result
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut result = data.to_vec();

        for key in self.layers.iter().rev() {
            result = Crypto::decrypt(&result, key);
        }

        result
    }
}
