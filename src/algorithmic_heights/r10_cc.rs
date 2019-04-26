use crate::algorithmic_heights::r5_ddeg::make_adjacency_list;
use crate::algorithmic_heights::DFS;
use crate::utils;

/// Connected Components
///
/// Given: A simple graph with n≤10^3 vertices in the edge list format.
///
/// Return: The number of connected components in the graph.
pub fn rosalind_cc() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_cc.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let (num_nodes, _, edges) = utils::read_edge_list(&mut lines, true);
    let adjacency_matrix = make_adjacency_list(&edges, false);
    println!(
        "{}",
        DFS::run_dfs(adjacency_matrix, num_nodes).num_connected_components
    );
}
