use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct DiGraph {
    pub nodes: HashMap<String, Node>,
}
impl DiGraph {
    pub fn new() -> Self {
        DiGraph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.clone(), node);
    }

    pub fn add_edge(&mut self, from: Option<&str>, to: Option<&str>) {
        if from.is_some() {
            let name = from.unwrap();
            self.nodes
                .entry(name.to_string())
                .or_insert(Node::new(name));
        }

        if to.is_some() {
            let name = to.unwrap();
            self.nodes
                .entry(name.to_string())
                .or_insert(Node::new(name));
        }

        if from.is_some() && to.is_some() {
            let source = self.nodes.get_mut(from.unwrap()).unwrap();
            source.successors.insert(to.unwrap().to_string());

            let target = self.nodes.get_mut(to.unwrap()).unwrap();
            target.predecessors.insert(from.unwrap().to_string());
        }
    }

    pub fn predecessors(&self, name: &str) -> Vec<&Node> {
        let node = self
            .nodes
            .get(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        node.predecessors
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect()
    }

    pub fn successors(&self, name: &str) -> Vec<&Node> {
        let node = self
            .node(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        node.successors
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect()
    }

    pub fn node(&self, name: &str) -> Option<&Node> {
        self.nodes.get(name)
    }

    pub fn node_mut(&mut self, name: &str) -> Option<&mut Node> {
        self.nodes.get_mut(name)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self, from: &str, to: &str) -> usize {
        let mut count = 0 as usize;
        for succ in self.successors(from) {
            if succ.name == to {
                count += 1;
            }
        }
        count
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub name: String,
    pub predecessors: HashSet<String>,
    pub successors: HashSet<String>,
}
impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            predecessors: HashSet::new(),
            successors: HashSet::new(),
        }
    }
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub struct DiGraphMatcher<'a> {
    pub g1: &'a DiGraph,
    pub g2: &'a DiGraph,

    pub g1_nodes: HashSet<String>,
    pub g2_nodes: HashSet<String>,
    pub g2_node_order: HashMap<String, usize>,

    // Declare that we will be searching for a graph-graph isomorphism.
    // test='graph'
    // Indicates that the graph matcher is looking for a graph-graph
    // isomorphism.

    // test='subgraph'
    // Indicates that the graph matcher is looking for a subgraph-graph
    // isomorphism such that a subgraph of G1 is isomorphic to G2.

    // test='mono'
    // Indicates that the graph matcher is looking for a subgraph-graph
    // monomorphism such that a subgraph of G1 is monomorphic to G2.
    pub test: String,

    // core_1[n] contains the index of the node paired with n, which is m, provided n is in the mapping.
    // core_2[m] contains the index of the node paired with m, which is n, provided m is in the mapping.
    // core_1.len() == number of nodes in G1
    // pub core_1: HashMap<&'a Node, &'a Node>,
    pub core_1: HashMap<String, String>,
    // core_2.len() == n&'a str, &'a str in G2
    // pub core_2: HashMap<&'a Node, &'a Node>,
    pub core_2: HashMap<String, String>,

    // See the paper for definitions of G1(s), G2(s), M1, M2, Tin_1, Tin_2, Tout_1, and Tout_2
    //
    // in_1[n] is nonzero if n is either in M1(s) or Tin_1(s), in_1.len() == number of nodes in G1(s)
    // out_1[n] is nonzero if n is either in M1(s) or Tout_1(s), out_1.len() == number of nodes in G1(s)
    //
    // in_2[m] is nonzero if m is either in M2(s) or Tin_2(s), in_2.len() == number of nodes in G2(s)
    // out_2[m] is nonzero if m is either in M2(s) or Tout_2(s), out_2.len() == number of nodes in G2(s)
    //
    // The value stored is the depth of the search tree when the node became part of the corresponding set
    pub in_1: HashMap<String, usize>,
    pub in_2: HashMap<String, usize>,
    pub out_1: HashMap<String, usize>,
    pub out_2: HashMap<String, usize>,

    // pub state: DiGMState<'a>,

    // Provide a convenient way to access the isomorphism mapping.
    pub mapping: HashMap<String, String>,
}
impl<'a> DiGraphMatcher<'a> {
    pub fn new(g1: &'a DiGraph, g2: &'a DiGraph) -> Self {
        DiGraphMatcher {
            g1: g1,
            g2: g2,
            g1_nodes: g1.nodes.keys().map(|x| x.clone()).collect(),
            g2_nodes: g2.nodes.keys().map(|x| x.clone()).collect(),
            g2_node_order: g2
                .nodes
                .keys()
                .enumerate()
                .map(|(order, key)| (key.clone(), order))
                .collect::<Vec<(String, usize)>>()
                .into_iter()
                .collect::<HashMap<String, usize>>(),
            test: String::from("graph"),
            core_1: HashMap::new(),
            core_2: HashMap::new(),
            in_1: HashMap::new(),
            in_2: HashMap::new(),
            out_1: HashMap::new(),
            out_2: HashMap::new(),
            // state: DiGMState::new(),
            mapping: HashMap::new(),
        }
    }

