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

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Clone)]
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
            weight: weight,
        }
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
