/// Custom 7-disk RAID:
/// 4 data + 3 parity (PTFS design)
pub struct Custom7;

impl Custom7 {
    pub const DATA_DISKS: usize = 4;
    pub const PARITY_DISKS: usize = 3;

    pub fn encode(data_blocks: &[Vec<u8>]) -> Vec<Vec<u8>> {
        assert_eq!(data_blocks.len(), Self::DATA_DISKS);

        let mut parity1 = vec![0u8; data_blocks[0].len()];
        let mut parity2 = vec![0u8; data_blocks[0].len()];
        let mut parity3 = vec![0u8; data_blocks[0].len()];

        for block in data_blocks {
            for i in 0..block.len() {
                parity1[i] ^= block[i];
                parity2[i] ^= block[i].wrapping_mul(2);
                parity3[i] ^= block[i].wrapping_mul(3);
            }
        }

        let mut result = data_blocks.to_vec();
        result.push(parity1);
        result.push(parity2);
        result.push(parity3);

        result
    }

    pub fn layout_description() -> &'static str {
        "4 data disks + 3 parity disks (custom PTFS RAID)"
    }
}
