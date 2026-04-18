#[derive(Debug, Clone)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Permissions {
    pub fn full() -> Self {
        Self {
            read: true,
            write: true,
            execute: true,
        }
    }
}
