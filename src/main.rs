use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct DiGraph {
    pub nodes: HashMap<String, Node>,
}
impl DiGraph {
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.clone(), node);
    }

    pub fn add_edge(&mut self, from: &mut Node, to: &mut Node) {
        from.successors.insert(to.name.clone());
        to.predecessors.insert(from.name.clone());
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
            .nodes
            .get(name)
            .expect(format!("Not found node with name: {}", name).as_str());
        node.successors
            .iter()
            .map(|name| self.nodes.get(name.as_str()).unwrap())
            .collect()
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

pub struct DiGraphMatcher {
    pub G1: &'static DiGraph,
    pub G2: &'static DiGraph,

    pub G1_nodes: HashSet<String>,
    pub G2_nodes: HashSet<String>,
    pub G2_node_order: HashMap<String, usize>,

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

    pub depth: usize,
}
impl DiGraphMatcher {
    pub fn new(G1: &'static DiGraph, G2: &'static DiGraph) -> Self {
        DiGraphMatcher {
            G1: G1,
            G2: G2,
            G1_nodes: G1.nodes.keys().map(|x| x.clone()).collect(),
            G2_nodes: G2.nodes.keys().map(|x| x.clone()).collect(),
            G2_node_order: G2
                .nodes
                .keys()
                .enumerate()
                .map(|(order, key)| (key.clone(), order))
                .collect::<Vec<(String, usize)>>()
                .into_iter()
                .collect::<HashMap<String, usize>>(),
            core_1: HashMap::new(),
            core_2: HashMap::new(),
            in_1: HashMap::new(),
            in_2: HashMap::new(),
            out_1: HashMap::new(),
            out_2: HashMap::new(),
            // state: DiGMState::new(),
            mapping: HashMap::new(),
            depth: 0,
        }
    }

    pub fn push_state(&mut self, state: &mut DiGMState, g1_node: String, g2_node: String) {
        // state.push(g1_node, g2_node);

        self.core_1.insert(g1_node.clone(), g2_node.clone());
        self.core_2.insert(g2_node.clone(), g1_node.clone());

        self.depth = self.core_1.len();

        // First we add the new nodes to Tin_1, Tin_2, Tout_1 and Tout_2
        if !self.in_1.contains_key(g1_node.as_str()) {
            self.in_1.insert(g1_node.clone(), self.depth);
        }
        if !self.out_1.contains_key(g1_node.as_str()) {
            self.out_1.insert(g1_node.clone(), self.depth);
        }
        if !self.in_2.contains_key(g2_node.as_str()) {
            self.in_2.insert(g2_node.clone(), self.depth);
        }
        if !self.out_2.contains_key(g2_node.as_str()) {
            self.out_2.insert(g2_node.clone(), self.depth);
        }

        // Now we add every other node...

        // Updates for Tin_1
        let mut new_nodes = HashSet::new();
        for name in self.core_1.keys() {
            for predecessor in self.G1.predecessors(name) {
                if !self.core_1.contains_key(predecessor.name.as_str()) {
                    new_nodes.insert(predecessor);
                }
            }
        }
        for node in new_nodes {
            if !self.in_1.contains_key(node.name.as_str()) {
                self.in_1.insert(node.name.clone(), self.depth);
            }
        }

        // Updates for Tin_2
        let mut new_nodes = HashSet::new();
        for name in self.core_2.keys() {
            for predecessor in self.G2.predecessors(name) {
                if !self.core_2.contains_key(predecessor.name.as_str()) {
                    new_nodes.insert(predecessor);
                }
            }
        }
        for node in new_nodes {
            if !self.in_2.contains_key(node.name.as_str()) {
                self.in_2.insert(node.name.clone(), self.depth);
            }
        }

        // Updates for Tout_1
        let mut new_nodes = HashSet::new();
        for name in self.core_1.keys() {
            for successor in self.G1.successors(name) {
                if !self.core_1.contains_key(successor.name.as_str()) {
                    new_nodes.insert(successor);
                }
            }
        }
        for node in new_nodes {
            if !self.out_1.contains_key(node.name.as_str()) {
                self.out_1.insert(node.name.clone(), self.depth);
            }
        }

        // Updates for Tout_2
        let mut new_nodes = HashSet::new();
        for name in self.core_2.keys() {
            for successor in self.G2.successors(name) {
                if !self.core_2.contains_key(successor.name.as_str()) {
                    new_nodes.insert(successor);
                }
            }
        }
        for node in new_nodes {
            if !self.out_2.contains_key(node.name.as_str()) {
                self.out_2.insert(node.name.clone(), self.depth);
            }
        }
    }

    pub fn pop_state(&mut self, g1_node: String, g2_node: String) {
        // First we remove the node that was added from the core vectors.
        // Watch out! G1_node == 0 should evaluate to True.
        self.core_1.remove_entry(g1_node.as_str());
        self.core_2.remove_entry(g2_node.as_str());

        // Now we revert the other four vectors.
        // Thus, we delete all entries which have this depth level.

        let keys: Vec<_> = self
            .in_1
            .iter()
            .filter(|&(_, depth)| *depth == self.depth)
            .map(|(name, _)| name.clone())
            .collect();
        for key in keys {
            self.in_1.remove(key.as_str());
        }

        let keys: Vec<_> = self
            .in_2
            .iter()
            .filter(|&(_, depth)| *depth == self.depth)
            .map(|(name, _)| name.clone())
            .collect();
        for key in keys {
            self.in_2.remove(key.as_str());
        }

        let keys: Vec<_> = self
            .out_1
            .iter()
            .filter(|&(_, depth)| *depth == self.depth)
            .map(|(name, _)| name.clone())
            .collect();
        for key in keys {
            self.out_1.remove(key.as_str());
        }

        let keys: Vec<_> = self
            .out_2
            .iter()
            .filter(|&(_, depth)| *depth == self.depth)
            .map(|(name, _)| name.clone())
            .collect();
        for key in keys {
            self.out_2.remove(key.as_str());
        }
    }

    // pub fn candidate_paris_iter(&self) -> PairIterator {
    //     PairIterator::new(self)
    // }
}

pub struct DiGMState {
    pub G1_node: Option<String>,
    pub G2_node: Option<String>,

    pub G1_nodes: HashSet<String>,
    pub G2_nodes: HashSet<String>,
    pub G2_node_order: HashMap<String, usize>,

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

    pub depth: usize,
}
impl DiGMState {
    pub fn new(G1: &DiGraph, G2: &DiGraph) -> Self {
        DiGMState {
            G1_node: None,
            G2_node: None,

            G1_nodes: G1.nodes.keys().map(|x| x.clone()).collect(),
            G2_nodes: G2.nodes.keys().map(|x| x.clone()).collect(),
            G2_node_order: G2
                .nodes
                .keys()
                .enumerate()
                .map(|(order, key)| (key.clone(), order))
                .collect::<Vec<(String, usize)>>()
                .into_iter()
                .collect::<HashMap<String, usize>>(),
            core_1: HashMap::new(),
            core_2: HashMap::new(),
            in_1: HashMap::new(),
            in_2: HashMap::new(),
            out_1: HashMap::new(),
            out_2: HashMap::new(),
            // state: DiGMState::new(),
            mapping: HashMap::new(),
            depth: 0,
        }
    }

    pub fn push(&mut self, g1_node: String, g2_node: String) {
        // Store the node that was added last.
        self.G1_node = Some(g1_node);
        self.G2_node = Some(g2_node);
    }

    pub fn pop(&mut self, g1_node: String, g2_node: String) {
        // remove the node that was added from the core vectors.
        todo!("pop")
    }

    pub fn candidate_paris_iter(&self) -> PairIterator {
        PairIterator::new(self)
    }
}

pub fn subgraph_isomorphisms_iter(g1: &'static DiGraph, g2: &'static DiGraph) {
    let mut state = DiGMState::new(g1, g2);
    let mut matcher = DiGraphMatcher::new(g1, g2);
    try_match(&mut matcher, &mut state);
}

pub fn try_match(matcher: &DiGraphMatcher, mut state: &mut DiGMState) {
    if matcher.core_1.len() == matcher.G2.node_count() {
        todo!("save the final mapping.");
    } else {
        // let mut c_pairs_iter = matcher.candidate_paris_iter();
        let mut c_pairs_iter = state.candidate_paris_iter();
        while let Some((G1_node, G2_node)) = c_pairs_iter.next() {
            if syntactic_feasibility(&matcher, G1_node.clone(), G2_node.clone()) {
                if semantic_feasibility(G1_node.clone(), G2_node.clone()) {
                    push_state(&mut state, G1_node.clone(), G2_node.clone());
                    try_match(&matcher, &mut state);
                    pop_state(&mut state);
                }
            }
        }
        // for (G1_node, G2_node) in candidate_paris_iter.next() {
        //     if syntactic_feasibility(matcher, G1_node, G2_node) {
        //         if semantic_feasibility(G1_node, G2_node) {
        //             matcher.push_state(state, G1_node, G2_node);
        //             try_match(matcher, state);
        //             matcher.pop_state(state, G1_node, G2_node);
        //         }
        //     }
        // }
    }
}

pub fn push_state<'a>(state: &mut DiGMState, g1_node_name: String, g2_node_name: String) {
    todo!()
}