    pub fn subgraph_isomorphisms_iter(&mut self, mapping: &mut Vec<Vec<(String, String)>>) {
        self.test = String::from("subgraph");
        let mut state = DiGMState::create(self, None, None);
        self.try_match(&mut state, mapping);
    }

    pub fn try_match(&mut self, state: &mut DiGMState, mapping: &mut Vec<Vec<(String, String)>>) {
        // ! debug
        println!("\n======== try_match begin ========");
        println!("core_1: {:?}", self.core_1);
        println!("core_2: {:?}", self.core_2);
        println!("mapping: {:?}", mapping);

        if self.core_1.len() == self.g2.node_count() {
            // Save the final mapping, otherwise garbage collection deletes it.
            let res: Vec<(String, String)> = self
                .core_1
                .iter()
                .map(|(g1_node_name, g2_node_name)| (g1_node_name.clone(), g2_node_name.clone()))
                .collect();
            mapping.push(res);

            // ! debug
            println!("\nmapping: {:?}", mapping);
        } else {
            let pairs = self.candidate_paris_iter();

            // ! debug
            println!("\ncandidate pair: {:?}\n", pairs);

            for (g1_node, g2_node) in pairs {
                // ! debug
                println!("\n====== iteration begin ======\n");
                println!("core_1: {:?}", self.core_1);
                println!("core_2: {:?}", self.core_2);
                println!("g1_node: {:?}", g1_node.as_str());
                println!("g2_node: {:?}", g2_node.as_str());

                if self.syntactic_feasibility(g1_node.clone(), g2_node.clone()) {
                    if self.semantic_feasibility(g1_node.clone(), g2_node.clone()) {
                        // state.initilize(self, g1_node.clone(), g2_node.clone());
                        let mut newstate = DiGMState::create(self, Some(g1_node.clone()), Some(g2_node.clone()));

                        // ! debug
                        println!("\nafter init state, core_1: {:?}", self.core_1);
                        println!("after init state, core_2: {:?}\n", self.core_2);

                        self.try_match(&mut newstate, mapping);

                        // ! debug
                        println!("\nafter try_match: {:?}\n", mapping);

                        // ! debug
                        println!("\nafter restore state, core_1: {:?}", self.core_1);
                        println!("before restore state, core_2: {:?}", self.core_2);
                        println!("before restore state, in_1: {:?}", self.in_1);
                        println!("before restore state, in_2: {:?}", self.in_2);
                        println!("before restore state, out_1: {:?}", self.out_1);
                        println!("before restore state, out_2: {:?}", self.out_2);
                        println!("before restore state, depth: {}\n", newstate.depth);

                        // state.restore(self);
                        newstate.restore(self);

                        // ! debug
                        println!("\nafter restore state, core_1: {:?}", self.core_1);
                        println!("after restore state, core_2: {:?}", self.core_2);
                        println!("after restore state, in_1: {:?}", self.in_1);
                        println!("after restore state, in_2: {:?}", self.in_2);
                        println!("after restore state, out_1: {:?}", self.out_1);
                        println!("after restore state, out_2: {:?}", self.out_2);
                        println!("after restore state, depth: {}\n", newstate.depth);
                        println!("after restore state done!");
                    }
                }

                println!("====== iteration end ======\n");
            }
        }

        // ! debug
        println!("\n======== try_match begin ========\n");
    }

