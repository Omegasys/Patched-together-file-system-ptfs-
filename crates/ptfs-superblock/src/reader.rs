use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use uuid::Uuid;

use crate::format::{Superblock, PTFS_MAGIC};

/// Reads PTFS superblock from disk
pub struct SuperblockReader;

impl SuperblockReader {
    pub fn read(path: &str) -> anyhow::Result<Option<Superblock>> {
        let mut file = File::open(path)?;
        let mut buf = vec![0u8; 64];

        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut buf)?;

        if &buf[0..4] != PTFS_MAGIC {
            return Ok(None);
        }

        let version = u16::from_le_bytes([buf[4], buf[5]]);
        let raid_id = Uuid::from_slice(&buf[6..22])?;

        let disk_index = buf[22];
        let total_disks = buf[23];

        let block_size = u32::from_le_bytes([
            buf[24], buf[25], buf[26], buf[27],
        ]);

        let flags = u64::from_le_bytes([
            buf[28], buf[29], buf[30], buf[31],
            buf[32], buf[33], buf[34], buf[35],
        ]);

        Ok(Some(Superblock {
            magic: PTFS_MAGIC,
            version,
            raid_id,
            disk_index,
            total_disks,
            block_size,
            flags,
        }))
    }
}
