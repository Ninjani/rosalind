use crate::algorithmic_heights::{r5_ddeg::make_adjacency_matrix, DFS};
use crate::utils;
use failure::Error;

/// Given: A simple directed acyclic graph with n≤103 vertices in the edge list format.
///
/// Return: A topological sorting (i.e., a permutation of vertices) of the graph.
pub fn rosalind_ts() -> Result<(), Error> {
    let (num_nodes, _, edges) = utils::read_edge_list(&utils::input_from_file(
        "data/algorithmic_heights/rosalind_ts.txt",
    ));
    let adjacency_matrix = make_adjacency_matrix(&edges, true);
    utils::print_array(&DFS::run_dfs(adjacency_matrix, num_nodes).get_topological_sort());
    Ok(())
}

impl DFS {
    pub fn get_topological_sort(&self) -> Vec<usize> {
        let mut postvisit = self.postvisit.iter().enumerate().collect::<Vec<_>>();
        postvisit.sort_by(|a, b| b.1.cmp(&a.1));
        postvisit.into_iter().map(|(i, _)| i + 1).collect()
    }
}
