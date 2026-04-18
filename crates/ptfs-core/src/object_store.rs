use std::collections::HashMap;

pub type ObjectId = u128;

#[derive(Debug, Clone)]
pub struct Object {
    pub id: ObjectId,
    pub data: Vec<u8>,
    pub checksum: [u8; 32], // placeholder for blake3
}

pub struct ObjectStore {
    objects: HashMap<ObjectId, Object>,
    next_id: ObjectId,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 1,
        }
    }

    fn generate_id(&mut self) -> ObjectId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn compute_checksum(data: &[u8]) -> [u8; 32] {
        // placeholder (replace with blake3 later)
        let mut checksum = [0u8; 32];
        for (i, byte) in data.iter().enumerate() {
            checksum[i % 32] ^= *byte;
        }
        checksum
    }

    pub fn write_object(&mut self, data: Vec<u8>) -> ObjectId {
        let id = self.generate_id();
        let checksum = Self::compute_checksum(&data);

        let obj = Object { id, data, checksum };
        self.objects.insert(id, obj);

        id
    }

    pub fn read_object(&self, id: &ObjectId) -> Option<&Object> {
        self.objects.get(id)
    }

    pub fn verify(&self, id: &ObjectId) -> bool {
        match self.objects.get(id) {
            Some(obj) => Self::compute_checksum(&obj.data) == obj.checksum,
            None => false,
        }
    }

    pub fn delete_object(&mut self, id: &ObjectId) {
        self.objects.remove(id);
    }
}
