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
pub mod r2_bins;
pub mod r3_deg;
pub mod r4_ins;
pub mod r5_ddeg;
pub mod r6_maj;
pub mod r7_mer;
pub mod r8_2sum;
pub mod r9_bfs;

use hashbrown::HashMap;
use std::iter::repeat;

pub struct DFS {
    pub adjacency_matrix: HashMap<usize, Vec<usize>>,
    pub num_nodes: usize,
    visited: Vec<bool>,
    pub previsit: Vec<usize>,
    pub postvisit: Vec<usize>,
    clock: usize,
    pub num_connected_components: usize,
    pub connected_components: Vec<usize>,
}

impl DFS {
    pub fn run_dfs(adjacency_matrix: HashMap<usize, Vec<usize>>, num_nodes: usize) -> Self {
        let mut dfs_struct = DFS {
            adjacency_matrix,
            num_nodes,
            visited: repeat(false).take(num_nodes).collect(),
            previsit: repeat(0).take(num_nodes).collect(),
            postvisit: repeat(0).take(num_nodes).collect(),
            clock: 0,
            num_connected_components: 0,
            connected_components: repeat(0).take(num_nodes).collect(),
        };
        dfs_struct.dfs();
        dfs_struct
    }

    fn dfs(&mut self) {
        for node in 1..=self.num_nodes {
            if !self.visited[node - 1] {
                self.explore(node);
                self.num_connected_components += 1;
            }
        }
    }

    fn previsit(&mut self, node: usize) {
        self.previsit[node - 1] = self.clock;
        self.connected_components[node - 1] = self.num_connected_components;
        self.clock += 1;
    }

    fn postvisit(&mut self, node: usize) {
        self.postvisit[node - 1] = self.clock;
        self.clock += 1;
    }

    fn explore(&mut self, start_node: usize) {
        self.visited[start_node - 1] = true;
        self.previsit(start_node);
        if let Some(edge_list) = self.adjacency_matrix.get(&start_node) {
            for next_node in edge_list.clone() {
                if !self.visited[next_node - 1] {
                    self.explore(next_node);
                }
            }
        }
        self.postvisit(start_node)
    }
}
