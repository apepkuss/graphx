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

use super::node::DiNode;
use crate::{
    algorithm::{isomorphism::GMGraph, topsort::TSortGraph},
    error::{GraphError, GraphErrorKind},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DiGraph {
    name: Option<String>,
    nodes: HashMap<String, DiNode>,
}
impl DiGraph {
    pub fn new(name: Option<String>) -> Self {
        DiGraph {
            name,
            nodes: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: Option<&str>) {
        match name {
            Some(name) => self.name = Some(name.to_string()),
            _ => self.name = None,
        }
    }

    pub fn add_node(&mut self, node: DiNode) {
        self.nodes.insert(node.get_name().clone(), node);
    }

    pub fn add_edge(&mut self, from: Option<&str>, to: Option<&str>) {
        if from.is_some() {
            // create a new node
            let name = from.unwrap();
            if !self.contains_node(name) {
                self.nodes
                    .entry(name.to_string())
                    .or_insert(DiNode::new(name, None));
            }
        }

        if to.is_some() {
            // create a new node
            let name = to.unwrap();
            if !self.contains_node(name) {
                self.nodes
                    .entry(name.to_string())
                    .or_insert(DiNode::new(name, None));
            }
        }

        if from.is_some() && to.is_some() {
            // update predecessors and successros of new nodes

            let source = self.nodes.get_mut(from.unwrap()).unwrap();
            source.add_successor(to.unwrap());

            let target = self.nodes.get_mut(to.unwrap()).unwrap();
            target.add_predecessor(from.unwrap());
        }
    }

    pub fn get_node(&self, name: &str) -> Option<&DiNode> {
        self.nodes.get(name)
    }

    pub fn get_node_mut(&mut self, name: &str) -> Option<&mut DiNode> {
        self.nodes.get_mut(name)
    }

    pub fn get_nodes(&self) -> Vec<String> {
        let mut names = Vec::new();
        for name in self.nodes.keys() {
            names.push(name.clone());
        }
        names
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn predecessors(&self, name: &str) -> Result<Vec<&DiNode>, GraphError> {
        if !self.nodes.contains_key(name) {
            return Err(GraphError {
                message: format!("Not found node: {}", name),
                kind: GraphErrorKind::NotFoundNodeError(format!("Not found node: {}", name)),
            });
        }

        let node = self
            .nodes
            .get(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        Ok(node
            .get_predecessors()
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect())
    }

    pub fn successors(&self, name: &str) -> Result<Vec<&DiNode>, GraphError> {
        if !self.nodes.contains_key(name) {
            return Err(GraphError {
                message: format!("Not found node: {}", name),
                kind: GraphErrorKind::NotFoundNodeError(format!("Not found node: {}", name)),
            });
        }

        let node = self
            .get_node(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        Ok(node
            .get_successors()
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect())
    }

    pub fn in_degree(&self, name: &str) -> Result<usize, GraphError> {
        if !self.nodes.contains_key(name) {
            return Err(GraphError {
                message: format!("Not found node: {}", name),
                kind: GraphErrorKind::NotFoundNodeError(format!("Not found node: {}", name)),
            });
        }

        let node = self.nodes.get(name).unwrap();
        Ok(node.in_degree())
    }

    pub fn out_degree(&self, name: &str) -> Result<usize, GraphError> {
        if !self.nodes.contains_key(name) {
            return Err(GraphError {
                message: format!("Not found node: {}", name),
                kind: GraphErrorKind::NotFoundNodeError(format!("Not found node: {}", name)),
            });
        }

        let node = self.nodes.get(name).unwrap();
        Ok(node.out_degree())
    }

    pub fn edge_count(&self, from: &str, to: &str) -> usize {
        let mut count = 0 as usize;
        let result_succ = self.successors(from);
        match result_succ {
            Ok(successor_vec) => {
                for succ in successor_vec {
                    if succ.get_name() == to {
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
impl GMGraph for DiGraph {
    type Node = DiNode;

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self, from: &str, to: &str) -> usize {
        let mut count = 0 as usize;
        let result_succ = self.successors(from);
        match result_succ {
            Ok(successor_vec) => {
                for succ in successor_vec {
                    if succ.get_name() == to {
                        count += 1;
                    }
                }
            }
            Err(err) => panic!("{}", err.message),
        }
        count
    }

    fn get_node(&self, name: &str) -> Option<&DiNode> {
        self.nodes.get(name)
    }

    fn get_nodes(&self) -> Vec<String> {
        let mut names = Vec::new();
        for name in self.nodes.keys() {
            names.push(name.clone());
        }
        names
    }

    fn predecessors(&self, name: &str) -> Result<Vec<&DiNode>, GraphError> {
        if !self.nodes.contains_key(name) {
            return Err(GraphError {
                message: format!("Not found node: {}", name),
                kind: GraphErrorKind::NotFoundNodeError(format!("Not found node: {}", name)),
            });
        }

        let node = self
            .nodes
            .get(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        Ok(node
            .get_predecessors()
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect())
    }

    fn successors(&self, name: &str) -> Result<Vec<&DiNode>, GraphError> {
        if !self.nodes.contains_key(name) {
            return Err(GraphError {
                message: format!("Not found node: {}", name),
                kind: GraphErrorKind::NotFoundNodeError(format!("Not found node: {}", name)),
            });
        }

        let node = GMGraph::get_node(self, name)
            .expect(format!("Not found node with name: {}", name).as_str());
        Ok(node
            .get_successors()
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect())
    }
}
impl TSortGraph for DiGraph {
    type Node = DiNode;

    fn get_nodes(&self) -> Vec<&DiNode> {
        self.nodes.values().map(|x| x).collect()
    }

    fn get_node(&self, name: &str) -> Option<&DiNode> {
        self.nodes.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digraph_to_json() {
        let mut g = DiGraph::new(None);
        g.add_edge(Some("A"), Some("B"));

        let expected1 = r#"{"name":null,"nodes":{"B":{"name":"B","inputs":["A"],"outputs":[],"weight":null},"A":{"name":"A","inputs":[],"outputs":["B"],"weight":null}}}"#;
        let expected2 = r#"{"name":null,"nodes":{"A":{"name":"A","inputs":[],"outputs":["B"],"weight":null},"B":{"name":"B","inputs":["A"],"outputs":[],"weight":null}}}"#;
        let actual = serde_json::to_string(&g).unwrap();
        assert!(expected1 == actual || expected2 == actual);
    }

    #[test]
    fn test_json_to_digraph() {
        let json_str = r#"{"nodes":{"B":{"name":"B","inputs":["A"],"outputs":[]},"A":{"name":"A","inputs":[],"outputs":["B"]}}}"#;
        let actual: DiGraph = serde_json::from_str(json_str).unwrap();

        let mut g = DiGraph::new(None);
        g.add_edge(Some("A"), Some("B"));

        assert_eq!(g, actual);
    }
}
