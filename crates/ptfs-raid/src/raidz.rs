/// Simplified RAID-Z (single parity)
pub struct RaidZ;

impl RaidZ {
    pub fn encode(data_blocks: &[Vec<u8>]) -> Vec<u8> {
        let max_len = data_blocks.iter().map(|b| b.len()).max().unwrap_or(0);
        let mut parity = vec![0u8; max_len];

        for block in data_blocks {
            for (i, byte) in block.iter().enumerate() {
                parity[i] ^= byte;
            }
        }

        parity
    }

    pub fn recover(missing_index: usize, blocks: &mut [Vec<u8>], parity: &[u8]) {
        let len = parity.len();
        let mut recovered = vec![0u8; len];

        for (i, block) in blocks.iter().enumerate() {
            if i == missing_index {
                continue;
            }

            for (j, byte) in block.iter().enumerate() {
                recovered[j] ^= byte;
            }
        }

        for i in 0..len {
            recovered[i] ^= parity[i];
        }

        blocks[missing_index] = recovered;
    }
}
