pub mod discovery;
pub mod assembly;
pub mod activation;
pub mod recovery;

use std::sync::Arc;

use crate::discovery::DiskInventory;
use crate::assembly::RaidGroup;
use crate::activation::ActivePool;

/// Main orchestrator entry point
pub struct Orchestrator {
    pub inventory: DiskInventory,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            inventory: DiskInventory::new(),
        }
    }

    /// Full boot sequence:
    /// discover → assemble → activate
    pub fn boot(&mut self) -> anyhow::Result<Vec<ActivePool>> {
        let disks = self.inventory.scan()?;
        let groups = assembly::assemble_raid_groups(&disks)?;
        let pools = activation::activate_pools(groups)?;
        Ok(pools)
    }
}
