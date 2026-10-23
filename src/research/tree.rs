use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind { Tree, Branch, SubBranch, Module, SubModule, Algorithm, Experiment, Result }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchNode {
    pub id: String,
    pub name: String,
    pub kind: NodeKind,
    pub description: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ResearchTree { nodes: BTreeMap<String, ResearchNode> }

impl ResearchTree {
    pub fn new() -> Self { Self::default() }
    pub fn root(id: &str, name: &str, description: &str) -> Self {
        let mut tree = Self::new();
        tree.insert(ResearchNode { id: id.into(), name: name.into(), kind: NodeKind::Tree, description: description.into(), parent: None, children: vec![] });
        tree
    }
    pub fn insert(&mut self, node: ResearchNode) { self.nodes.insert(node.id.clone(), node); }
    pub fn add_child(&mut self, parent: &str, id: &str, name: &str, kind: NodeKind, description: &str) -> Result<(), String> {
        if self.nodes.contains_key(id) { return Err(format!("duplicate node: {id}")); }
        if !self.nodes.contains_key(parent) { return Err(format!("missing parent: {parent}")); }
        self.insert(ResearchNode { id: id.into(), name: name.into(), kind, description: description.into(), parent: Some(parent.into()), children: vec![] });
        self.nodes.get_mut(parent).unwrap().children.push(id.into());
        Ok(())
    }
    pub fn get(&self, id: &str) -> Option<&ResearchNode> { self.nodes.get(id) }
    pub fn children(&self, id: &str) -> Vec<&ResearchNode> { self.nodes.get(id).map(|n| n.children.iter().filter_map(|c| self.nodes.get(c)).collect()).unwrap_or_default() }
    pub fn path(&self, id: &str) -> Vec<String> {
        let mut path = Vec::new(); let mut current = Some(id.to_string());
        while let Some(value) = current { if let Some(node) = self.get(&value) { path.push(node.id.clone()); current = node.parent.clone(); } else { break; } }
        path.reverse(); path
    }
    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
}
