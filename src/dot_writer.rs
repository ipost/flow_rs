use crate::edge::Edge;
use crate::node::Node;

pub struct DotWriter {
    dot: String,
}

impl DotWriter {
    pub fn new() -> DotWriter {
        DotWriter {
            dot: "".to_string(),
        }
    }

    pub fn write_edge(&mut self, edge: &Edge, end: &Node) {
        let mut writeable = format!("{} -> {}", edge.start_id(), end.id);
        if let Some(label) = &edge.label {
            writeable.push_str(&format!(" [label=\"{}\"]", label))
        }
        writeable.push_str(";");
        self.write_line(&writeable);
    }

    pub fn write_node(&mut self, node: &Node) {
        let mut writeable = format!("{} [", node.id,);
        for attr in &node.attrs {
            writeable.push_str(&format!("{}, ", attr.as_dot()));
        }
        writeable.push_str("];");
        self.write_line(&writeable);
    }

    pub fn write_line(&mut self, line: &str) {
        self.dot.push_str(&format!("{}\n", line.trim()));
    }

    pub fn consume(self) -> String {
        self.dot
    }
}
