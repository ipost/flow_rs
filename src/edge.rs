use crate::node::Node;

#[derive(Clone)]
pub struct Edge {
    pub label: Option<String>,
    pub start_node: Node,
}

impl Edge {
    pub fn starting_at(n: Node) -> Self {
        Edge {
            start_node: n,
            label: None,
        }
    }

    pub fn labelled(mut self, l: String) -> Self {
        self.label = Some(l);
        self
    }

    pub fn start_id(&self) -> &str {
        &self.start_node.id
    }
}
