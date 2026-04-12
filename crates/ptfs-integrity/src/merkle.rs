use crate::checksum::Checksum;

/// Simple Merkle tree (binary)
#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: Checksum,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    pub fn leaf(data: &[u8]) -> Self {
        Self {
            hash: Checksum::compute(data),
            left: None,
            right: None,
        }
    }

    pub fn parent(left: MerkleNode, right: MerkleNode) -> Self {
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&left.hash.0);
        combined.extend_from_slice(&right.hash.0);

        Self {
            hash: Checksum::compute(&combined),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

/// Build Merkle tree from chunks
pub fn build_tree(chunks: &[Vec<u8>]) -> Option<MerkleNode> {
    if chunks.is_empty() {
        return None;
    }

    let mut nodes: Vec<MerkleNode> = chunks
        .iter()
        .map(|c| MerkleNode::leaf(c))
        .collect();

    while nodes.len() > 1 {
        let mut next = Vec::new();

        for pair in nodes.chunks(2) {
            if pair.len() == 2 {
                next.push(MerkleNode::parent(pair[0].clone(), pair[1].clone()));
            } else {
                next.push(pair[0].clone());
            }
        }

        nodes = next;
    }

    Some(nodes.remove(0))
}