pub fn pop_state<'a>(state: &mut DiGMState) {
    todo!()
}

pub struct PairIterator {
    pairs: Vec<(String, String)>,
}
impl PairIterator {
    fn new(matcher: &DiGMState) -> Self {
        // All computations are done using the current state!

        let mut pairs = Vec::new();

        // G1_nodes = self.G1_nodes
        // G2_nodes = self.G2_nodes
        // min_key = self.G2_node_order.__getitem__

        // First we compute the out-terminal sets.
        let mut Tout_1 = Vec::new();
        for name in matcher.out_1.keys() {
            if !matcher.core_1.contains_key(name.as_str()) {
                Tout_1.push(name.clone());
            }
        }
        let mut Tout_2 = Vec::new();
        for name in matcher.out_2.keys() {
            if !matcher.core_2.contains_key(name.as_str()) {
                Tout_2.push(name.clone());
            }
        }

        // If T1_out and T2_out are both nonempty.
        // P(s) = Tout_1 x {min Tout_2}
        if Tout_1.len() > 0 && Tout_2.len() > 0 {
            for name1 in Tout_1.iter() {
                for name2 in Tout_2.iter() {
                    pairs.push((name1.clone(), name2.clone()));
                }
            }
        } else {
            // If T1_out and T2_out were both empty....
            // We compute the in-terminal sets.

            let mut Tin_1 = Vec::new();
            for name in matcher.in_1.keys() {
                if !matcher.core_1.contains_key(name.as_str()) {
                    Tin_1.push(name.clone());
                }
            }
            let mut Tin_2 = Vec::new();
            for name in matcher.in_2.keys() {
                if !matcher.core_2.contains_key(name.as_str()) {
                    Tin_2.push(name.clone());
                }
            }

            // If T1_in and T2_in are both nonempty.
            // P(s) = T1_out x {min T2_out}
            if Tin_1.len() > 0 && Tin_2.len() > 0 {
                for node1 in Tin_1.iter() {
                    for node2 in Tin_2.iter() {
                        pairs.push((node1.clone(), node2.clone()));
                    }
                }
            } else {
                // If all terminal sets are empty...
                // P(s) = (N_1 - M_1) x {min (N_2 - M_2)}

                let m2: HashSet<_> = matcher.core_2.keys().map(|name| name.clone()).collect();
                for name1 in matcher.G1_nodes.iter() {
                    for name2 in matcher.G2_nodes.difference(&m2) {
                        pairs.push((name1.to_string().clone(), name2.to_string().clone()));
                    }
                }
            }
        }
        // pairs

        PairIterator {
            pairs: pairs
                .iter()
                .map(|(g1_node_name, g2_node_name)| (g1_node_name.clone(), g2_node_name.clone()))
                .collect(),
        }
    }

