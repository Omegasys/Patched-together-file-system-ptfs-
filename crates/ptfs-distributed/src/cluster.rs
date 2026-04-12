use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u128,
    pub address: String,
}

pub struct Cluster {
    nodes: HashMap<u128, Node>,
    local_node: u128,
}

impl Cluster {
    pub fn new(local_node: u128) -> Self {
        Self {
            nodes: HashMap::new(),
            local_node,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn remove_node(&mut self, node_id: &u128) {
        self.nodes.remove(node_id);
    }

    pub fn get_node(&self, node_id: &u128) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    pub fn all_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }

    pub fn local_node(&self) -> u128 {
        self.local_node
    }
}
