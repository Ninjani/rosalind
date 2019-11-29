use std::collections::{btree_map::BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

pub fn convert_graph<T: Hash + Clone + Eq + Debug>(
    adjacency_list: &HashMap<T, Vec<T>>,
) -> (HashMap<usize, T>, BTreeMap<usize, Vec<usize>>) {
    let mut node_to_index = HashMap::new();
    let mut new_adj_list = BTreeMap::new();
    let mut i = 0;
    for (node_1, edges) in adjacency_list {
        if !node_to_index.contains_key(node_1) {
            node_to_index.insert(node_1.clone(), i);
            i += 1;
        }
        let index_1 = node_to_index[node_1];
        for node_2 in edges {
            if !node_to_index.contains_key(node_2) {
                node_to_index.insert(node_2.clone(), i);
                i += 1;
            }
            let index_2 = node_to_index[node_2];
            new_adj_list
                .entry(index_1)
                .or_insert_with(Vec::new)
                .push(index_2);
        }
    }
    (
        node_to_index.into_iter().map(|(n, i)| (i, n)).collect(),
        new_adj_list,
    )
}

#[derive(Debug, Clone)]
pub struct WeightedGraph {
    pub edges: Vec<(usize, usize, i32)>,
    pub adjacency_list: HashMap<usize, Vec<(usize, i32)>>,
    pub nodes: Vec<usize>,
    pub node_to_index: HashMap<usize, usize>,
    pub num_nodes: usize,
    pub num_edges: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IntegerGraph {
    pub adjacency_list: BTreeMap<usize, Vec<usize>>,
    pub nodes: Vec<usize>,
    pub node_to_index: BTreeMap<usize, usize>,
    pub num_nodes: usize,
    pub ran_dfs: bool,
    pub visited: Vec<bool>,
    pub previsit: Vec<usize>,
    pub postvisit: Vec<usize>,
    clock: usize,
    pub num_connected_components: usize,
    pub connected_components: Vec<usize>,
}

impl IntegerGraph {
    pub fn from_weighted_graph(weighted_graph: &WeightedGraph, run_dfs: bool) -> Self {
        let adjacency_list = weighted_graph
            .adjacency_list
            .iter()
            .map(|(node_1, edge_list)| {
                (
                    *node_1,
                    edge_list.iter().map(|(node_2, _)| *node_2).collect(),
                )
            })
            .collect();
        Self::new(adjacency_list, weighted_graph.nodes.clone(), run_dfs)
    }

    pub fn get_reverse_graph(&self, run_dfs: bool) -> Self {
        let mut adj_list_rev = BTreeMap::new();
        for (node_2, edge_list) in &self.adjacency_list {
            for node_1 in edge_list {
                adj_list_rev
                    .entry(*node_1)
                    .or_insert_with(Vec::new)
                    .push(*node_2);
            }
        }
        Self::new(adj_list_rev, self.nodes.clone(), run_dfs)
    }

    /// Convert an adjacency list of nodes of any type to an indexed adjacency list
    /// where each node is given an index (starting from 0)
    pub fn convert_graph<T: Eq + Clone + Hash>(
        adjacency_list: &HashMap<T, Vec<T>>,
        num_nodes: usize,
    ) -> (HashMap<usize, T>, BTreeMap<usize, Vec<usize>>) {
        let mut node_to_index = HashMap::with_capacity(num_nodes);
        let mut adjacency_list_indexed = BTreeMap::new();
        let mut i = 0;
        for (node_1, edges) in adjacency_list {
            if !node_to_index.contains_key(node_1) {
                node_to_index.insert(node_1.clone(), i);
                i += 1;
            }
            let index_1 = node_to_index[node_1];
            for node_2 in edges {
                if !node_to_index.contains_key(node_2) {
                    node_to_index.insert(node_2.clone(), i);
                    i += 1;
                }
                let index_2 = node_to_index[node_2];
                adjacency_list_indexed
                    .entry(index_1)
                    .or_insert_with(Vec::new)
                    .push(index_2);
            }
        }
        (
            node_to_index.into_iter().map(|(n, i)| (i, n)).collect(),
            adjacency_list_indexed,
        )
    }

    pub fn new(
        adjacency_list: BTreeMap<usize, Vec<usize>>,
        nodes: Vec<usize>,
        run_dfs: bool,
    ) -> Self {
        let num_nodes = nodes.len();
        let node_to_index = nodes.iter().enumerate().map(|(i, n)| (*n, i)).collect();
        let mut graph = IntegerGraph {
            adjacency_list,
            nodes,
            num_nodes,
            node_to_index,
            visited: (0..num_nodes).map(|_| false).collect(),
            previsit: (0..num_nodes).map(|_| 0).collect(),
            postvisit: (0..num_nodes).map(|_| 0).collect(),
            ran_dfs: false,
            clock: 0,
            num_connected_components: 0,
            connected_components: (0..num_nodes).map(|_| 0).collect(),
        };
        if run_dfs {
            graph.run_dfs();
        }
        graph
    }

    fn run_dfs(&mut self) {
        self.ran_dfs = true;
        for node_index in 0..self.num_nodes {
            if !self.visited[node_index] {
                self.explore(node_index);
                self.num_connected_components += 1;
            }
        }
    }

    fn clear_dfs(&mut self) {
        self.visited = (0..self.num_nodes).map(|_| false).collect();
        self.previsit = (0..self.num_nodes).map(|_| 0).collect();
        self.postvisit = (0..self.num_nodes).map(|_| 0).collect();
        self.clock = 0;
        self.num_connected_components = 0;
        self.connected_components = (0..self.num_nodes).map(|_| 0).collect();
    }

    pub fn run_dfs_given_node_order(&mut self, node_order: &[usize]) {
        if self.ran_dfs {
            self.clear_dfs();
        }
        for node_index in node_order {
            if !self.visited[*node_index] {
                self.explore(*node_index);
                self.num_connected_components += 1;
            }
        }
    }

    fn previsit(&mut self, node_index: usize) {
        self.previsit[node_index] = self.clock;
        self.connected_components[node_index] = self.num_connected_components;
        self.clock += 1;
    }

    fn postvisit(&mut self, node_index: usize) {
        self.postvisit[node_index] = self.clock;
        self.clock += 1;
    }

    fn explore(&mut self, start_node_index: usize) {
        self.visited[start_node_index] = true;
        self.previsit(start_node_index);
        if let Some(edge_list) = self.adjacency_list.get(&self.nodes[start_node_index]) {
            for next_node in edge_list.clone() {
                if !self.visited[self.node_to_index[&next_node]] {
                    self.explore(self.node_to_index[&next_node]);
                }
            }
        }
        self.postvisit(start_node_index)
    }
}
