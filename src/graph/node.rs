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
use crate::algorithm::isomorphism::GMNode;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct DiNode {
    name: String,
    inputs: HashSet<String>,
    outputs: HashSet<String>,
    weight: Option<String>,
}
impl DiNode {
    pub fn new(name: &str, weight: Option<String>) -> Self {
        DiNode {
            name: name.to_string(),
            inputs: HashSet::new(),
            outputs: HashSet::new(),
            weight,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_predecessors(&self) -> Vec<String> {
        self.inputs.iter().map(|name| name.clone()).collect()
    }

    pub fn add_predecessor(&mut self, name: &str) {
        self.inputs.insert(name.to_string());
    }

    pub fn remove_predecessor(&mut self, name: &str) {
        self.inputs.remove(name);
    }

    pub fn get_successors(&self) -> Vec<String> {
        self.outputs.iter().map(|name| name.clone()).collect()
    }

    pub fn add_successor(&mut self, name: &str) {
        self.outputs.insert(name.to_string());
    }

    pub fn remove_successor(&mut self, name: &str) {
        self.outputs.remove(name);
    }

    pub fn in_degree(&self) -> usize {
        self.inputs.len()
    }

    pub fn out_degree(&self) -> usize {
        self.outputs.len()
    }

    pub fn get_weight(&self) -> Option<String> {
        if self.weight.is_some() {
            return self.weight.clone();
        }
        None
    }
}
impl Hash for DiNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl GMNode for DiNode {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_to_json() {
        let node = DiNode::new("A", None);
        let serialized = serde_json::to_string(&node).unwrap();
        assert_eq!(
            serialized,
            r#"{"name":"A","inputs":[],"outputs":[],"weight":null}"#
        );

        let mut node = DiNode::new("A", Some("weight".to_string()));
        node.add_predecessor("B");
        node.add_successor("C");
        let serialized = serde_json::to_string(&node).unwrap();
        assert_eq!(
            serialized,
            r#"{"name":"A","inputs":["B"],"outputs":["C"],"weight":"weight"}"#
        );
    }

    #[test]
    fn test_json_to_node() {
        let json_str = r#"{"name":"A","inputs":["B"],"outputs":["C"],"weight":"weight"}"#;
        let actual: DiNode = serde_json::from_str(json_str).unwrap();

        let mut expected = DiNode::new("A", Some("weight".to_string()));
        expected.add_predecessor("B");
        expected.add_successor("C");

        assert_eq!(expected, actual);
    }
}
