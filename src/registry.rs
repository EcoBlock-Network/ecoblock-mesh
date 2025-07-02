use std::collections::HashMap;

pub type NodeId = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeInfo {
    pub id: NodeId,
    pub label: String,
}

#[derive(Default, Debug)]
pub struct NodeRegistry {
    nodes: HashMap<NodeId, NodeInfo>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn register_node(&mut self, id: &str, label: &str) {
        let node = NodeInfo {
            id: id.to_string(),
            label: label.to_string(),
        };
        self.nodes.insert(id.to_string(), node);
    }

    pub fn unregister_node(&mut self, id: &str) {
        self.nodes.remove(id);
    }

    pub fn get_node(&self, id: &str) -> Option<&NodeInfo> {
        self.nodes.get(id)
    }

    pub fn list_nodes(&self) -> Vec<NodeInfo> {
        self.nodes.values().cloned().collect()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn count(&self) -> usize {
        self.nodes.len()
    }
}
