pub mod bindings;

use std::collections::HashMap;

/// Core API struct (entry point for PTFS)
pub struct Ptfs {
    mounted: bool,
    objects: HashMap<u128, Vec<u8>>, // placeholder object store
}

impl Ptfs {
    pub fn new() -> Self {
        Self {
            mounted: false,
            objects: HashMap::new(),
        }
    }

    pub fn mkfs(&mut self) {
        self.objects.clear();
        println!("PTFS: filesystem created");
    }

    pub fn mount(&mut self) {
        self.mounted = true;
        println!("PTFS: mounted");
    }

    pub fn is_mounted(&self) -> bool {
        self.mounted
    }

    pub fn write_object(&mut self, id: u128, data: Vec<u8>) {
        self.objects.insert(id, data);
    }

    pub fn read_object(&self, id: &u128) -> Option<&Vec<u8>> {
        self.objects.get(id)
    }

    pub fn snapshot(&self) {
        println!("PTFS: snapshot created (placeholder)");
    }

    pub fn scrub(&self) {
        println!("PTFS: scrub started (placeholder)");
    }
}