    pub fn syntactic_feasibility(&self, g1_node_name: String, g2_node_name: String) -> bool {
        let g1_node = self.g1.nodes.get(g1_node_name.as_str()).unwrap();
        let g2_node = self.g2.nodes.get(g2_node_name.as_str()).unwrap();

        // R_self for checking self loops
        // The number of selfloops for G1_node must equal the number of
        // self-loops for G2_node. Without this check, we would fail on R_pred
        // at the next recursion level. This should prune the tree even further.
        if !self.r_self(&g1_node, &g2_node) {
            println!("*** r_self failed ***");
            return false;
        }

        // R_pred and R_succ for checking the consistency of the partial solution
        if !self.r_pred(g1_node, g2_node) {
            println!("*** r_pred failed ***");
            return false;
        }

        if !self.r_succ(g1_node, g2_node) {
            println!("*** r_succ failed ***");
            return false;
        }

        // R_in, R_out and R_new for pruning the search tree
        // R_in and R_out is 1-look-ahead, and R_new is 2-look-ahead
        if !self.r_in(g1_node, g2_node) {
            println!("*** r_in failed ***");
            return false;
        }

        if !self.r_out(g1_node, g2_node) {
            println!("*** r_out failed ***");
            return false;
        }

        if !self.r_new(g1_node, g2_node) {
            println!("*** r_new failed ***");
            return false;
        }

        true
    }

    pub fn semantic_feasibility(&self, _g1_node: String, _g2_node: String) -> bool {
        true
    }

    fn candidate_paris_iter(&self) -> Vec<(String, String)> {
        // All computations are done using the current state!

        let mut pairs = Vec::new();

        // First we compute the out-terminal sets.
        let mut tout_1 = Vec::new();
        for name in self.out_1.keys() {
            if !self.core_1.contains_key(name.as_str()) {
                tout_1.push(name.clone());
            }
        }
        let mut tout_2 = Vec::new();
        for name in self.out_2.keys() {
            if !self.core_2.contains_key(name.as_str()) {
                tout_2.push(name.clone());
            }
        }

        // If T1_out and T2_out are both nonempty.
        // P(s) = Tout_1 x {min Tout_2}
        if tout_1.len() > 0 && tout_2.len() > 0 {
            let mut name2 = String::new();
            let mut min_order = usize::MAX;
            for key in tout_2.iter() {
                let order = self.g2_node_order.get(key.as_str()).unwrap().clone();
                if order < min_order {
                    min_order = order;
                    name2 = key.clone();
                }
            }
            for name1 in tout_1.iter() {
                pairs.push((name1.clone(), name2.clone()));
            }
        } else {
            // If T1_out and T2_out were both empty....
            // We compute the in-terminal sets.

            let mut tin_1 = Vec::new();
            for name in self.in_1.keys() {
                if !self.core_1.contains_key(name.as_str()) {
                    tin_1.push(name.clone());
                }
            }
            let mut tin_2 = Vec::new();
            for name in self.in_2.keys() {
                if !self.core_2.contains_key(name.as_str()) {
                    tin_2.push(name.clone());
                }
            }

            // If T1_in and T2_in are both nonempty.
            // P(s) = T1_out x {min T2_out}
            if tin_1.len() > 0 && tin_2.len() > 0 {
                let mut name2 = String::new();
                let mut min_order = usize::MAX;
                for key in tin_2.iter() {
                    let order = self.g2_node_order.get(key.as_str()).unwrap().clone();
                    if order < min_order {
                        min_order = order;
                        name2 = key.clone();
                    }
                }
                for name1 in tin_1.iter() {
                    pairs.push((name1.clone(), name2.clone()));
                }
            } else {
                // If all terminal sets are empty...
                // P(s) = (N_1 - M_1) x {min (N_2 - M_2)}

                let m2 = self.core_2.keys().cloned().collect();
                let diff_set = self.g2_nodes.difference(&m2);

                let mut name2 = String::new();
                let mut min_order = usize::MAX;
                for key in diff_set {
                    let order = self.g2_node_order.get(key.as_str()).unwrap().clone();
                    if order < min_order {
                        min_order = order;
                        name2 = key.clone();
                    }
                }
                for name1 in self.g1_nodes.iter() {
                    if !self.core_1.contains_key(name1.as_str()) {
                        pairs.push((name1.clone(), name2.clone()));
                    }
                }
            }
        }
        pairs
    }