    pub fn next(&mut self) -> Option<(String, String)> {
        while let Some(pair) = self.pairs.pop() {
            return Some(pair);
        }
        None
    }
}

// /// Iterator over candidate pairs of nodes in G1 and G2.
// pub fn candidate_paris_iter<'a>(matcher: &'a DiGraphMatcher<'a>) -> Vec<(String, String)> {
//     // All computations are done using the current state!

//     let mut pairs = Vec::new();

//     // G1_nodes = self.G1_nodes
//     // G2_nodes = self.G2_nodes
//     // min_key = self.G2_node_order.__getitem__

//     // First we compute the out-terminal sets.
//     let mut Tout_1 = Vec::new();
//     for &name in matcher.out_1.keys() {
//         if !matcher.core_1.contains_key(name.as_str()) {
//             Tout_1.push(name.clone());
//         }
//     }
//     let mut Tout_2 = Vec::new();
//     for &name in matcher.out_2.keys() {
//         if !matcher.core_2.contains_key(name.as_str()) {
//             Tout_2.push(name.clone());
//         }
//     }

//     // If T1_out and T2_out are both nonempty.
//     // P(s) = Tout_1 x {min Tout_2}
//     if Tout_1.len() > 0 && Tout_2.len() > 0 {
//         for node1 in Tout_1 {
//             for &node2 in Tout_2.iter() {
//                 pairs.push((node1, node2));
//             }
//         }
//     } else {
//         // If T1_out and T2_out were both empty....
//         // We compute the in-terminal sets.

