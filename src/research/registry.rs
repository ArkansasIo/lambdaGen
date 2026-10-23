use super::{NodeKind, ResearchTree};

#[derive(Debug, Default)]
pub struct ResearchRegistry { pub tree: ResearchTree }

impl ResearchRegistry {
    pub fn new() -> Self { Self { tree: ResearchTree::new() } }
    pub fn with_default_conjectures() -> Self { Self { tree: super::default_conjecture_tree() } }
    pub fn register(&mut self, parent: &str, id: &str, name: &str, kind: NodeKind, description: &str) -> Result<(), String> {
        self.tree.add_child(parent, id, name, kind, description)
    }
}
