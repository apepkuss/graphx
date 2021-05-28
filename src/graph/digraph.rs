// Copyright 2021 apepkuss
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::node::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DiGraph {
    name: Option<String>,
    pub nodes: HashMap<String, Node>,
}
impl DiGraph {
    pub fn new(name: Option<String>) -> Self {
        DiGraph {
            name: name,
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.clone(), node);
    }

    pub fn add_edge(&mut self, from: Option<&str>, to: Option<&str>) {
        if from.is_some() {
            // create a new node
            let name = from.unwrap();
            if !self.contains_node(name) {
                self.nodes
                    .entry(name.to_string())
                    .or_insert(Node::new(name, None));
            }
        }

        if to.is_some() {
            // create a new node
            let name = to.unwrap();
            if !self.contains_node(name) {
                self.nodes
                    .entry(name.to_string())
                    .or_insert(Node::new(name, None));
            }
        }

        if from.is_some() && to.is_some() {
            // update predecessors and successros of new nodes

            let source = self.nodes.get_mut(from.unwrap()).unwrap();
            source.successors.insert(to.unwrap().to_string());

            let target = self.nodes.get_mut(to.unwrap()).unwrap();
            target.predecessors.insert(from.unwrap().to_string());
        }
    }

    pub fn predecessors(&self, name: &str) -> Vec<&Node> {
        let node = self
            .nodes
            .get(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        node.predecessors
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect()
    }

    pub fn successors(&self, name: &str) -> Vec<&Node> {
        let node = self
            .get_node(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        node.successors
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect()
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.get(name)
    }

    pub fn get_node_mut(&mut self, name: &str) -> Option<&mut Node> {
        self.nodes.get_mut(name)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self, from: &str, to: &str) -> usize {
        let mut count = 0 as usize;
        for succ in self.successors(from) {
            if succ.name == to {
                count += 1;
            }
        }
        count
    }

    pub fn contains_node(&self, name: &str) -> bool {
        self.nodes.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digraph_to_json() {
        let mut g = DiGraph::new(None);
        g.add_edge(Some("A"), Some("B"));

        let expected = r#"{"name":null,"nodes":{"B":{"name":"B","predecessors":["A"],"successors":[],"weight":null},"A":{"name":"A","predecessors":[],"successors":["B"],"weight":null}}}"#;
        let actual = serde_json::to_string(&g).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_json_to_digraph() {
        let json_str = r#"{"name":null,"nodes":{"B":{"name":"B","predecessors":["A"],"successors":[],"weight":null},"A":{"name":"A","predecessors":[],"successors":["B"],"weight":null}}}"#;
        let actual: DiGraph = serde_json::from_str(json_str).unwrap();

        let mut g = DiGraph::new(None);
        g.add_edge(Some("A"), Some("B"));

        assert_eq!(g, actual);
    }
}
