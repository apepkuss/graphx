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

use std::collections::HashMap;

pub fn dijkstra(graph: &impl SPGraph, source: &str) -> HashMap<String, usize> {
    // dist[i]: distance from source to i
    let mut dist = HashMap::new();
    for name in graph.get_nodes().iter() {
        if name == source {
            dist.insert(name.clone(), 0);
        } else {
            dist.insert(name.clone(), usize::MAX);
        }
    }

    println!("dist: {:?}", dist);

    // spt_set: shortest path tree set that keeps track of nodes included in the shortest path tree
    let mut spt = HashMap::new();
    while spt.len() < graph.node_count() {
        let (name, distance) = min_distance(&dist);
        dist.remove(name.as_str());
        if !spt.contains_key(name.as_str()) {
            spt.insert(name.clone(), distance.clone());

            // update distance from source to each child v of node
            let cnames = graph.get_successors(name.as_str());
            if cnames.is_some() {
                let cnames = cnames.unwrap();
                for cname in cnames.iter() {
                    if dist.contains_key(cname.as_str()) {
                        let new_dist =
                            distance + graph.get_edge_weight(name.as_str(), cname).unwrap();
                        let cur_dist = dist.get_mut(cname).unwrap();
                        if new_dist <= *cur_dist {
                            *cur_dist = new_dist;
                        }
                    }
                }
            }
        }
    }
    spt
}

fn min_distance(dist: &HashMap<String, usize>) -> (String, usize) {
    let mut d = &usize::MAX;
    let mut name = &String::new();
    for (key, val) in dist.iter() {
        if d > val {
            d = val;
            name = key;
        }
    }
    (name.clone(), d.clone())
}

pub trait SPGraph {
    fn node_count(&self) -> usize;
    fn get_nodes(&self) -> Vec<String>;
    fn get_successors(&self, name: &str) -> Option<Vec<String>>;
    fn get_edge_weight(&self, source: &str, target: &str) -> Option<usize>;
}

pub struct MyGraph {
    edges: HashMap<String, HashMap<String, Option<usize>>>,
}
impl MyGraph {
    pub fn new() -> Self {
        MyGraph {
            edges: HashMap::new(),
        }
    }
    pub fn add_edge(&mut self, source: &str, target: &str, weight: usize) {
        if source == target {
            panic!("Cannot add a self loop");
        }
        if !self.edges.contains_key(source) {
            self.edges.insert(source.to_string(), HashMap::new());
        }

        if !self.edges.contains_key(target) {
            self.edges.insert(target.to_string(), HashMap::new());
        }

        let map = self.edges.get_mut(source).unwrap();
        map.entry(target.to_string())
            .and_modify(|x| *x = Some(weight))
            .or_insert(Some(weight));
    }
}
impl SPGraph for MyGraph {
    fn node_count(&self) -> usize {
        self.edges.len()
    }
    fn get_nodes(&self) -> Vec<String> {
        self.edges.keys().map(|x| x.clone()).collect()
    }
    fn get_successors(&self, name: &str) -> Option<Vec<String>> {
        let succs = self.edges.get(name);
        if succs.is_none() {
            return None;
        }

        let names: Vec<String> = succs
            .unwrap()
            .iter()
            .filter(|&(key, val)| key.as_str() != name && val.is_some())
            .map(|(x, _)| x.clone())
            .collect();
        if names.len() == 0 {
            return None;
        }
        Some(names)
    }
    fn get_edge_weight(&self, source: &str, target: &str) -> Option<usize> {
        let succs = self.edges.get(source);
        if succs.is_none() {
            return None;
        }

        let succs = succs.unwrap();
        if !succs.contains_key(target) {
            return None;
        }
        let weight = succs.get(target).unwrap();
        if weight.is_none() {
            return None;
        }
        Some(weight.unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sssp_dijkstra() {
        let mut g = MyGraph::new();
        g.add_edge("0", "1", 4);
        g.add_edge("0", "7", 8);
        g.add_edge("1", "7", 11);
        g.add_edge("1", "2", 8);
        g.add_edge("2", "3", 7);
        g.add_edge("2", "5", 4);
        g.add_edge("2", "8", 2);
        g.add_edge("3", "4", 9);
        g.add_edge("3", "5", 14);
        g.add_edge("4", "5", 10);
        g.add_edge("5", "6", 2);
        g.add_edge("6", "7", 1);
        g.add_edge("6", "8", 6);
        g.add_edge("7", "8", 7);

        let actual = dijkstra(&g, "0");

        let tuples = vec![
            ("7", 8),
            ("0", 0),
            ("8", 14),
            ("5", 16),
            ("1", 4),
            ("4", 28),
            ("2", 12),
            ("6", 18),
            ("3", 19),
        ];
        let expected: HashMap<String, usize> = tuples
            .into_iter()
            .map(|(x, y)| (x.to_string(), y))
            .collect();
        assert_eq!(expected, actual);
    }
}
