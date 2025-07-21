use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Default, Clone)]
pub struct TopologyGraph {
    pub connections: HashMap<String, Vec<(String, f32)>>,
}

impl TopologyGraph {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, from: &str, to: &str, weight: f32) {
        self.connections
            .entry(from.to_string())
            .or_default()
            .push((to.to_string(), weight));
    }

    pub fn get_neighbors(&self, id: &str) -> Option<Vec<(String, f32)>> {
        self.connections.get(id).cloned()
    }

    pub fn add_node(&mut self, id: &str) {
        self.connections.entry(id.to_string()).or_insert_with(Vec::new);
    }

    pub fn remove_node(&mut self, id: &str) {
        self.connections.remove(id);
        for neighbors in self.connections.values_mut() {
            neighbors.retain(|(target, _)| target != id);
        }
    }

    pub fn shortest_path(&self, start: &str, goal: &str) -> Option<Vec<String>> {
        let mut dist: HashMap<String, f32> = HashMap::new();
        let mut prev: HashMap<String, String> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start.to_string(), 0.0);
        heap.push(State {
            cost: 0.0,
            node: start.to_string(),
        });

        while let Some(State { cost, node }) = heap.pop() {
            if node == goal {
                let mut path = vec![goal.to_string()];
                let mut current = goal.to_string();
                while let Some(p) = prev.get(&current) {
                    path.push(p.clone());
                    current = p.clone();
                }
                path.reverse();
                return Some(path);
            }

            if let Some(neighbors) = self.connections.get(&node) {
                for (neighbor, weight) in neighbors {
                    let next = State {
                        cost: cost + weight,
                        node: neighbor.clone(),
                    };

                    let d = dist.get(neighbor).copied().unwrap_or(f32::INFINITY);
                    if next.cost < d {
                        let next_cost = next.cost;
                        heap.push(next);
                        dist.insert(neighbor.clone(), next_cost);
                        prev.insert(neighbor.clone(), node.clone());
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct State {
    cost: f32,
    node: String,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
