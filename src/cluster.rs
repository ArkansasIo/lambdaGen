#[derive(Debug, Clone)]
pub struct ClusterNode { pub id: String, pub address: String, pub capacity: usize, pub healthy: bool }

#[derive(Debug, Default)]
pub struct Cluster { nodes: Vec<ClusterNode> }

impl Cluster {
    pub fn new() -> Self { Self::default() }
    pub fn add_node(&mut self, node: ClusterNode) { self.nodes.push(node); }
    pub fn healthy_nodes(&self) -> impl Iterator<Item=&ClusterNode> { self.nodes.iter().filter(|n| n.healthy) }
    pub fn total_capacity(&self) -> usize { self.healthy_nodes().map(|n| n.capacity).sum() }
    pub fn select_node(&self, hash: u64) -> Option<&ClusterNode> {
        let nodes: Vec<_> = self.healthy_nodes().collect();
        if nodes.is_empty() { None } else { Some(nodes[(hash as usize) % nodes.len()]) }
    }
}

#[derive(Debug, Clone)]
pub struct DistributedTask { pub id: String, pub algorithm: String, pub payload: String, pub preferred_node: Option<String> }
