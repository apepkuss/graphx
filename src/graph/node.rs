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

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Node {
    pub name: String,
    pub predecessors: HashSet<String>,
    pub successors: HashSet<String>,
    weight: Option<String>,
}
impl Node {
    pub fn new(name: &str, weight: Option<String>) -> Self {
        Node {
            name: name.to_string(),
            predecessors: HashSet::new(),
            successors: HashSet::new(),
            weight,
        }
    }

    pub fn get_predecessors(&self) -> Vec<String> {
        self.predecessors.iter().map(|name| name.clone()).collect()
    }

    pub fn add_predecessor(&mut self, name: &str) {
        self.predecessors.insert(name.to_string());
    }

    pub fn remove_predecessor(&mut self, name: &str) {
        self.predecessors.remove(name);
    }

    pub fn get_successors(&self) -> Vec<String> {
        self.successors.iter().map(|name| name.clone()).collect()
    }

    pub fn add_successors(&mut self, name: &str) {
        self.successors.insert(name.to_string());
    }

    pub fn remove_successor(&mut self, name: &str) {
        self.successors.remove(name);
    }

    pub fn in_degree(&self) -> usize {
        self.predecessors.len()
    }

    pub fn out_degree(&self) -> usize {
        self.successors.len()
    }

    pub fn get_weight(&self) -> Option<String> {
        if self.weight.is_some() {
            return self.weight.clone();
        }
        None
    }
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_to_json() {
        let node = Node::new("A", None);
        let serialized = serde_json::to_string(&node).unwrap();
        assert_eq!(
            serialized,
            r#"{"name":"A","predecessors":[],"successors":[],"weight":null}"#
        );

        let mut node = Node::new("A", Some("weight".to_string()));
        node.add_predecessor("B");
        node.add_successors("C");
        let serialized = serde_json::to_string(&node).unwrap();
        assert_eq!(
            serialized,
            r#"{"name":"A","predecessors":["B"],"successors":["C"],"weight":"weight"}"#
        );
    }

    #[test]
    fn test_json_to_node() {
        let json_str = r#"{"name":"A","predecessors":["B"],"successors":["C"],"weight":"weight"}"#;
        let actual: Node = serde_json::from_str(json_str).unwrap();

        let mut expected = Node::new("A", Some("weight".to_string()));
        expected.add_predecessor("B");
        expected.add_successors("C");

        assert_eq!(expected, actual);
    }
}
