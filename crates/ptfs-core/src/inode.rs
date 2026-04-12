use crate::object::Object;
use anyhow::Result;

pub struct Inode {
    pub id: u128,
    pub objects: Vec<Object>,
}

impl Inode {
    pub fn new(id: u128) -> Self {
        Self {
            id,
            objects: Vec::new(),
        }
    }

    pub fn write(&mut self, data: Vec<u8>) -> Result<()> {
        let obj = Object::new(data);
        self.objects.push(obj);
        Ok(())
    }

    pub fn read(&self) -> Result<Vec<u8>> {
        let mut result = Vec::new();
        for obj in &self.objects {
            result.extend(&obj.data);
        }
        Ok(result)
    }
}
