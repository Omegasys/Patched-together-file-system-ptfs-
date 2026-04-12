use crate::checksum::Checksum;

/// Scrubbing result
#[derive(Debug)]
pub struct ScrubResult {
    pub checked: usize,
    pub corrupted: usize,
}

/// Scrubber scans data and validates checksums
pub struct Scrubber;

impl Scrubber {
    pub fn scrub_blocks(blocks: &[(Vec<u8>, Checksum)]) -> ScrubResult {
        let mut result = ScrubResult {
            checked: 0,
            corrupted: 0,
        };

        for (data, expected) in blocks {
            result.checked += 1;

            if !expected.verify(data) {
                result.corrupted += 1;
            }
        }

        result
    }
}
