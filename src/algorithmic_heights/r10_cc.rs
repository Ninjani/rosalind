use crate::algorithmic_heights::r5_ddeg::make_adjacency_matrix;
use crate::algorithmic_heights::DFS;
use crate::utils;

/// Connected Components
///
/// Given: A simple graph with n≤10^3 vertices in the edge list format.
///
/// Return: The number of connected components in the graph.
pub fn rosalind_cc() {
    let (num_nodes, _, edges) = utils::read_edge_list(&utils::input_from_file(
        "data/algorithmic_heights/rosalind_cc.txt",
    ));
    let adjacency_matrix = make_adjacency_matrix(&edges, false);
    println!(
        "{}",
        DFS::run_dfs(adjacency_matrix, num_nodes).num_connected_components
    );
}
