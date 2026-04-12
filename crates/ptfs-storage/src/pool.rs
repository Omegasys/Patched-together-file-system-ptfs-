use crate::backend::{Address, StorageBackend};
use anyhow::{Result, anyhow};
use std::sync::Arc;

/// Storage pool with basic striping (RAID-0 style)
pub struct StoragePool {
    devices: Vec<Arc<dyn StorageBackend>>,
    stripe_size: u64,
}

impl StoragePool {
    pub fn new(stripe_size: u64) -> Self {
        Self {
            devices: Vec::new(),
            stripe_size,
        }
    }

    pub fn add_device(&mut self, device: Arc<dyn StorageBackend>) {
        self.devices.push(device);
    }

    fn get_device_and_offset(&self, addr: Address) -> Result<(usize, Address)> {
        if self.devices.is_empty() {
            return Err(anyhow!("No devices in pool"));
        }

        let stripe = addr / self.stripe_size as u128;
        let device_index = (stripe % self.devices.len() as u128) as usize;

        let offset = (stripe / self.devices.len() as u128) * self.stripe_size as u128
            + (addr % self.stripe_size as u128);

        Ok((device_index, offset))
    }
}

impl StorageBackend for StoragePool {
    fn read(&self, addr: Address, len: usize) -> Result<Vec<u8>> {
        let mut remaining = len;
        let mut current_addr = addr;
        let mut result = Vec::with_capacity(len);

        while remaining > 0 {
            let (dev_idx, dev_offset) = self.get_device_and_offset(current_addr)?;
            let dev = &self.devices[dev_idx];

            let chunk = std::cmp::min(
                remaining,
                (self.stripe_size - (current_addr % self.stripe_size as u128) as u64) as usize,
            );

            let data = dev.read(dev_offset, chunk)?;
            result.extend_from_slice(&data);

            current_addr += chunk as u128;
            remaining -= chunk;
        }

        Ok(result)
    }

    fn write(&self, addr: Address, data: &[u8]) -> Result<()> {
        let mut remaining = data.len();
        let mut current_addr = addr;
        let mut offset = 0;

        while remaining > 0 {
            let (dev_idx, dev_offset) = self.get_device_and_offset(current_addr)?;
            let dev = &self.devices[dev_idx];

            let chunk = std::cmp::min(
                remaining,
                (self.stripe_size - (current_addr % self.stripe_size as u128) as u64) as usize,
            );

            dev.write(dev_offset, &data[offset..offset + chunk])?;

            current_addr += chunk as u128;
            offset += chunk;
            remaining -= chunk;
        }

        Ok(())
    }

    fn flush(&self) -> Result<()> {
        for dev in &self.devices {
            dev.flush()?;
        }
        Ok(())
    }

    fn size(&self) -> Result<Address> {
        if self.devices.is_empty() {
            return Ok(0);
        }

        let mut min_size = u128::MAX;
        for dev in &self.devices {
            let size = dev.size()?;
            if size < min_size {
                min_size = size;
            }
        }

        // Total usable size in striped configuration
        Ok(min_size * self.devices.len() as u128)
    }
}
