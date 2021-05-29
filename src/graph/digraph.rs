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

use super::Graph;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DiGraph {
    name: Option<String>,
    pub nodes: HashMap<String, Node>,
}
impl DiGraph {
    pub fn new(name: Option<String>) -> Self {
        DiGraph {
            name,
            nodes: HashMap::new(),
        }
    }

    pub fn predecessors(&self, name: &str) -> Result<Vec<&Node>, NotFoundNodeError> {
        if !self.nodes.contains_key(name) {
            return Err(NotFoundNodeError {
                message: format!("Not found node: {}", name),
            });
        }

        let node = self
            .nodes
            .get(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        Ok(node
            .predecessors
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect())
    }

    pub fn successors(&self, name: &str) -> Result<Vec<&Node>, NotFoundNodeError> {
        if !self.nodes.contains_key(name) {
            return Err(NotFoundNodeError {
                message: format!("Not found node: {}", name),
            });
        }

        let node = self
            .get_node(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        Ok(node
            .successors
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect())
    }

    pub fn in_degree(&self, name: &str) -> Result<usize, NotFoundNodeError> {
        if !self.nodes.contains_key(name) {
            return Err(NotFoundNodeError {
                message: format!("Not found node: {}", name),
            });
        }

        let node = self.nodes.get(name).unwrap();
        Ok(node.in_degree())
    }

    pub fn out_degree(&self, name: &str) -> Result<usize, NotFoundNodeError> {
        if !self.nodes.contains_key(name) {
            return Err(NotFoundNodeError {
                message: format!("Not found node: {}", name),
            });
        }

        let node = self.nodes.get(name).unwrap();
        Ok(node.out_degree())
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
        let result_succ = self.successors(from);
        match result_succ {
            Ok(successor_vec) => {
                for succ in successor_vec {
                    if succ.name == to {
                        count += 1;
                    }
                }
            }
            Err(err) => panic!("{}", err.message),
        }
        count
    }

    pub fn contains_node(&self, name: &str) -> bool {
        self.nodes.contains_key(name)
    }
}
impl Graph for DiGraph {
    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.clone(), node);
    }

    fn add_edge(&mut self, from: Option<&str>, to: Option<&str>) {
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
}

#[derive(Debug)]
pub struct NotFoundNodeError {
    pub message: String,
}
impl std::fmt::Display for NotFoundNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not found node")
    }
}
impl std::error::Error for NotFoundNodeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digraph_to_json() {
        let mut g = DiGraph::new(None);
        g.add_edge(Some("A"), Some("B"));

        let expected1 = r#"{"name":null,"nodes":{"B":{"name":"B","predecessors":["A"],"successors":[],"weight":null},"A":{"name":"A","predecessors":[],"successors":["B"],"weight":null}}}"#;
        let expected2 = r#"{"name":null,"nodes":{"A":{"name":"A","predecessors":[],"successors":["B"],"weight":null},"B":{"name":"B","predecessors":["A"],"successors":[],"weight":null}}}"#;
        let actual = serde_json::to_string(&g).unwrap();
        assert!(expected1 == actual || expected2 == actual);
    }

    #[test]
    fn test_json_to_digraph() {
        let json_str = r#"{"nodes":{"B":{"name":"B","predecessors":["A"],"successors":[]},"A":{"name":"A","predecessors":[],"successors":["B"]}}}"#;
        let actual: DiGraph = serde_json::from_str(json_str).unwrap();

        let mut g = DiGraph::new(None);
        g.add_edge(Some("A"), Some("B"));

        assert_eq!(g, actual);
    }
}
