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

mod digraph;
mod node;

pub use digraph::DiGraph;
pub use node::DiNode;

pub trait Graph {
    fn get_name(&self) -> Option<String>;
    fn set_name(&mut self, new_name: Option<&str>);
    fn add_node(&mut self, node: DiNode);
    fn add_edge(&mut self, name1: Option<&str>, name2: Option<&str>);
    fn get_nodes(&self) -> Vec<String>;
    fn get_node(&self, name: &str) -> Option<&DiNode>;
    fn get_node_mut(&mut self, name: &str) -> Option<&mut DiNode>;
    fn node_count(&self) -> usize;
}

pub trait Node {
    fn get_name(&self) -> String;
    fn set_name(&mut self, new_name: &str);
    fn degree(&self) -> usize;
    fn neighbors(&self) -> Vec<String>;
}
