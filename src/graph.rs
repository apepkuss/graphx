use std::collections::HashMap;

pub type NodeIndex = String;

#[derive(Debug, Eq, PatialEq)]
pub struct Node {
    name: NodeIndex,
    predecessors: Vec<NodeIndex>,
    successors: Vec<NodeIndex>,
}
pub impl Node {
    fn new(name: String) -> Self {
        Node {
            name: name,
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }
}

pub struct DiGraph {
    nodes: HashMap<NodeIndex, Node>
}
pub impl DiGraph {
    fn new () -> Self {
        DiGraph {
            nodes: HashMap::new(),
        }
    }
}
