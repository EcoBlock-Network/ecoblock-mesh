use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub last_seen: u64, // timestamp
}

#[derive(Debug)]
pub struct MeshTopology {
    peers: HashMap<String, Peer>,
}

impl MeshTopology {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn add_or_update_peer(&mut self, id: &str, timestamp: u64) {
        self.peers.insert(
            id.to_string(),
            Peer {
                id: id.to_string(),
                last_seen: timestamp,
            },
        );
    }

    pub fn remove_peer(&mut self, id: &str) {
        self.peers.remove(id);
    }

    pub fn list_peers(&self) -> Vec<Peer> {
        self.peers.values().cloned().collect()
    }
}
