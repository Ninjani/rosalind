use crate::utils;
use failure::Error;
use crate::algorithmic_heights::{DFS, r5_ddeg::make_adjacency_matrix};

pub fn rosalind_ts() -> Result<(), Error> {
    let (num_nodes, _, edges) = utils::read_edge_list(&utils::input_from_file("data/algorithmic_heights/rosalind_ts.txt"));
    let adjacency_matrix = make_adjacency_matrix(&edges, true);
    utils::print_array(&DFS::run_dfs(adjacency_matrix, num_nodes).topological_sort());
    Ok(())
}

impl DFS {
    pub fn topological_sort(&self) -> Vec<usize> {
       let mut postvisit = self.postvisit.iter().enumerate().collect::<Vec<_>>();
        postvisit.sort_by(|a, b| b.1.cmp(&a.1));
        return postvisit.into_iter().map(|(i, _)| i+1).collect()
    }
}


