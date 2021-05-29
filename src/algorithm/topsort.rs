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

use crate::graph::{DiGraph, DiNode, Graph, Node};
use std::collections::{HashMap, VecDeque};

/// topological sort
pub fn topsort(graph: &DiGraph) -> Vec<String> {
    let mut map: HashMap<&DiNode, usize> = HashMap::new();
    for name in graph.get_nodes().iter() {
        let node = graph.get_node(name).unwrap();
        map.insert(node, node.in_degree());
    }

    let mut queue = VecDeque::new();
    for (&key, val) in map.iter() {
        if *val == 0 {
            queue.push_back(key);
        }
    }

    let mut names = Vec::new();
    while queue.len() > 0 {
        let curr_node = queue.pop_front().unwrap();
        names.push(curr_node.get_name());
        for name in curr_node.get_successors() {
            let succ = graph.get_node(name.as_str()).unwrap();
            let degree = map.get_mut(succ).unwrap();
            *degree -= 1 as usize;
            if *degree == 0 {
                queue.push_back(succ);
            }
        }
    }

    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{DiGraph, DiNode};
    #[test]
    fn test_topsort_digraph() {
        let mut g = DiGraph::new(None);
        g.add_node(DiNode::new("A", Some("A".to_string())));
        g.add_node(DiNode::new("B", Some("B".to_string())));
        g.add_node(DiNode::new("C", Some("C".to_string())));
        g.add_node(DiNode::new("D", Some("D".to_string())));
        g.add_node(DiNode::new("E", Some("E".to_string())));
        g.add_node(DiNode::new("F", Some("F".to_string())));
        g.add_node(DiNode::new("G", Some("G".to_string())));
        g.add_node(DiNode::new("H", Some("H".to_string())));
        g.add_node(DiNode::new("I", Some("I".to_string())));
        g.add_node(DiNode::new("J", Some("J".to_string())));
        g.add_edge(Some("A"), Some("B"));
        g.add_edge(Some("B"), Some("C"));
        g.add_edge(Some("C"), Some("E"));
        g.add_edge(Some("D"), Some("E"));
        g.add_edge(Some("E"), Some("F"));
        g.add_edge(Some("F"), Some("G"));
        g.add_edge(Some("G"), Some("I"));
        g.add_edge(Some("H"), Some("I"));
        g.add_edge(Some("I"), Some("J"));

        let names = topsort(&g);
        assert!(names.len() == g.node_count());
    }
}
