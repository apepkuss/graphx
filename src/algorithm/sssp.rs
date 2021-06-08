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

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

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

    // create a min_heap
    let mut min_heap = BinaryHeap::new();
    for (key, val) in dist.iter() {
        min_heap.push(Reverse((val, key)));
    }

    // spt_set: shortest path tree set that keeps track of nodes included in the shortest path tree
    let mut spt_set = HashSet::new();

    while spt_set.len() < graph.node_count() {
        let (name, distance) = min_distance(&dist);
        if !spt_set.contains(name.as_str()) {
            spt_set.insert(name.clone());

            // update distance from source to each child v of node
            let node = graph.get_node(name.as_str()).unwrap();
            let cnames = node.get_successors();
            for cname in cnames.iter() {
                let mut new_dist: usize = usize::MAX;
                {
                    new_dist = distance + graph.get_edge_weight(name.as_str(), cname).unwrap();
                }
                let cur_dist = dist.get_mut(cname).unwrap();
                if new_dist <= *cur_dist {
                    *cur_dist = new_dist;
                }
            }
        }
    }

    dist.clone()
}

fn min_distance(dist: &HashMap<String, usize>) -> (String, usize) {
    let mut d = &usize::MAX;
    let mut name = &String::new();
    for (key, val) in dist {
        if d > val {
            d = val;
            name = key;
        }
    }
    (name.clone(), d.clone())
}

pub trait SPGraph {
    type Node: SPNode;
    fn node_count(&self) -> usize;
    fn get_nodes(&self) -> Vec<String>;
    fn get_node(&self, name: &str) -> Option<&Self::Node>;
    fn get_successors(&self) -> Vec<String>;
    fn get_edge_weight(&self, source: &str, target: &str) -> Option<usize>;
}

pub trait SPNode {
    fn get_successors(&self) -> Vec<String>;
}

pub fn run() {
    let mut map = HashMap::new();
    map.insert("B", (2, "B"));
    map.insert("E", (5, "E"));
    map.insert("C", (3, "C"));

    let mut min_heap = BinaryHeap::new();
    for (_, val) in map.iter() {
        min_heap.push(Reverse(val));
    }

    assert_eq!(min_heap.peek(), Some(&Reverse(&(2, "B"))));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minheap() {
        run();
    }
}
