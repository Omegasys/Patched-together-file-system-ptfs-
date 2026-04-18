#[derive(Debug, Clone)]
pub struct DriveHealth {
    pub temperature_c: f32,
    pub power_on_hours: u64,
    pub read_errors: u64,
    pub write_errors: u64,
    pub healthy: bool,
}

impl DriveHealth {
    pub fn new() -> Self {
        Self {
            temperature_c: 30.0,
            power_on_hours: 0,
            read_errors: 0,
            write_errors: 0,
            healthy: true,
        }
    }

    pub fn update_errors(&mut self, read: u64, write: u64) {
        self.read_errors += read;
        self.write_errors += write;

        if self.read_errors > 100 || self.write_errors > 100 {
            self.healthy = false;
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy
    }
}