    /// R_self for checking self loops
    /// The number of selfloops for G1_node must equal the number of
    /// self-loops for G2_node. Without this check, we would fail on R_pred
    /// at the next recursion level. This should prune the tree even further.
    fn r_self(&self, g1_node: &Node, g2_node: &Node) -> bool {
        // ! debug
        println!("\n*** r_self ***");

        if self
            .g1
            .edge_count(g1_node.name.as_str(), g1_node.name.as_str())
            != self
                .g2
                .edge_count(g2_node.name.as_str(), g2_node.name.as_str())
        {
            return false;
        }

        // ! debug
        println!("*** r_self done ***\n");

        true
    }

    /// R_pred and R_succ for checking the consistency of the partial solution
    fn r_pred(&self, g1_node: &Node, g2_node: &Node) -> bool {
        // ! debug
        println!("*** r_pred ***");

        // For each predecessor n' of n in the partial mapping, the
        // corresponding node m' is a predecessor of m, and vice versa. Also,
        // the number of edges must be equal

        for predecessor in self.g1.predecessors(g1_node.name.as_str()) {
            if self.core_1.contains_key(predecessor.name.as_str()) {
                if self
                    .g2
                    .predecessors(g2_node.name.as_str())
                    .iter()
                    .all(|&x| x.name != *self.core_1.get(predecessor.name.as_str()).unwrap())
                {
                    return false;
                } else if self
                    .g1
                    .edge_count(predecessor.name.as_str(), g1_node.name.as_str())
                    != self.g2.edge_count(
                        self.core_1.get(predecessor.name.as_str()).unwrap(),
                        g2_node.name.as_str(),
                    )
                {
                    return false;
                }
            }
        }

        // ! debug
        println!("g1.predecessors checked!");

        for predecessor in self.g2.predecessors(g2_node.name.as_str()) {
            if self.core_2.contains_key(predecessor.name.as_str()) {
                if self
                    .g1
                    .predecessors(g1_node.name.as_str())
                    .iter()
                    .all(|&x| x.name != *self.core_2.get(predecessor.name.as_str()).unwrap())
                {
                    return false;
                } else if self
                    .g2
                    .edge_count(predecessor.name.as_str(), g2_node.name.as_str())
                    != self.g1.edge_count(
                        self.core_2.get(predecessor.name.as_str()).unwrap(),
                        g1_node.name.as_str(),
                    )
                {
                    return false;
                }
            }
        }

        // ! debug
        println!("g2.predecessors checked!");
        println!("*** r_pred done ***\n");

        true
    }

    /// R_pred and R_succ for checking the consistency of the partial solution
    fn r_succ(&self, g1_node: &Node, g2_node: &Node) -> bool {
        // ! debug
        println!("\n*** r_succ ***");

        // For each successor n' of n in the partial mapping, the corresponding
        // node m' is a successor of m, and vice versa. Also, the number of
        // edges must be equal.

        for successor in self.g1.successors(g1_node.name.as_str()) {
            if self.core_1.contains_key(successor.name.as_str()) {
                if self
                    .g2
                    .successors(g2_node.name.as_str())
                    .iter()
                    .all(|&x| x.name != *self.core_1.get(successor.name.as_str()).unwrap())
                {
                    return false;
                } else if self
                    .g1
                    .edge_count(g1_node.name.as_str(), successor.name.as_str())
                    != self.g2.edge_count(
                        g2_node.name.as_str(),
                        self.core_1.get(successor.name.as_str()).unwrap(),
                    )
                {
                    return false;
                }
            }
        }

        // ! debug
        println!("g1.sucessors checked!");

        for successor in self.g2.successors(g2_node.name.as_str()) {
            if self.core_2.contains_key(successor.name.as_str()) {
                if self
                    .g1
                    .successors(g1_node.name.as_str())
                    .iter()
                    .all(|&x| x.name != *self.core_2.get(successor.name.as_str()).unwrap())
                {
                    return false;
                } else if self
                    .g2
                    .edge_count(g2_node.name.as_str(), successor.name.as_str())
                    != self.g1.edge_count(
                        g1_node.name.as_str(),
                        self.core_2.get(successor.name.as_str()).unwrap(),
                    )
                {
                    return false;
                }
            }
        }

        // ! debug
        println!("g2.sucessors checked!");
        println!("*** r_succ done ***\n");

        true
    }

