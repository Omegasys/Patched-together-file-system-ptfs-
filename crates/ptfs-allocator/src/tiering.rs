/// Storage tier classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageTier {
    Ram,
    Ssd,
    Hdd,
    Cold,
}

/// Basic tiering policy (placeholder, expandable later)
pub struct TieringPolicy;

impl TieringPolicy {
    pub fn choose_tier(size: u128, hotness: u32) -> StorageTier {
        match (size, hotness) {
            (_, h) if h > 80 => StorageTier::Ram,
            (s, _) if s < 4 * 1024 * 1024 => StorageTier::Ssd, // <4MB
            (s, _) if s < 1_000_000_000 => StorageTier::Hdd,
            _ => StorageTier::Cold,
        }
    }
}
