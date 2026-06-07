use std::fmt;
use uuid::Uuid;

/// Wrapper for PTFS identity layer
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PtfsUuid(pub Uuid);

impl PtfsUuid {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(Uuid::from_bytes(bytes))
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Debug for PtfsUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PtfsUuid({})", self.0)
    }
}
