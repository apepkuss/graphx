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

use graphx::{
    algorithm::isomorphism as iso,
    graph::{DiGraph, DiNode},
};

#[test]
fn iso_digraph_weight_test() {
    let mut g1 = DiGraph::new(None);
    g1.add_node(DiNode::new("A", Some("A".to_string())));
    g1.add_node(DiNode::new("B", Some("B".to_string())));
    g1.add_node(DiNode::new("C", Some("C".to_string())));
    g1.add_node(DiNode::new("D", Some("D".to_string())));
    g1.add_node(DiNode::new("E", Some("E".to_string())));
    g1.add_node(DiNode::new("F", Some("F".to_string())));
    g1.add_node(DiNode::new("G", Some("G".to_string())));
    g1.add_node(DiNode::new("H", Some("H".to_string())));
    g1.add_node(DiNode::new("I", Some("I".to_string())));
    g1.add_node(DiNode::new("J", Some("J".to_string())));
    g1.add_edge(Some("A"), Some("B"));
    g1.add_edge(Some("B"), Some("C"));
    g1.add_edge(Some("C"), Some("E"));
    g1.add_edge(Some("D"), Some("E"));
    g1.add_edge(Some("E"), Some("F"));
    g1.add_edge(Some("F"), Some("G"));
    g1.add_edge(Some("G"), Some("I"));
    g1.add_edge(Some("H"), Some("I"));
    g1.add_edge(Some("I"), Some("J"));

    let mut g2 = DiGraph::new(None);
    g2.add_node(DiNode::new("1", Some("B".to_string())));
    g2.add_node(DiNode::new("2", Some("C".to_string())));
    g2.add_node(DiNode::new("3", Some("D".to_string())));
    g2.add_node(DiNode::new("4", Some("E".to_string())));
    g2.add_edge(Some("1"), Some("2"));
    g2.add_edge(Some("2"), Some("4"));
    g2.add_edge(Some("3"), Some("4"));

    let mut matcher = iso::DiGraphMatcher::new(&g1, &g2);
    let mut mapping = Vec::new();
    matcher.subgraph_isomorphism_iter(&mut mapping);

    assert_eq!(mapping.len(), 1);
    assert!(mapping[0].contains_key("1") && mapping[0].get("1").unwrap() == "B");
    assert!(mapping[0].contains_key("2") && mapping[0].get("2").unwrap() == "C");
    assert!(mapping[0].contains_key("3") && mapping[0].get("3").unwrap() == "D");
    assert!(mapping[0].contains_key("4") && mapping[0].get("4").unwrap() == "E");
}

#[test]
fn iso_digraph_test() {
    let mut g1 = DiGraph::new(None);
    g1.add_edge(Some("A"), Some("B"));
    g1.add_edge(Some("B"), Some("C"));
    g1.add_edge(Some("C"), Some("E"));
    g1.add_edge(Some("D"), Some("E"));
    g1.add_edge(Some("E"), Some("F"));
    g1.add_edge(Some("F"), Some("G"));
    g1.add_edge(Some("G"), Some("I"));
    g1.add_edge(Some("H"), Some("I"));
    g1.add_edge(Some("I"), Some("J"));

    let mut g2 = DiGraph::new(None);
    g2.add_edge(Some("1"), Some("2"));
    g2.add_edge(Some("2"), Some("4"));
    g2.add_edge(Some("3"), Some("4"));

    let mut matcher = iso::DiGraphMatcher::new(&g1, &g2);
    let mut mapping = Vec::new();
    matcher.subgraph_isomorphism_iter(&mut mapping);

    assert_eq!(mapping.len(), 2);
}
