/// Placeholder for erasure coding (Reed-Solomon later)
pub struct ErasureCoding;

impl ErasureCoding {
    pub fn encode(data: &[u8], shards: usize) -> Vec<Vec<u8>> {
        // naive split (NOT real erasure coding)
        let chunk_size = (data.len() + shards - 1) / shards;
        data.chunks(chunk_size).map(|c| c.to_vec()).collect()
    }

    pub fn decode(shards: &[Vec<u8>]) -> Vec<u8> {
        shards.concat()
    }
}