    /// R_in, R_out and R_new for pruning the search tree
    /// R_in and R_out is 1-look-ahead, and R_new is 2-look-ahead
    fn r_in(&self, g1_node: &Node, g2_node: &Node) -> bool {
        // ! debug
        println!("\n*** r_in ***");

        // The number of predecessors of n that are in Tin_1 is equal to the
        // number of predecessors of m that are in Tin_2.

        // Tin = in - core

        let mut num1 = 0;
        for predecessor in self.g1.predecessors(g1_node.name.as_str()) {
            if self.in_1.contains_key(predecessor.name.as_str())
                && !self.core_1.contains_key(predecessor.name.as_str())
            {
                num1 += 1;
            }
        }
        let mut num2 = 0;
        for predecessor in self.g2.predecessors(g2_node.name.as_str()) {
            if self.in_2.contains_key(predecessor.name.as_str())
                && !self.core_2.contains_key(predecessor.name.as_str())
            {
                num2 += 1;
            }
        }
        if self.test == "graph" {
            if !(num1 == num2) {
                return false;
            }
        } else {
            if !(num1 >= num2) {
                return false;
            }
        }

        // ! debug
        println!("predecessors same!");

        // The number of successors of n that are in Tin_1 is equal to the
        // number of successors of m that are in Tin_2.
        let mut num1 = 0;
        for successor in self.g1.successors(g1_node.name.as_str()) {
            if self.in_1.contains_key(successor.name.as_str())
                && !self.core_1.contains_key(successor.name.as_str())
            {
                num1 += 1;
            }
        }
        let mut num2 = 0;
        for successor in self.g2.successors(g2_node.name.as_str()) {
            if self.in_2.contains_key(successor.name.as_str())
                && !self.core_2.contains_key(successor.name.as_str())
            {
                num2 += 1;
            }
        }
        if self.test == "graph" {
            if !(num1 == num2) {
                return false;
            }
        } else {
            if !(num1 >= num2) {
                return false;
            }
        }

        // ! debug
        println!("successors same!");
        println!("*** r_in done ***\n");

        true
    }

    /// R_in, R_out and R_new for pruning the search tree
    /// R_in and R_out is 1-look-ahead, and R_new is 2-look-ahead
    fn r_out(&self, g1_node: &Node, g2_node: &Node) -> bool {
        // ! debug
        println!("\n*** r_out ***");

        // The number of predecessors of n that are in Tout_1 is equal to the
        // number of predecessors of m that are in Tout_2.

        // Tout = out - core

        let mut num1 = 0;
        for predecessor in self.g1.predecessors(g1_node.name.as_str()) {
            if self.out_1.contains_key(predecessor.name.as_str())
                && !self.core_1.contains_key(predecessor.name.as_str())
            {
                num1 += 1;
            }
        }
        let mut num2 = 0;
        for predecessor in self.g2.predecessors(g2_node.name.as_str()) {
            if self.out_2.contains_key(predecessor.name.as_str())
                && !self.core_2.contains_key(predecessor.name.as_str())
            {
                num2 += 1;
            }
        }
        if self.test == "graph" {
            if !(num1 == num2) {
                return false;
            }
        } else {
            if !(num1 >= num2) {
                return false;
            }
        }

        // ! debug
        println!("predecessors same!");

        // The number of successors of n that are in Tout_1 is equal to the
        // number of successors of m that are in Tout_2.

        let mut num1 = 0;
        for successor in self.g1.successors(g1_node.name.as_str()) {
            if self.out_1.contains_key(successor.name.as_str())
                && !self.core_1.contains_key(successor.name.as_str())
            {
                num1 += 1;
            }
        }
        let mut num2 = 0;
        for successor in self.g2.successors(g2_node.name.as_str()) {
            if self.out_2.contains_key(successor.name.as_str())
                && !self.core_2.contains_key(successor.name.as_str())
            {
                num2 += 1;
            }
        }
        if self.test == "graph" {
            if !(num1 == num2) {
                return false;
            }
        } else {
            if !(num1 >= num2) {
                return false;
            }
        }

        // ! debug
        println!("sucessors same!");
        println!("*** r_out done ***\n");

        true
    }