//         let mut Tin_1 = Vec::new();
//         for &name in matcher.in_1.keys() {
//             if !matcher.core_1.contains_key(name.as_str()) {
//                 Tin_1.push(name.clone());
//             }
//         }
//         let mut Tin_2 = Vec::new();
//         for &name in matcher.in_2.keys() {
//             if !matcher.core_2.contains_key(name.as_str()) {
//                 Tin_2.push(name.clone());
//             }
//         }

//         // If T1_in and T2_in are both nonempty.
//         // P(s) = T1_out x {min T2_out}
//         if Tin_1.len() > 0 && Tin_2.len() > 0 {
//             for name1 in Tin_1 {
//                 for &name2 in Tin_2.iter() {
//                     pairs.push((name1.clone(), name2.clone()));
//                 }
//             }
//         } else {
//             // If all terminal sets are empty...
//             // P(s) = (N_1 - M_1) x {min (N_2 - M_2)}

//             let m2 = matcher
//                 .core_2
//                 .keys()
//                 .map(|&name| name.as_str())
//                 .collect::<HashSet<&str>>();

//             for &name1 in matcher.G1_nodes.iter() {
//                 for &name2 in matcher.G2_nodes.difference(&m2) {
//                     pairs.push((name1.to_string().clone(), name2.to_string().clone()));
//                 }
//             }
//         }
//     }
//     pairs
// }

pub fn syntactic_feasibility(
    matcher: &DiGraphMatcher,
    g1_node_name: String,
    g2_node_name: String,
) -> bool {
    let g1_node = matcher.G1.nodes.get(g1_node_name.as_str()).unwrap();
    let g2_node = matcher.G2.nodes.get(g2_node_name.as_str()).unwrap();

    // R_self for checking self loops
    // The number of selfloops for G1_node must equal the number of
    // self-loops for G2_node. Without this check, we would fail on R_pred
    // at the next recursion level. This should prune the tree even further.
    if !R_self(matcher, g1_node, g2_node) {
        return false;
    }

    // R_pred and R_succ for checking the consistency of the partial solution
    if !R_pred(matcher, g1_node, g2_node) || !R_succ(matcher, g1_node, g2_node) {
        return false;
    }

    // R_in, R_out and R_new for pruning the search tree
    // R_in and R_out is 1-look-ahead, and R_new is 2-look-ahead
    if !R_in(matcher, g1_node, g2_node)
        || R_out(matcher, g1_node, g2_node)
        || R_new(matcher, g1_node, g2_node)
    {
        return false;
    }

    true
}

/// R_self for checking self loops
/// The number of selfloops for G1_node must equal the number of
/// self-loops for G2_node. Without this check, we would fail on R_pred
/// at the next recursion level. This should prune the tree even further.
pub fn R_self<'a>(matcher: &DiGraphMatcher, g1_node: &'a Node, g2_node: &'a Node) -> bool {
    if matcher
        .G1
        .edge_count(g1_node.name.as_str(), g1_node.name.as_str())
        != matcher
            .G2
            .edge_count(g2_node.name.as_str(), g2_node.name.as_str())
    {
        return false;
    }
    true
}

