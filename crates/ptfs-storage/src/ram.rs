use crate::backend::{Address, StorageBackend};
use anyhow::Result;
use parking_lot::RwLock;

pub struct RamBackend {
    storage: RwLock<Vec<u8>>,
}

impl RamBackend {
    pub fn new(size: usize) -> Self {
        Self {
         