    /// R_in, R_out and R_new for pruning the search tree
    /// R_in and R_out is 1-look-ahead, and R_new is 2-look-ahead
    fn r_new(&self, g1_node: &Node, g2_node: &Node) -> bool {
        // ! debug
        println!("\n*** r_new ***");
        println!("in_1: {:?}", self.in_1);
        println!("out_1: {:?}", self.out_1);
        println!("in_2: {:?}", self.in_2);
        println!("out_2: {:?}", self.out_2);

        // The number of predecessors of n that are neither in the core_1 nor
        // Tin_1 nor Tout_1 is equal to the number of predecessors of m
        // that are neither in core_2 nor Tin_2 nor Tout_2.

        let mut num1 = 0;
        for predecessor in self.g1.predecessors(g1_node.name.as_str()) {
            if !self.in_1.contains_key(predecessor.name.as_str())
                && !self.out_1.contains_key(predecessor.name.as_str())
            {
                num1 += 1;
            }
        }
        let mut num2 = 0;
        for predecessor in self.g2.predecessors(g2_node.name.as_str()) {
            if !self.in_2.contains_key(predecessor.name.as_str())
                && !self.out_2.contains_key(predecessor.name.as_str())
            {
                num2 += 1;
            }
        }
        if self.test == "graph" {
            if !(num1 == num2) {
                return false;
            }
        } else {
            if !(num1 >= num2) {
                // ! debug
                println!("num1:{}, num2:{}", num1, num2);
                return false;
            }
        }

        // ! debug
        println!("predecessors same!");

        // The number of successors of n that are neither in the core_1 nor
        // Tin_1 nor Tout_1 is equal to the number of successors of m
        // that are neither in core_2 nor Tin_2 nor Tout_2.

        let mut num1 = 0;
        for successor in self.g1.successors(g1_node.name.as_str()) {
            if !self.in_1.contains_key(successor.name.as_str())
                && !self.out_1.contains_key(successor.name.as_str())
            {
                num1 += 1;
            }
        }
        let mut num2 = 0;
        for successor in self.g2.successors(g2_node.name.as_str()) {
            if !self.in_2.contains_key(successor.name.as_str())
                && !self.out_2.contains_key(successor.name.as_str())
            {
                num2 += 1;
            }
        }
        if self.test == "graph" {
            if !(num1 == num2) {
                return false;
            }
        } else {
            if !(num1 >= num2) {
                // ! debug
                println!("num1:{}, num2:{}", num1, num2);
                return false;
            }
        }

        // ! debug
        println!("successors same!");
        println!("*** r_new done ***\n");

        true
    }
}

pub struct DiGMState {
    pub g1_node: Option<String>,
    pub g2_node: Option<String>,
    pub depth: usize,
}
impl DiGMState {
    pub fn new(matcher: &DiGraphMatcher) -> Self {
        DiGMState {
            g1_node: None,
            g2_node: None,
            depth: matcher.core_1.len(),
        }
    }

