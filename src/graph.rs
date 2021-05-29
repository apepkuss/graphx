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
pub use node::Node;

pub trait Graph {
    fn get_name(&self) -> Option<String>;
    fn set_name(&mut self, new_name: Option<&str>);
    fn add_node(&mut self, node: Node);
    fn add_edge(&mut self, name1: Option<&str>, name2: Option<&str>);
    fn get_nodes(&self) -> Vec<String>;
}
