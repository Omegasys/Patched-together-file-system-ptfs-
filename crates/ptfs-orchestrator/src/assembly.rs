use crate::discovery::Disk;

#[derive(Debug, Clone)]
pub struct RaidGroup {
    pub id: String,
    pub disks: Vec<Disk>,
    pub expected_disks: usize,
}

/// Simple grouping strategy:
/// - groups disks by prefix (placeholder)
/// - later replaced with RAID metadata headers (PTFS superblock)
pub fn assemble_raid_groups(disks: &[Disk]) -> anyhow::Result<Vec<RaidGroup>> {
    let mut groups: Vec<RaidGroup> = Vec::new();

    for disk in disks {
        let key = disk
            .id
            .as_ref()
            .and_then(|id| id.chars().take(3).collect::<String>().into())
            .unwrap_or_else(|| "unknown".to_string());

        let mut found = false;

        for group in &mut groups {
            if group.id == key {
                group.disks.push(disk.clone());
                found = true;
                break;
            }
        }

        if !found {
            groups.push(RaidGroup {
                id: key,
                disks: vec![disk.clone()],
                expected_disks: 7, // matches your Custom7 RAID design
            });
        }
    }

    Ok(groups)
}