/// R_pred and R_succ for checking the consistency of the partial solution
pub fn R_pred<'a>(matcher: &DiGraphMatcher, g1_node: &'a Node, g2_node: &'a Node) -> bool {
    // For each predecessor n' of n in the partial mapping, the
    // corresponding node m' is a predecessor of m, and vice versa. Also,
    // the number of edges must be equal

    for predecessor in matcher.G1.predecessors(g1_node.name.as_str()) {
        if matcher.core_1.contains_key(predecessor.name.as_str()) {
            if matcher
                .G2
                .predecessors(g2_node.name.as_str())
                .iter()
                .all(|&x| x.name != *matcher.core_1.get(predecessor.name.as_str()).unwrap())
            {
                return false;
            }
        } else if matcher
            .G1
            .edge_count(predecessor.name.as_str(), g1_node.name.as_str())
            != matcher.G2.edge_count(
                matcher.core_1.get(predecessor.name.as_str()).unwrap(),
                g2_node.name.as_str(),
            )
        {
            return false;
        }
    }

    for predecessor in matcher.G2.predecessors(g2_node.name.as_str()) {
        if matcher.core_2.contains_key(predecessor.name.as_str()) {
            if matcher
                .G1
                .predecessors(g1_node.name.as_str())
                .iter()
                .all(|&x| x.name != *matcher.core_2.get(predecessor.name.as_str()).unwrap())
            {
                return false;
            } else if matcher
                .G2
                .edge_count(predecessor.name.as_str(), g2_node.name.as_str())
                != matcher.G2.edge_count(
                    matcher.core_2.get(predecessor.name.as_str()).unwrap(),
                    g1_node.name.as_str(),
                )
            {
                return false;
            }
        }
    }

    true
}

/// R_pred and R_succ for checking the consistency of the partial solution
pub fn R_succ<'a>(matcher: &DiGraphMatcher, g1_node: &'a Node, g2_node: &'a Node) -> bool {
    // For each successor n' of n in the partial mapping, the corresponding
    // node m' is a successor of m, and vice versa. Also, the number of
    // edges must be equal.

    for successor in matcher.G1.successors(g1_node.name.as_str()) {
        if matcher.core_1.contains_key(successor.name.as_str()) {
            if matcher
                .G2
                .successors(g2_node.name.as_str())
                .iter()
                .all(|&x| x.name != *matcher.core_1.get(successor.name.as_str()).unwrap())
            {
                return false;
            } else if matcher
                .G1
                .edge_count(g1_node.name.as_str(), successor.name.as_str())
                != matcher.G2.edge_count(
                    g2_node.name.as_str(),
                    matcher.core_1.get(successor.name.as_str()).unwrap(),
                )
            {
                return false;
            }
        }
    }

    for successor in matcher.G2.successors(g2_node.name.as_str()) {
        if matcher.core_2.contains_key(successor.name.as_str()) {
            if matcher
                .G1
                .successors(g1_node.name.as_str())
                .iter()
                .all(|&x| x.name != *matcher.core_2.get(successor.name.as_str()).unwrap())
            {
                return false;
            } else if matcher
                .G2
                .edge_count(g2_node.name.as_str(), successor.name.as_str())
                != matcher.G1.edge_count(
                    g1_node.name.as_str(),
                    matcher.core_2.get(successor.name.as_str()).unwrap(),
                )
            {
                return false;
            }
        }
    }

    true
}

/// R_in, R_out and R_new for pruning the search tree
/// R_in and R_out is 1-look-ahead, and R_new is 2-look-ahead
pub fn R_in<'a>(matcher: &DiGraphMatcher, g1_node: &'a Node, g2_node: &'a Node) -> bool {
    // The number of predecessors of n that are in Tin_1 is equal to the
    // number of predecessors of m that are in Tin_2.

    let mut num1 = 0;
    for predecessor in matcher.G1.predecessors(g1_node.name.as_str()) {
        if matcher.in_1.contains_key(predecessor.name.as_str())
            && !matcher.core_1.contains_key(predecessor.name.as_str())
        {
            num1 += 1;
        }
    }
    let mut num2 = 0;
    for predecessor in matcher.G2.predecessors(g2_node.name.as_str()) {
        if matcher.in_2.contains_key(predecessor.name.as_str())
            && !matcher.core_2.contains_key(predecessor.name.as_str())
        {
            num2 += 1;
        }
    }
    if !(num1 >= num2) {
        return false;
    }

    // The number of successors of n that are in Tin_1 is equal to the
    // number of successors of m that are in Tin_2.
    let mut num1 = 0;
    for successor in matcher.G1.successors(g1_node.name.as_str()) {
        if matcher.in_1.contains_key(successor.name.as_str())
            && !matcher.core_1.contains_key(successor.name.as_str())
        {
            num1 += 1;
        }
    }
    let mut num2 = 0;
    for successor in matcher.G2.successors(g2_node.name.as_str()) {
        if matcher.in_2.contains_key(successor.name.as_str())
            && !matcher.core_2.contains_key(successor.name.as_str())
        {
            num2 += 1;
        }
    }
    if !(num1 >= num2) {
        return false;
    }

    true
}

