use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

use crate::format::Superblock;

/// Writes superblock to disk (raw block 0 region)
pub struct SuperblockWriter;

impl SuperblockWriter {
    pub fn write(path: &str, sb: &Superblock) -> anyhow::Result<()> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .open(path)?;

        file.seek(SeekFrom::Start(0))?;

        let bytes = Self::serialize(sb)?;
        file.write_all(&bytes)?;

        Ok(())
    }

    fn serialize(sb: &Superblock) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.extend_from_slice(&sb.magic);
        buf.extend_from_slice(&sb.version.to_le_bytes());
        buf.extend_from_slice(sb.raid_id.as_bytes());
        buf.push(sb.disk_index);
        buf.push(sb.total_disks);
        buf.extend_from_slice(&sb.block_size.to_le_bytes());
        buf.extend_from_slice(&sb.flags.to_le_bytes());

        Ok(buf)
    }
}