    pub fn initilize(&mut self, matcher: &mut DiGraphMatcher, g1_node: String, g2_node: String) {
        self.g1_node = Some(g1_node.clone());
        self.g2_node = Some(g2_node.clone());
        self.depth = matcher.core_1.len();

        // update matcher
        matcher.core_1.insert(g1_node.clone(), g2_node.clone());
        matcher.core_2.insert(g2_node.clone(), g1_node.clone());

        // First we add the new nodes to Tin_1, Tin_2, Tout_1 and Tout_2
        matcher.in_1.entry(g1_node.clone()).or_insert(self.depth);
        matcher.out_1.entry(g1_node.clone()).or_insert(self.depth);
        matcher.in_2.entry(g2_node.clone()).or_insert(self.depth);
        matcher.out_2.entry(g2_node.clone()).or_insert(self.depth);

        // Now we add every other node...

        // Updates for Tin_1
        let mut new_nodes = HashSet::new();
        for name in matcher.core_1.keys() {
            for predecessor in matcher.g1.predecessors(name) {
                if !matcher.core_1.contains_key(predecessor.name.as_str()) {
                    new_nodes.insert(predecessor);
                }
            }
        }
        for node in new_nodes {
            matcher.in_1.entry(node.name.clone()).or_insert(self.depth);
        }

        // Updates for Tin_2
        let mut new_nodes = HashSet::new();
        for name in matcher.core_2.keys() {
            for predecessor in matcher.g2.predecessors(name) {
                if !matcher.core_2.contains_key(predecessor.name.as_str()) {
                    new_nodes.insert(predecessor);
                }
            }
        }
        for node in new_nodes {
            matcher.in_2.entry(node.name.clone()).or_insert(self.depth);
        }

        // Updates for Tout_1
        let mut new_nodes = HashSet::new();
        for name in matcher.core_1.keys() {
            for successor in matcher.g1.successors(name) {
                if !matcher.core_1.contains_key(successor.name.as_str()) {
                    new_nodes.insert(successor);
                }
            }
        }
        for node in new_nodes {
            matcher.out_1.entry(node.name.clone()).or_insert(self.depth);
        }

        // Updates for Tout_2
        let mut new_nodes = HashSet::new();
        for name in matcher.core_2.keys() {
            for successor in matcher.g2.successors(name) {
                if !matcher.core_2.contains_key(successor.name.as_str()) {
                    new_nodes.insert(successor);
                }
            }
        }
        for node in new_nodes {
            matcher.out_2.entry(node.name.clone()).or_insert(self.depth);
        }
    }

    pub fn create(
        matcher: &mut DiGraphMatcher,
        g1_node: Option<String>,
        g2_node: Option<String>,
    ) -> DiGMState {
        if g1_node.is_none() || g2_node.is_none() {
            // Then we reset the class variables
            matcher.core_1.clear();
            matcher.core_2.clear();
            matcher.in_1.clear();
            matcher.in_2.clear();
            matcher.out_1.clear();
            matcher.out_2.clear();
        }

        let depth = matcher.core_1.len();

        if g1_node.is_some() && g2_node.is_some() {
            let g1_name = g1_node.clone().unwrap();
            let g2_name = g2_node.clone().unwrap();

            // update matcher
            matcher.core_1.insert(g1_name.clone(), g2_name.clone());
            matcher.core_2.insert(g2_name.clone(), g1_name.clone());

            // First we add the new nodes to Tin_1, Tin_2, Tout_1 and Tout_2
            matcher.in_1.entry(g1_name.clone()).or_insert(depth);
            matcher.out_1.entry(g1_name.clone()).or_insert(depth);
            matcher.in_2.entry(g2_name.clone()).or_insert(depth);
            matcher.out_2.entry(g2_name.clone()).or_insert(depth);

            // Now we add every other node...

            // Updates for Tin_1
            let mut new_nodes = HashSet::new();
            for name in matcher.core_1.keys() {
                for predecessor in matcher.g1.predecessors(name) {
                    if !matcher.core_1.contains_key(predecessor.name.as_str()) {
                        new_nodes.insert(predecessor);
                    }
                }
            }
            for node in new_nodes {
                matcher.in_1.entry(node.name.clone()).or_insert(depth);
            }

            // Updates for Tin_2
            let mut new_nodes = HashSet::new();
            for name in matcher.core_2.keys() {
                for predecessor in matcher.g2.predecessors(name) {
                    if !matcher.core_2.contains_key(predecessor.name.as_str()) {
                        new_nodes.insert(predecessor);
                    }
                }
            }
            for node in new_nodes {
                matcher.in_2.entry(node.name.clone()).or_insert(depth);
            }

            // Updates for Tout_1
            let mut new_nodes = HashSet::new();
            for name in matcher.core_1.keys() {
                for successor in matcher.g1.successors(name) {
                    if !matcher.core_1.contains_key(successor.name.as_str()) {
                        new_nodes.insert(successor);
                    }
                }
            }
            for node in new_nodes {
                matcher.out_1.entry(node.name.clone()).or_insert(depth);
            }

            // Updates for Tout_2
            let mut new_nodes = HashSet::new();
            for name in matcher.core_2.keys() {
                for successor in matcher.g2.successors(name) {
                    if !matcher.core_2.contains_key(successor.name.as_str()) {
                        new_nodes.insert(successor);
                    }
                }
            }
            for node in new_nodes {
                matcher.out_2.entry(node.name.clone()).or_insert(depth);
            }
        }

        if g1_node.is_some() && g2_node.is_some() {
            DiGMState {
                g1_node: g1_node.clone(),
                g2_node: g2_node.clone(),
                depth: depth,
            }
        } else {
            DiGMState {
                g1_node: None,
                g2_node: None,
                depth: depth,
            }
        }
    }