/// R_in, R_out and R_new for pruning the search tree
/// R_in and R_out is 1-look-ahead, and R_new is 2-look-ahead
pub fn R_out<'a>(matcher: &DiGraphMatcher, g1_node: &'a Node, g2_node: &'a Node) -> bool {
    // The number of predecessors of n that are in Tout_1 is equal to the
    // number of predecessors of m that are in Tout_2.

    let mut num1 = 0;
    for predecessor in matcher.G1.predecessors(g1_node.name.as_str()) {
        if matcher.out_1.contains_key(predecessor.name.as_str())
            && !matcher.core_1.contains_key(predecessor.name.as_str())
        {
            num1 += 1;
        }
    }
    let mut num2 = 0;
    for predecessor in matcher.G2.predecessors(g2_node.name.as_str()) {
        if matcher.out_2.contains_key(predecessor.name.as_str())
            && !matcher.core_2.contains_key(predecessor.name.as_str())
        {
            num2 += 1;
        }
    }
    if !(num1 >= num2) {
        return false;
    }

    // The number of successors of n that are in Tout_1 is equal to the
    // number of successors of m that are in Tout_2.

    let mut num1 = 0;
    for successor in matcher.G1.successors(g1_node.name.as_str()) {
        if matcher.out_1.contains_key(successor.name.as_str())
            && !matcher.core_1.contains_key(successor.name.as_str())
        {
            num1 += 1;
        }
    }
    let mut num2 = 0;
    for successor in matcher.G2.successors(g2_node.name.as_str()) {
        if matcher.out_2.contains_key(successor.name.as_str())
            && !matcher.core_2.contains_key(successor.name.as_str())
        {
            num2 += 1;
        }
    }
    if !(num1 >= num2) {
        return false;
    }

    true
}

/// R_in, R_out and R_new for pruning the search tree
/// R_in and R_out is 1-look-ahead, and R_new is 2-look-ahead
pub fn R_new<'a>(matcher: &DiGraphMatcher, g1_node: &'a Node, g2_node: &'a Node) -> bool {
    // The number of predecessors of n that are neither in the core_1 nor
    // Tin_1 nor Tout_1 is equal to the number of predecessors of m
    // that are neither in core_2 nor Tin_2 nor Tout_2.

    let mut num1 = 0;
    for predecessor in matcher.G1.predecessors(g1_node.name.as_str()) {
        if matcher.in_1.contains_key(predecessor.name.as_str())
            && !matcher.out_1.contains_key(predecessor.name.as_str())
        {
            num1 += 1;
        }
    }
    let mut num2 = 0;
    for predecessor in matcher.G2.predecessors(g2_node.name.as_str()) {
        if matcher.in_2.contains_key(predecessor.name.as_str())
            && !matcher.out_2.contains_key(predecessor.name.as_str())
        {
            num2 += 1;
        }
    }
    if !(num1 >= num2) {
        return false;
    }

    // The number of successors of n that are neither in the core_1 nor
    // Tin_1 nor Tout_1 is equal to the number of successors of m
    // that are neither in core_2 nor Tin_2 nor Tout_2.

    let mut num1 = 0;
    for successor in matcher.G1.successors(g1_node.name.as_str()) {
        if matcher.in_1.contains_key(successor.name.as_str())
            && !matcher.out_1.contains_key(successor.name.as_str())
        {
            num1 += 1;
        }
    }
    let mut num2 = 0;
    for successor in matcher.G2.successors(g2_node.name.as_str()) {
        if matcher.in_2.contains_key(successor.name.as_str())
            && !matcher.out_2.contains_key(successor.name.as_str())
        {
            num2 += 2;
        }
    }
    if !(num1 >= num2) {
        return false;
    }

    true
}

pub fn semantic_feasibility(g1_node: String, g2_node: String) -> bool {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
