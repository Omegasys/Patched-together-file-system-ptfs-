use crate::assembly::RaidGroup;

#[derive(Debug)]
pub struct ActivePool {
    pub raid_id: String,
    pub device_count: usize,
    pub status: PoolStatus,
}

#[derive(Debug)]
pub enum PoolStatus {
    Healthy,
    Degraded,
    Missing,
}

/// Converts RAID groups into active storage pools
pub fn activate_pools(groups: Vec<RaidGroup>) -> anyhow::Result<Vec<ActivePool>> {
    let mut pools = Vec::new();

    for group in groups {
        let status = if group.disks.len() == group.expected_disks {
            PoolStatus::Healthy
        } else if group.disks.len() > 0 {
            PoolStatus::Degraded
        } else {
            PoolStatus::Missing
        };

        pools.push(ActivePool {
            raid_id: group.id,
            device_count: group.disks.len(),
            status,
        });
    }

    Ok(pools)
}
