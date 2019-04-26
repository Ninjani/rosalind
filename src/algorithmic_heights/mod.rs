pub mod r10_cc;
pub mod r11_hea;
pub mod r12_ms;
pub mod r13_par;
pub mod r14_3sum;
pub mod r15_bip;
pub mod r16_dag;
pub mod r17_dij;
pub mod r18_hs;
pub mod r19_inv;
pub mod r1_fibo;
pub mod r20_par3;
pub mod r21_sq;
pub mod r22_bf;
pub mod r23_cte;
pub mod r24_med;
pub mod r25_ps;
pub mod r26_ts;
pub mod r27_hdag;
pub mod r28_nwc;
pub mod r29_qs;
pub mod r2_bins;
pub mod r30_scc;
pub mod r31_2sat;
pub mod r32_gs;
pub mod r33_sc;
pub mod r34_sdag;
pub mod r3_deg;
pub mod r4_ins;
pub mod r5_ddeg;
pub mod r6_maj;
pub mod r7_mer;
pub mod r8_2sum;
pub mod r9_bfs;
use std::hash::Hash;
use std::fmt::Debug;

use hashbrown::HashMap;
use std::collections::btree_map::BTreeMap;

pub fn convert_graph<T: Hash + Clone + Eq + Debug>(adjacency_list: &HashMap<T, Vec<T>>) -> (HashMap<usize, T>, BTreeMap<usize, Vec<usize>>) {
    let mut node_to_index = HashMap::new();
    let mut new_adj_list = BTreeMap::new();
    let mut index = 0;
    let (mut index_1, mut index_2);
    for (node_1, edges) in adjacency_list {
        match node_to_index.get(node_1) {
            Some(i) => index_1 = *i,
            None => {
                node_to_index.insert(node_1.clone(), index);
                index_1 = index;
                index += 1;
            }
        }
        for node_2 in edges {
            match node_to_index.get(node_2) {
                Some(i) => index_2 = *i,
                None => {
                    node_to_index.insert(node_2.clone(), index);
                    index_2 = index;
                    index += 1;
                }
            }
            new_adj_list.entry(index_1).or_insert_with(Vec::new).push(index_2);
        }
    }
    (node_to_index.into_iter().map(|(n, i)| (i, n)).collect(), new_adj_list)
}


pub struct DFS {
    pub adjacency_list: BTreeMap<usize, Vec<usize>>,
    pub num_nodes: usize,
    pub visited: Vec<bool>,
    pub previsit: Vec<usize>,
    pub postvisit: Vec<usize>,
    clock: usize,
    pub num_connected_components: usize,
    pub connected_components: Vec<usize>,
}

impl DFS {
    fn new(adjacency_list: BTreeMap<usize, Vec<usize>>, num_nodes: usize) -> Self {
        DFS {
            adjacency_list,
            num_nodes,
            visited: (0..num_nodes).map(|_| false).collect(),
            previsit: (0..num_nodes).map(|_| 0).collect(),
            postvisit: (0..num_nodes).map(|_| 0).collect(),
            clock: 0,
            num_connected_components: 0,
            connected_components: (0..num_nodes).map(|_| 0).collect(),
        }
    }

    pub fn run_dfs(adjacency_list: BTreeMap<usize, Vec<usize>>, num_nodes: usize) -> Self {
        let mut dfs_struct = DFS::new(adjacency_list, num_nodes);
        dfs_struct.dfs();
        dfs_struct
    }

    fn dfs(&mut self) {
        for node in 0..self.num_nodes {
            if !self.visited[node] {
                self.explore(node);
                self.num_connected_components += 1;
            }
        }
    }

    fn previsit(&mut self, node: usize) {
        self.previsit[node] = self.clock;
        self.connected_components[node] = self.num_connected_components;
        self.clock += 1;
    }

    fn postvisit(&mut self, node: usize) {
        self.postvisit[node] = self.clock;
        self.clock += 1;
    }

    fn explore(&mut self, start_node: usize) {
        self.visited[start_node] = true;
        self.previsit(start_node);
        if let Some(edge_list) = self.adjacency_list.get(&start_node) {
            for next_node in edge_list.clone() {
                if !self.visited[next_node] {
                    self.explore(next_node);
                }
            }
        }
        self.postvisit(start_node)
    }
}