    pub fn restore(&self, matcher: &mut DiGraphMatcher) {
        // First we remove the node that was added from the core vectors.
        // Watch out! G1_node == 0 should evaluate to True.
        if self.g1_node.is_some() && self.g2_node.is_some() {
            matcher
                .core_1
                .remove_entry(self.g1_node.as_ref().unwrap().as_str());
            matcher
                .core_2
                .remove_entry(self.g2_node.as_ref().unwrap().as_str());
        }

        // Now we revert the other four vectors.
        // Thus, we delete all entries which have this depth level.

        let keys: Vec<String> = matcher
            .in_1
            .iter()
            .filter(|&(_, depth)| *depth == self.depth)
            .map(|(name, _)| name.clone())
            .collect();
        for key in keys {
            matcher.in_1.remove(key.as_str());
        }

        let keys: Vec<String> = matcher
            .in_2
            .iter()
            .filter(|&(_, depth)| *depth == self.depth)
            .map(|(name, _)| name.clone())
            .collect();
        for key in keys {
            matcher.in_2.remove(key.as_str());
        }

        let keys: Vec<String> = matcher
            .out_1
            .iter()
            .filter(|&(_, depth)| *depth == self.depth)
            .map(|(name, _)| name.clone())
            .collect();
        for key in keys {
            matcher.out_1.remove(key.as_str());
        }

        let keys: Vec<String> = matcher
            .out_2
            .iter()
            .filter(|&(_, depth)| *depth == self.depth)
            .map(|(name, _)| name.clone())
            .collect();
        for key in keys {
            matcher.out_2.remove(key.as_str());
        }
    }
}

fn main() {
    let mut g1 = DiGraph::new();
    g1.add_edge(Some("A"), Some("B"));
    g1.add_edge(Some("B"), Some("C"));
    // println!("g1: {:?}", g1);
    // assert!(g1.node_count() == 3);
    // println!(
    //     "\nA pred:{:?}, succ:{:?}",
    //     g1.predecessors("A"),
    //     g1.successors("A")
    // );
    // println!(
    //     "\nB pred:{:?}, succ:{:?}",
    //     g1.predecessors("B"),
    //     g1.successors("B")
    // );
    // println!(
    //     "\nC pred:{:?}, succ:{:?}",
    //     g1.predecessors("C"),
    //     g1.successors("C")
    // );

    let mut g2 = DiGraph::new();
    g2.add_edge(Some("Y"), Some("Z"));
    // assert!(g2.node_count() == 2);
    // println!("g2: {:?}", g2);

    let mut matcher = DiGraphMatcher::new(&g1, &g2);
    let mut mapping: Vec<Vec<(String, String)>> = Vec::new();
    matcher.subgraph_isomorphisms_iter(&mut mapping);

    println!("\nnum of matches: {}", mapping.len());
    println!("mapping: {:?}", mapping);
}
