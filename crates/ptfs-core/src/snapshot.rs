use std::collections::HashMap;
use crate::inode::Inode;

#[derive(Clone)]
pub struct Snapshot {
    pub id: u128,
    pub state: HashMap<u128, Inode>,
}

impl Snapshot {
    pub fn new(id: u128, state: &HashMap<u128, Inode>) -> Self {
        Self {
            id,
            state: state.clone(),
        }
    }
}
