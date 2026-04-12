use serde::{Deserialize, Serialize};

/// Messages exchanged between nodes
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Heartbeat { node_id: u128 },

    PutObject {
        object_id: u128,
        data: Vec<u8>,
    },

    GetObject {
        object_id: u128,
    },

    ObjectData {
        object_id: u128,
        data: Vec<u8>,
    },

    VoteRequest {
        term: u64,
        candidate_id: u128,
    },

    VoteResponse {
        term: u64,
        vote_granted: bool,
    },
}
