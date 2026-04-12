use blake3::Hasher;

#[derive(Clone)]
pub struct Object {
    pub id: u128,
    pub data: Vec<u8>,
    pub checksum: [u8; 32],
}

impl Object {
    pub fn new(data: Vec<u8>) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&data);
        let checksum = *hasher.finalize().as_bytes();

        Self {
            id: rand::random(),
            data,
            checksum,
        }
    }
}
